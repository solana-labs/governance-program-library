use crate::error::QuadraticError;
use crate::state::*;
use anchor_lang::prelude::*;
use gpl_shared::compose::{resolve_input_max_voter_weight, MaxVoterWeightRecordBase};
use gpl_shared::generic_max_voter_weight::GenericMaxVoterWeight;
use std::cmp::max;

impl<'a> MaxVoterWeightRecordBase<'a> for MaxVoterWeightRecord {
    fn get_governing_token_mint(&'a self) -> &'a Pubkey {
        &self.governing_token_mint
    }
}

/// Updates MaxVoterWeightRecord to calculate the max voting weight power
/// This instruction updates MaxVoterWeightRecord which is valid for the current Slot and the given target action only
/// and hence the instruction has to be executed inside the same transaction as the corresponding spl-gov instruction
#[derive(Accounts)]
#[instruction()]
pub struct UpdateMaxVoterWeightRecord<'info> {
    /// The quadratic plugin Registrar
    pub registrar: Account<'info, Registrar>,

    /// An account that is either a governance token mint or a MaxVoterWeightRecord
    /// from a predecessor plugin.
    /// depending on whether the registrar includes a predecessor or not
    /// CHECK: Checked in the code depending on the registrar
    #[account()]
    pub input_max_voter_weight: UncheckedAccount<'info>,

    #[account(
    mut,
    constraint = max_voter_weight_record.realm == registrar.realm
    @ QuadraticError::InvalidVoterWeightRecordRealm,

    constraint = max_voter_weight_record.governing_token_mint == registrar.governing_token_mint
    @ QuadraticError::InvalidVoterWeightRecordMint,
    )]
    pub max_voter_weight_record: Account<'info, MaxVoterWeightRecord>,
}

/// Adapts the weight of from the predecessor
pub fn update_max_voter_weight_record(ctx: Context<UpdateMaxVoterWeightRecord>) -> Result<()> {
    let max_voter_weight_record = &mut ctx.accounts.max_voter_weight_record;

    let input_max_voter_weight_account = ctx.accounts.input_max_voter_weight.to_account_info();

    let clone_record = max_voter_weight_record.clone();
    let input_max_voter_weight_record = resolve_input_max_voter_weight(
        &input_max_voter_weight_account,
        &clone_record,
        &ctx.accounts.registrar,
    )?;

    let output_max_voter_weight =
        (input_max_voter_weight_record.get_max_voter_weight() as f64).sqrt() as u64;
    msg!(
        "input weight: {}. output weight {}. coefficients: {:?}",
        input_max_voter_weight_record.get_max_voter_weight(),
        output_max_voter_weight,
        ctx.accounts.registrar.quadratic_coefficients
    );
    max_voter_weight_record.max_voter_weight = output_max_voter_weight;

    // If the input voter weight record has an expiry, use the max between that and the current slot
    // Otherwise use the current slot
    let current_slot = Clock::get()?.slot;
    max_voter_weight_record.max_voter_weight_expiry = input_max_voter_weight_record
        .get_max_voter_weight_expiry()
        .map_or(
            Some(current_slot), // no previous expiry, use current slot
            |previous_expiry| Some(max(previous_expiry, current_slot)),
        );

    Ok(())
}
