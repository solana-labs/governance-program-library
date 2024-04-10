use crate::error::RealmVoterError;
use crate::state::*;
use anchor_lang::prelude::*;
use spl_governance::state::token_owner_record;

/// Updates VoterWeightRecord based on Realm DAO membership
/// The membership is evaluated via a valid TokenOwnerRecord which must belong to one of the configured spl-governance instances
///
/// This instruction sets VoterWeightRecord.voter_weight which is valid for the current slot only
/// and must be executed inside the same transaction as the corresponding spl-gov instruction
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
    /// CHECK: Owned by any of the spl-governance instances specified in registrar.governance_program_configs
    pub token_owner_record: UncheckedAccount<'info>,
}

pub fn update_voter_weight_record(ctx: Context<UpdateVoterWeightRecord>) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    let governance_program_id = ctx.accounts.token_owner_record.owner;

    // Note: We only verify a valid TokenOwnerRecord account exists for one of the configured spl-governance instances
    // The existence of the account proofs the governing_token_owner has interacted with spl-governance Realm at least once in the past
    if !registrar
        .governance_program_configs
        .iter()
        .any(|cc| cc.program_id == governance_program_id.key())
    {
        return err!(RealmVoterError::GovernanceProgramNotConfigured);
    };

    let token_owner_record = token_owner_record::get_token_owner_record_data(
        governance_program_id,
        &ctx.accounts.token_owner_record,
    )?;

    // Ensure VoterWeightRecord and TokenOwnerRecord are for the same governing_token_owner
    require_eq!(
        token_owner_record.governing_token_owner,
        voter_weight_record.governing_token_owner,
        RealmVoterError::GoverningTokenOwnerMustMatch
    );

    // Membership of the Realm the plugin is configured for is not allowed as a source of governance power
    require_neq!(
        token_owner_record.realm,
        registrar.realm,
        RealmVoterError::TokenOwnerRecordFromOwnRealmNotAllowed
    );

    // Setup voter_weight
    voter_weight_record.voter_weight = registrar.realm_member_voter_weight;

    // Record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);

    // Set action and target to None to indicate the weight is valid for any action and target
    voter_weight_record.weight_action = None;
    voter_weight_record.weight_action_target = None;

    Ok(())
}
