use solana_program::{
    entrypoint::ProgramResult, 
    pubkey::Pubkey, 
    account_info::{AccountInfo, next_account_info}, 
    msg, 
    program_error::ProgramError, rent::Rent, sysvar::Sysvar, program_pack::Pack
};

use crate::{
    instruction::EscrowInstruction, 
    error::EscrowError, 
    state::Initialize
};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey, 
        accounts: &[AccountInfo], 
        instruction_data: &[u8]
    ) -> ProgramResult {
        let instruction = EscrowInstruction::unpack(instruction_data)?;

        match instruction {
            EscrowInstruction::InitEscrow { amount } => {
                msg!("Instruction: InitEscrow");
                Self::process_init_escrow(accounts, amount, program_id)
            }
        }
    }

    fn process_init_escrow(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let exhibitor = next_account_info(account_info_iter)?;

        if !exhibitor.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let auction_state = next_account_info(account_info_iter)?;
        let treasury = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(treasury.lamports(), treasury.data_len()) {
            return Err(EscrowError::NotRentExempt.into());
        }

        // let mut auction_state_info = Initialize::unpack_unchecked(&auction_state.data.borrow())?;

        // Initialize::pack(src, dst)

        let (pda, _bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);
        

        Ok(())
    }
}

