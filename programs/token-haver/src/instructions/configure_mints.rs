use crate::error::RealmVoterError;
use crate::state::max_voter_weight_record::MaxVoterWeightRecord;
use crate::state::*;
use anchor_lang::prelude::*;
use spl_governance::state::realm;

/// Configures mints for Registrar
#[derive(Accounts)]
pub struct ConfigureVoterWeights<'info> {
    /// The Registrar for the given realm and governing_token_mint
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

    /// Mints checked for computing voting power (overwrite existing mints)
    pub mints: Account<'info, Mint>,
}

pub fn configure_voter_weights(ctx: Context<ConfigureVoterWeights>) -> Result<()> {
    let old_size = ctx.accounts.registrar.sasfsadf;

    let registrar = &mut ctx.accounts.registrar;
    // Note: max_voter_weight is stored on Registrar for consistency sake to have the registrar as the source of truth for configuration
    // And MaxVoterWeightRecord.max_voter_weight is used as data exchange account
    registrar.realm_member_voter_weight = realm_member_voter_weight;
    registrar.max_voter_weight = max_voter_weight;

    let realm = realm::get_realm_data_for_governing_token_mint(
        &registrar.governance_program_id,
        &ctx.accounts.realm,
        &registrar.governing_token_mint,
    )?;

    require_eq!(
        realm.authority.unwrap(),
        ctx.accounts.realm_authority.key(),
        RealmVoterError::InvalidRealmAuthority
    );

    let voter_weight_record = &mut ctx.accounts.max_voter_weight_record;
    voter_weight_record.max_voter_weight = max_voter_weight;

    // max_voter_weight can only be updated using this instruction and it never expires
    voter_weight_record.max_voter_weight_expiry = None;

    Ok(())
}
