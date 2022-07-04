use crate::error::RealmVoterError;
use crate::state::max_voter_weight_record::MaxVoterWeightRecord;
use crate::state::*;
use anchor_lang::prelude::*;
use spl_governance::state::realm;

/// Sets MaxVoterWeightRecord and Registrar max_voter_weight to the provided value
/// MaxVoterWeightRecord.max_voter_weight is static and can only be set using this instruction and hence it never expires
#[derive(Accounts)]
pub struct ConfigureMaxVoterWeight<'info> {
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

pub fn configure_max_voter_weight(
    ctx: Context<ConfigureMaxVoterWeight>,
    max_voter_weight: u64,
) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;
    // max_voter_weight on Registrar is redundant and it's only stored for reference and consistency only
    // It's not needed in the current version of the program because it's always set in this instruction together with MaxVoterWeightRecord
    registrar.max_voter_weight = max_voter_weight;

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
