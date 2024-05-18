use crate::error::*;
use anchor_lang::prelude::*;

/// Exchange rate for an asset that can be used to mint voting rights.
///
/// See documentation of configure_voting_mint for details on how
/// native token amounts convert to vote weight.
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq)]
pub struct VotingMintConfig {
    /// Mint for this entry.
    pub mint: Pubkey,
    
    /// Number of digits to shift native amounts, applying a 10^digit_shift factor.
    pub digit_shift: i8,

    // Empty bytes for future upgrades.
    pub reserved1: [u8; 63],
}

const_assert!(std::mem::size_of::<VotingMintConfig>() == 32 + 1 + 63);
const_assert!(std::mem::size_of::<VotingMintConfig>() % 8 == 0);

impl VotingMintConfig {
    /// Converts an amount in this voting mints's native currency
    /// to the base vote weight
    /// by applying the digit_shift factor.
    pub fn digit_shift_native(&self, amount_native: u64) -> Result<u64> {
        let compute = || -> Option<u64> {
            let val = if self.digit_shift < 0 {
                (amount_native as u128).checked_div(10u128.pow((-self.digit_shift) as u32))?
            } else {
                (amount_native as u128).checked_mul(10u128.pow(self.digit_shift as u32))?
            };
            u64::try_from(val).ok()
        };
        compute().ok_or_else(|| error!(TokenVoterError::VoterWeightOverflow))
    }

    /// Whether this voting mint is configured.
    pub fn in_use(&self) -> bool {
        self.mint != Pubkey::default()
    }
}
