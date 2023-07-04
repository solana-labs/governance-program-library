use crate::id;
use anchor_lang::prelude::*;
use solana_program::pubkey::PUBKEY_BYTES;

/// Configuration of an NFT collection used for governance power
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq, Default)]
pub struct CollectionConfig {
    /// The NFT collection used for governance
    pub collection: Pubkey,

    /// The size of the NFT collection used to calculate max voter weight
    /// Note: At the moment the size is not captured on Metaplex accounts
    /// and it has to be manually updated on the Registrar
    pub size: u32,

    /// Governance power weight of the collection
    /// Each NFT in the collection has governance power = 1 * weight
    /// Note: The weight is scaled accordingly to the governing_token_mint decimals
    /// Ex: if the the mint has 2 decimal places then weight of 1 should be stored as 100
    pub weight: u64,

    /// Reserved for future upgrades
    pub reserved: [u8; 8],
}

/// Registrar which stores NFT voting configuration for the given Realm
#[account]
#[derive(Debug, PartialEq)]
pub struct Registrar {
    /// spl-governance program the Realm belongs to
    pub governance_program_id: Pubkey,

    /// Realm of the Registrar
    pub realm: Pubkey,

    /// Governing token mint the Registrar is for
    /// It can either be the Community or the Council mint of the Realm
    /// When the plugin is used the mint is only used as identity of the governing power (voting population)
    /// and the actual token of the mint is not used
    pub governing_token_mint: Pubkey,

    /// MPL Collection used for voting
    pub collection_configs: Vec<CollectionConfig>,

    /// Reserved for future upgrades
    pub reserved: [u8; 128],
}

pub const DISCRIMINATOR_SIZE: usize = 8;

impl Registrar {
    pub fn get_space(max_collections: u8) -> usize {
        DISCRIMINATOR_SIZE
            + PUBKEY_BYTES * 3
            + 4
            + max_collections as usize * (PUBKEY_BYTES + 4 + 8 + 8)
            + 128
    }
}

/// Returns Registrar PDA seeds
pub fn get_registrar_seeds<'a>() -> [&'a [u8]; 1] {
    [b"registrar"]
}

/// Returns Registrar PDA address
pub fn get_registrar_address() -> Pubkey {
    Pubkey::find_program_address(&get_registrar_seeds(), &id()).0
}
