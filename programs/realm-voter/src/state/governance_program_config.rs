use anchor_lang::prelude::*;

/// Configuration of an spl-governance instance used to grant governance power
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq, Default)]
pub struct GovernanceProgramConfig {
    /// The program id of the spl-governance instance
    pub program_id: Pubkey,

    /// Governance power weight of the spl-governance instance
    /// Each DAO member from the DAO using the instance has governance power = 1 * weight
    /// Note: The weight is scaled accordingly to the governing_token_mint decimals
    /// Ex: if the the mint has 2 decimal places then weight of 1 should be stored as 100
    pub weight: u64,

    /// Reserved for future upgrades
    pub reserved: [u8; 8],
}
