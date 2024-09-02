use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
    clock::Clock,
};

use crate::{
    error::VaultError,
    instruction::VaultInstruction,
    state::VaultState,
    validation,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = VaultInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        VaultInstruction::Initialize => initialize(program_id, accounts),
        VaultInstruction::Deposit { amount } => deposit(accounts, amount),
        VaultInstruction::Withdraw { amount } => withdraw(accounts, amount),
    }
}

fn initialize(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let owner = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    validation::validate_is_signer(owner)?;
    validation::validate_is_writable(vault_account)?;
    validation::validate_rent_exempt(vault_account, &Rent::get()?)?;

    let clock = Clock::get()?;
    let vault_state = VaultState::new(*owner.key, clock.unix_timestamp);
    let space = VaultState::LEN;
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(space);

    invoke(
        &system_instruction::create_account(
            owner.key,
            vault_account.key,
            lamports,
            space as u64,
            program_id,
        ),
        &[owner.clone(), vault_account.clone(), system_program.clone()],
    )?;

    vault_state.serialize(&mut *vault_account.data.borrow_mut())?;

    msg!("Vault initialized");
    Ok(())
}

fn deposit(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let owner = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    validation::validate_is_signer(owner)?;
    validation::validate_is_writable(vault_account)?;

    let mut vault_state = VaultState::try_from_slice(&vault_account.data.borrow())?;
    validation::validate_owner(&vault_state, owner.key)?;

    invoke(
        &system_instruction::transfer(owner.key, vault_account.key, amount),
        &[owner.clone(), vault_account.clone(), system_program.clone()],
    )?;

    vault_state.deposit(amount)?;
    vault_state.serialize(&mut *vault_account.data.borrow_mut())?;

    msg!("Deposited {} lamports", amount);
    Ok(())
}

fn withdraw(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let vault_account = next_account_info(account_info_iter)?;
    let owner = next_account_info(account_info_iter)?;
    let _system_program = next_account_info(account_info_iter)?;

    validation::validate_is_signer(owner)?;
    validation::validate_is_writable(vault_account)?;

    let mut vault_state = VaultState::try_from_slice(&vault_account.data.borrow())?;
    validation::validate_owner(&vault_state, owner.key)?;

    let clock = Clock::get()?;
    let cooldown_period = 24 * 60 * 60; // 24 hours in seconds

    if !vault_state.can_withdraw(clock.unix_timestamp, cooldown_period) {
        return Err(VaultError::WithdrawalCooldown.into());
    }

    let withdrawal_amount = if amount == 0 {
        vault_state.total_deposits / 10
    } else {
        amount
    };

    if withdrawal_amount == 0 {
        return Err(VaultError::InsufficientFunds.into());
    }

    **vault_account.try_borrow_mut_lamports()? -= withdrawal_amount;
    **owner.try_borrow_mut_lamports()? += withdrawal_amount;

    vault_state.withdraw(withdrawal_amount, clock.unix_timestamp)?;
    vault_state.serialize(&mut *vault_account.data.borrow_mut())?;

    msg!("Withdrawn {} lamports", withdrawal_amount);
    Ok(())
}