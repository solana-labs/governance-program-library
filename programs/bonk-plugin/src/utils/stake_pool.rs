use anchor_lang::prelude::*;

#[repr(C)]
#[derive(AnchorDeserialize, Debug)]
pub struct StakePool {
    pub discriminator: u64,
    /** Pubkey that can make updates to StakePool */
    pub authority: Pubkey,
    /** Total amount staked that accounts for the lock up period weighting.
    Note, this is not equal to the amount of SPL Tokens staked. */
    pub total_weighted_stake: u128,
    /** Token Account to store the staked SPL Token */
    pub vault: Pubkey,
    /** Mint of the token being staked */
    pub mint: Pubkey,
    /** Mint of the token representing effective stake */
    pub stake_mint: Pubkey,
    /// Array of RewardPools that apply to the stake pool.
    /// Unused entries are Pubkey default. In arbitrary order, and may have gaps.
    pub reward_pools: [RewardPool; 10],
    /// The minimum weight received for staking. In terms of 1 / SCALE_FACTOR_BASE.
    /// Examples:
    /// * `min_weight = 1 x SCALE_FACTOR_BASE` = minmum of 1x multiplier for > min_duration staking
    /// * `min_weight = 2 x SCALE_FACTOR_BASE` = minmum of 2x multiplier for > min_duration staking
    pub base_weight: u64,
    /// Maximum weight for staking lockup (i.e. weight multiplier when locked
    /// up for max duration). In terms of 1 / SCALE_FACTOR_BASE. Examples:
    /// * A `max_weight = 1 x SCALE_FACTOR_BASE` = 1x multiplier for max staking duration
    /// * A `max_weight = 2 x SCALE_FACTOR_BASE` = 2x multiplier for max staking duration
    pub max_weight: u64,
    /** Minimum duration for lockup. At this point, the staker would receive the base weight. In seconds. */
    pub min_duration: u64,
    /** Maximum duration for lockup. At this point, the staker would receive the max weight. In seconds. */
    pub max_duration: u64,
    /** Nonce to derive multiple stake pools from same mint */
    pub nonce: u8,
    /** Bump seed for stake_mint */
    pub bump_seed: u8,
    // padding to next 8-byte
    _padding0: [u8; 6],
    _reserved0: [u8; 8],
}

#[derive(Clone, Copy, Default, AnchorDeserialize, AnchorSerialize, Debug)]
#[repr(C)]
pub struct RewardPool {
    /** Token Account to store the reward SPL Token */
    pub reward_vault: Pubkey,
    /** Ever increasing accumulator of the amount of rewards per effective stake.
    Said another way, if a user deposited before any rewards were added to the
    `vault`, then this would be the token amount per effective stake they could
    claim. */
    pub rewards_per_effective_stake: u128,
    /** latest amount of tokens in the vault */
    pub last_amount: u64,
    _padding0: [u8; 8],
}

impl StakePool {
    pub const ACCOUNT_DISCRIMINATOR: [u8; 8] = [121, 34, 206, 21, 79, 127, 255, 28];

    pub fn deserialize_checked(stake_pool_account_info: &AccountInfo) -> Result<Self> {
        let stake_pool_data = &stake_pool_account_info.try_borrow_data()?;
        let data = &mut stake_pool_data.as_ref();

        let stake_pool = Self::try_from_slice(data)?;

        if stake_pool.discriminator.to_le_bytes() != Self::ACCOUNT_DISCRIMINATOR {
            return Err(anchor_lang::error!(
                anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
            )
            .with_account_name("StakePool"));
        }

        Ok(stake_pool)
    }
}
