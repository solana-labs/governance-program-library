use anchor_lang::prelude::*;

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