use thiserror::Error;

use crate::{error::EscrowError, instruction::EscrowInstruction};

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
  /// Invalid instruction
  #[error("Invalid Instruction")]
  InvalidInstruction,
}

impl From<EscrowError> for ProgramError {
  fn from(e: EscrowError) -> Self {
    ProgramError::Custom(e as u32)
  }
}
