use anchor_lang::prelude::*;

/// Configuration of an spl-governance instance used to grant governance power
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq, Default)]
pub struct GovernanceProgramConfig {
    /// The program id of the spl-governance instance
    pub program_id: Pubkey,

    /// Reserved for future upgrades
    pub reserved: [u8; 8],
}
