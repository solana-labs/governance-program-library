// Add the generic max voter weight trait to SPL-Token mint structs
use crate::generic_max_voter_weight::GenericMaxVoterWeight;
use anchor_lang::prelude::Pubkey;
use spl_token::state::Mint;

pub struct MintMaxVoterWeight {
    pub mint: Mint,
    pub key: Pubkey,
}

impl GenericMaxVoterWeight for MintMaxVoterWeight {
    fn get_governing_token_mint(&self) -> Pubkey {
        self.key
    }

    /// By default, the max voter weight is equal to the total supply of governance tokens
    fn get_max_voter_weight(&self) -> u64 {
        self.mint.supply
    }

    // when using a governing token - the max voter weight has no expiry
    fn get_max_voter_weight_expiry(&self) -> Option<u64> {
        None
    }
}
