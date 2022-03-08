use anchor_lang::prelude::*;

/// Configuration of an NFT collection used for governance power
#[account]
#[derive(Default)]
pub struct CollectionConfig {
    /// The NFT collection used for governance
    pub collection: Pubkey,

    /// Governance power weight of the collection
    /// Each NFT in the collection has governance power = 1 * weight
    pub weight: u16,

    /// Reserved for future upgrades
    pub reserved: [u8; 8],
}
