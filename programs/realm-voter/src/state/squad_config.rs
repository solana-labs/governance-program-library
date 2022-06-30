use anchor_lang::prelude::*;

/// Configuration of a Squad used for governance power
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq, Default)]
pub struct SquadConfig {
    /// The Squad used for governance
    pub squad: Pubkey,

    /// Governance power weight of the Squad
    /// Membership in the Squad gives governance power equal to the weight
    ///
    /// Note: The weight is scaled accordingly to the governing_token_mint decimals
    /// Ex: if the the mint has 2 decimal places then weight of 1 should be stored as 100
    pub weight: u64,

    /// Reserved for future upgrades
    pub reserved: [u8; 8],
}
