use arrayref::{array_ref, array_refs, array_mut_ref, mut_array_refs};
use serde::{Serialize, Deserialize};
use solana_program::{
    pubkey::Pubkey, 
    program_pack::{Sealed, Pack}, 
    program_error::ProgramError
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Initialize {
    pub exhibitor: Pubkey,
    pub auction_account_state: Pubkey,
    pub treasury: Pubkey,
}

impl Sealed for Initialize {}

impl Pack for Initialize {
    const LEN: usize = std::mem::size_of::<Initialize>();
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Initialize::LEN];
        let (
            exhibitor,
            auction_account_state,
            treasury,
        ) = array_refs![src, 32, 32, 32];

        Ok(Initialize {
            exhibitor: Pubkey::new_from_array(*exhibitor),
            auction_account_state: Pubkey::new_from_array(*auction_account_state),
            treasury: Pubkey::new_from_array(*treasury),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Initialize::LEN];
        let (
            exhibitor_dst,
            auction_account_state_dst,
            treasury_dst,
        ) = mut_array_refs![dst, 32, 32, 32];

        let Initialize {
            exhibitor,
            auction_account_state,
            treasury,
        } = self;

        exhibitor_dst.copy_from_slice(exhibitor.as_ref());
        auction_account_state_dst.copy_from_slice(auction_account_state.as_ref());
        treasury_dst.copy_from_slice(treasury.as_ref());
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Auction {
    pub exhibitor_pubkey: Pubkey,
    pub highest_bidder_pubkey: Pubkey,

    pub price: u64,
    pub end_at: i64,

    pub treasury: Pubkey,
}

impl Auction {
    pub const SERIALIZED_SIZE: usize = std::mem::size_of::<Auction>();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BidInfo {
    pub amount_locked: u64,
    pub bump: u8,
}