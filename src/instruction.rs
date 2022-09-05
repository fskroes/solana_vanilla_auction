use solana_program::program_error::ProgramError;
use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {

    /// Start the trade by creating and populating an escrow account
    /// and transferring ownership of the given temp token 
    /// account to the PDA
    ///
    /// Accounts expected
    /// 
    /// 0. '[signer]' the account of the person initializing the escrow
    /// 1. '[writable]' the auction account holding state of aucion
    /// 2. '[writable]' The escrow account
    /// 3. '[]' The rent sysvar
    InitEscrow {
        /// the amount party A expects to receive of token Y
        amount: u64
    }
}

impl EscrowInstruction {
    /// Unpack a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html)
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}