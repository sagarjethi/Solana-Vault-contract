use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use solana_program::program_error::ProgramError;
use crate::error::VaultError;

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
pub struct VaultState {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub total_deposits: u64,
    pub last_withdrawal_time: i64,
}

impl VaultState {
    pub const LEN: usize = 1 + 32 + 8 + 8;

    pub fn new(owner: Pubkey, current_time: i64) -> Self {
        Self {
            is_initialized: true,
            owner,
            total_deposits: 0,
            last_withdrawal_time: current_time,
        }
    }

    pub fn deposit(&mut self, amount: u64) -> Result<(), ProgramError> {
        self.total_deposits = self.total_deposits.checked_add(amount)
            .ok_or(VaultError::Overflow)?;
        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64, current_time: i64) -> Result<(), ProgramError> {
        if amount > self.total_deposits {
            return Err(VaultError::InsufficientFunds.into());
        }
        self.total_deposits = self.total_deposits.checked_sub(amount)
            .ok_or(VaultError::Overflow)?;
        self.last_withdrawal_time = current_time;
        Ok(())
    }

    pub fn can_withdraw(&self, current_time: i64, cooldown_period: i64) -> bool {
        current_time - self.last_withdrawal_time >= cooldown_period
    }
}