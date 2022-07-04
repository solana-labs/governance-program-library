use crate::error::SquadsVoterError;
use crate::state::*;
use anchor_lang::prelude::*;

/// Updates VoterWeightRecord to evaluate governance power for users and the Squads they belong to
/// This instruction updates VoterWeightRecord which is valid for the current Slot only
/// The instruction must be executed inside the same transaction as the corresponding spl-gov instruction
#[derive(Accounts)]
pub struct UpdateVoterWeightRecord<'info> {
    /// The Squads voting Registrar
    pub registrar: Account<'info, Registrar>,

    #[account(
        mut,
        constraint = voter_weight_record.realm == registrar.realm
        @ SquadsVoterError::InvalidVoterWeightRecordRealm,

        constraint = voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ SquadsVoterError::InvalidVoterWeightRecordMint,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,
    //
    // Remaining Accounts: Squads Membership
}

pub fn update_voter_weight_record(ctx: Context<UpdateVoterWeightRecord>) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    let _governing_token_owner = &ctx.accounts.voter_weight_record.governing_token_owner;

    let mut voter_weight = 0u64;

    let mut unique_squads = vec![];

    for squad_info in ctx.remaining_accounts.iter() {
        // Ensure the same Squad was not provided more than once
        if unique_squads.contains(&squad_info.key) {
            return Err(SquadsVoterError::DuplicatedSquadDetected.into());
        }
        unique_squads.push(squad_info.key);

        // TODO: Assert squad_info is owned by squads-protocol program
        // TODO: Validate Squad membership for governing_token_owner and squad_info

        let squad_config = registrar.get_squad_config(squad_info.key)?;
    }

    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    voter_weight_record.voter_weight = voter_weight;

    // Record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);

    // Set action and target to None to indicate the weight is valid for any action
    voter_weight_record.weight_action = None;
    voter_weight_record.weight_action_target = None;

    Ok(())
}
