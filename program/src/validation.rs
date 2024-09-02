use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
};
use crate::error::VaultError;
use crate::state::VaultState;

pub fn validate_is_signer(account_info: &AccountInfo) -> ProgramResult {
    if !account_info.is_signer {
        Err(ProgramError::MissingRequiredSignature)
    } else {
        Ok(())
    }
}

pub fn validate_is_writable(account_info: &AccountInfo) -> ProgramResult {
    if !account_info.is_writable {
        Err(ProgramError::InvalidAccountData)
    } else {
        Ok(())
    }
}

pub fn validate_rent_exempt(
    account_info: &AccountInfo,
    rent: &Rent,
) -> ProgramResult {
    if !rent.is_exempt(account_info.lamports(), account_info.data_len()) {
        Err(ProgramError::AccountNotRentExempt)
    } else {
        Ok(())
    }
}

pub fn validate_owner(vault_state: &VaultState, expected_owner: &Pubkey) -> ProgramResult {
    if vault_state.owner != *expected_owner {
        Err(VaultError::InvalidOwner.into())
    } else {
        Ok(())
    }
}