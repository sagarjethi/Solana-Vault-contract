use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, FromPrimitive)]
pub enum VaultError {
    #[error("Account not initialized")]
    UninitializedAccount,
    #[error("Insufficient funds for withdrawal")]
    InsufficientFunds,
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Account already initialized")]
    AlreadyInitialized,
    #[error("Invalid owner")]
    InvalidOwner,
    #[error("Arithmetic overflow")]
    Overflow,
    #[error("Withdrawal cooldown period not elapsed")]
    WithdrawalCooldown,
}

impl From<VaultError> for ProgramError {
    fn from(e: VaultError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for VaultError {
    fn type_of() -> &'static str {
        "VaultError"
    }
}