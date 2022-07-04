use crate::error::RealmVoterError;
use crate::state::*;
use anchor_lang::prelude::*;
use spl_governance::state::token_owner_record;

/// Updates VoterWeightRecord to evaluate governance power for users and the Realm DAO they belong to
/// Realm DAO membership is evaluated via a valid TokenOwnerRecord which must belong to one of the configured spl-governance instances
/// This instruction updates VoterWeightRecord which is valid for the current Slot only
/// The instruction must be executed inside the same transaction as the corresponding spl-gov instruction
#[derive(Accounts)]
pub struct UpdateVoterWeightRecord<'info> {
    /// The RealmVoter voting Registrar
    pub registrar: Account<'info, Registrar>,

    #[account(
        mut,
        constraint = voter_weight_record.realm == registrar.realm
        @ RealmVoterError::InvalidVoterWeightRecordRealm,

        constraint = voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ RealmVoterError::InvalidVoterWeightRecordMint,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,

    /// TokenOwnerRecord for any of the configured spl-governance instances
    /// CHECK: Owned by spl-governance instance specified in registrar.governance_program_id
    pub token_owner_record: UncheckedAccount<'info>,
}

pub fn update_voter_weight_record(ctx: Context<UpdateVoterWeightRecord>) -> Result<()> {
    let registrar = &ctx.accounts.registrar;

    let governance_program_id = ctx.accounts.token_owner_record.owner;

    let voter_weight = if registrar
        .governance_program_configs
        .iter()
        .any(|cc| cc.program_id == governance_program_id.key())
    {
        registrar.realm_member_voter_weight
    } else {
        0u64
    };

    // Deserialize TokenOwnerRecord to ensure it's a valid account
    let _ = token_owner_record::get_token_owner_record_data(
        &governance_program_id,
        &ctx.accounts.token_owner_record,
    )?;

    // Set voter_weight
    let voter_weight_record = &mut ctx.accounts.voter_weight_record;
    voter_weight_record.voter_weight = voter_weight;

    // Record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);

    // Set action and target to None to indicate the weight is valid for any action and target
    voter_weight_record.weight_action = None;
    voter_weight_record.weight_action_target = None;

    Ok(())
}
