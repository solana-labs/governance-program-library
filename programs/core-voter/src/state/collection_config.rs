use anchor_lang::prelude::*;

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

impl CollectionConfig {
    pub fn get_max_weight(&self) -> u64 {
        (self.size as u64).checked_mul(self.weight).unwrap()
    }
}
