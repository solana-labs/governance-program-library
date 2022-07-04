use crate::error::RealmVoterError;
use crate::state::max_voter_weight_record::MaxVoterWeightRecord;
use crate::state::*;
use anchor_lang::prelude::*;
use spl_governance::state::realm;

/// Updates MaxVoterWeightRecord to evaluate max governance power for the configured Squads
/// This instruction updates MaxVoterWeightRecord which is valid for the current Slot only
/// The instruction must be executed inside the same transaction as the corresponding spl-gov instruction
#[derive(Accounts)]
pub struct ConfigureMaxVoterWeight<'info> {
    /// The Squads voting Registrar
    #[account(mut)]
    pub registrar: Account<'info, Registrar>,

    #[account(
        address = registrar.realm @ RealmVoterError::InvalidRealmForRegistrar,
        owner = registrar.governance_program_id
     )]
    /// CHECK: Owned by spl-governance instance specified in registrar.governance_program_id
    pub realm: UncheckedAccount<'info>,

    /// Authority of the Realm must sign and match realm.authority
    pub realm_authority: Signer<'info>,

    /// MaxVoterWeightRecord for the given registrar.realm and registrar.governing_token_mint
    #[account(
        mut,
        constraint = max_voter_weight_record.realm == registrar.realm
        @ RealmVoterError::InvalidVoterWeightRecordRealm,

        constraint = max_voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ RealmVoterError::InvalidVoterWeightRecordMint,
    )]
    pub max_voter_weight_record: Account<'info, MaxVoterWeightRecord>,
}

pub fn update_max_voter_weight_record(
    ctx: Context<ConfigureMaxVoterWeight>,
    max_voter_weight: u64,
) -> Result<()> {
    let registrar = &ctx.accounts.registrar;

    let realm = realm::get_realm_data_for_governing_token_mint(
        &registrar.governance_program_id,
        &ctx.accounts.realm,
        &registrar.governing_token_mint,
    )?;

    require!(
        realm.authority.unwrap() == ctx.accounts.realm_authority.key(),
        RealmVoterError::InvalidRealmAuthority
    );

    let voter_weight_record = &mut ctx.accounts.max_voter_weight_record;
    voter_weight_record.max_voter_weight = max_voter_weight;

    // max_voter_weight can only be updated using this instruction and it never expires
    voter_weight_record.max_voter_weight_expiry = None;

    Ok(())
}
