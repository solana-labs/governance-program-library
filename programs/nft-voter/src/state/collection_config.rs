use anchor_lang::prelude::*;

/// Configuration of an NFT collection used for governance power
#[account]
#[derive(Default, Debug, PartialEq)]
pub struct CollectionConfig {
    /// The NFT collection used for governance
    pub collection: Pubkey,

    /// The size of the NFT collection used to calculate max voter weight
    /// Note: At the moment the size is not captured on Metaplex accounts
    /// and it has to be manually updated on the Registrar
    pub size: u32,

    /// Governance power weight of the collection
    /// Each NFT in the collection has governance power = 1 * weight
    pub weight: u16,

    /// Reserved for future upgrades
    pub reserved: [u8; 8],
}
