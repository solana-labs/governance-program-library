use crate::error::NftVoterError;
use crate::state::*;
use anchor_lang::prelude::*;
use max_voter_weight_record::MaxVoterWeightRecord;

// Takes all collections added to `register`, iterates over them and calculates
// the max voter weight
#[derive(Accounts)]
pub struct UpdateMaxVoterWeightRecord<'info> {
    /// The NFT voting Registrar
    pub registrar: Account<'info, Registrar>,

    #[account(
        mut,
        constraint = max_voter_weight_record.realm == registrar.realm
        @ NftVoterError::InvalidVoterWeightRecordRealm,

        constraint = max_voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ NftVoterError::InvalidVoterWeightRecordMint,
    )]
    pub max_voter_weight_record: Account<'info, MaxVoterWeightRecord>,
}

pub fn update_max_voter_weight_record(ctx: Context<UpdateMaxVoterWeightRecord>) -> Result<()> {
    let registrar = &ctx.accounts.registrar;

    // Calculate the max voter weight by iterating over all collections and summing
    // the max weight of each collection.
    ctx.accounts.max_voter_weight_record.max_voter_weight = registrar
        .collection_configs
        .iter()
        .try_fold(0u64, |sum, cc| sum.checked_add(cc.get_max_weight()))
        .unwrap();

    // Record is only valid as of the current slot
    let clock = Clock::get()?.slot;
    msg!("Clock: {:?}", clock);

    ctx.accounts.max_voter_weight_record.max_voter_weight_expiry = Some(Clock::get()?.slot);

    Ok(())
}
