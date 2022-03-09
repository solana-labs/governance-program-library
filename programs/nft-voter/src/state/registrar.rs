use crate::state::CollectionConfig;
use anchor_lang::prelude::*;

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
    /// TODO: should be expanded to list of collections
    pub collection_configs: Vec<CollectionConfig>,

    /// Reserved for future upgrades
    pub reserved: [u8; 64],
}
