// use crate::error::*;
use anchor_lang::prelude::*;

use super::Collection;

/// Registrar which is responsible for voting
#[account]
#[derive(Default)]
pub struct Registrar {
    pub governance_program_id: Pubkey,
    pub realm: Pubkey,
    pub realm_governing_token_mint: Pubkey,
    pub reserved1: [u8; 32],

    /// MPL Collection used for voting
    /// TODO: should be expanded to list of collections
    pub collection: Collection,

    pub bump: u8,
    // pub reserved2: [u8; 7],
    // pub reserved3: [u64; 11], // split because `Default` does not support [u8; 95]
}
// const_assert!(std::mem::size_of::<Registrar>() == 5 * 32 + 4 * 152 + 8 + 1 + 95);
// const_assert!(std::mem::size_of::<Registrar>() % 8 == 0);



#[macro_export]
macro_rules! registrar_seeds {
    ( $registrar:expr ) => {
        &[
            $registrar.realm.as_ref(),
            b"nft-registrar".as_ref(),
            $registrar.realm_governing_token_mint.as_ref(),
            &[$registrar.bump],
        ]
    };
}

pub use registrar_seeds;
