use anchor_lang::prelude::*;

use crate::SPL_TOKEN_STAKING_PROGRAM_ID;

#[repr(C)]
#[derive(AnchorDeserialize, Debug)]
pub struct StakeDepositReceipt {
    pub discriminator: u64,
    /** Pubkey that owns the staked assets */
    pub owner: Pubkey,
    /** Pubkey that paid for the deposit */
    pub payer: Pubkey,
    /** StakePool the deposit is for */
    pub stake_pool: Pubkey,
    /** Duration of the lockup period in seconds */
    pub lockup_duration: u64,
    /** Timestamp in seconds of when the stake lockup began */
    pub deposit_timestamp: i64,
    /** Amount of SPL token deposited */
    pub deposit_amount: u64,
    /** Amount of stake weighted by lockup duration. */
    pub effective_stake: u128,
    /// The amount per reward that has been claimed or perceived to be claimed. Indexes align with
    /// the StakedPool reward_pools property.
    pub claimed_amounts: [u128; 10],
}

impl StakeDepositReceipt {
    pub const ACCOUNT_DISCRIMINATOR: [u8; 8] = [210, 98, 254, 196, 151, 68, 235, 0];

    pub fn deserialize_checked(stake_deposit_receipt_account_info: &AccountInfo) -> Result<Self> {
        if stake_deposit_receipt_account_info.owner != &SPL_TOKEN_STAKING_PROGRAM_ID {
            return Err(anchor_lang::error!(
                anchor_lang::error::ErrorCode::AccountOwnedByWrongProgram
            )
            .with_account_name("StakeDepositReceipt"));
        }

        let stake_deposit_receipt_data = &stake_deposit_receipt_account_info.try_borrow_data()?;
        let data = &mut stake_deposit_receipt_data.as_ref();

        let stake_deposit_receipt = Self::try_from_slice(data)?;

        if stake_deposit_receipt.discriminator.to_le_bytes() != Self::ACCOUNT_DISCRIMINATOR {
            return Err(anchor_lang::error!(
                anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
            )
            .with_account_name("StakeDepositReceipt"));
        }

        Ok(stake_deposit_receipt)
    }
}
