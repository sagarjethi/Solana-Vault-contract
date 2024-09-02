use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub enum VaultInstruction {
    Initialize,
    Deposit { amount: u64 },
    Withdraw { amount: u64 },
}

pub fn initialize(program_id: &Pubkey, vault: &Pubkey, owner: &Pubkey) -> Instruction {
    Instruction::new_with_borsh(
        *program_id,
        &VaultInstruction::Initialize,
        vec![
            AccountMeta::new(*vault, false),
            AccountMeta::new_readonly(*owner, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    )
}

pub fn deposit(program_id: &Pubkey, vault: &Pubkey, owner: &Pubkey, amount: u64) -> Instruction {
    Instruction::new_with_borsh(
        *program_id,
        &VaultInstruction::Deposit { amount },
        vec![
            AccountMeta::new(*vault, false),
            AccountMeta::new(*owner, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    )
}

pub fn withdraw(program_id: &Pubkey, vault: &Pubkey, owner: &Pubkey, amount: u64) -> Instruction {
    Instruction::new_with_borsh(
        *program_id,
        &VaultInstruction::Withdraw { amount },
        vec![
            AccountMeta::new(*vault, false),
            AccountMeta::new(*owner, true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    )
}