use crate::error::QuadraticError;
use crate::state::quadratic_coefficients::QuadraticCoefficients;
use crate::state::*;
use anchor_lang::prelude::*;
use spl_governance::state::realm;

/// Configures the quadratic Registrar,
/// allowing the gatekeeper network or previous plugin to be updated
#[derive(Accounts)]
#[instruction(coefficients: QuadraticCoefficients, use_previous_voter_weight_plugin:bool)]
pub struct ConfigureRegistrar<'info> {
    /// The quadratic Plugin Registrar to be updated
    #[account(mut)]
    pub registrar: Account<'info, Registrar>,

    /// An spl-governance Realm
    ///
    /// Realm is validated in the instruction:
    /// - Realm is owned by the governance_program_id
    /// - realm_authority is realm.authority
    ///
    /// CHECK: Owned by spl-governance instance specified in governance_program_id
    #[account(
        address = registrar.realm @ QuadraticError::InvalidRealmForRegistrar,
        owner = registrar.governance_program_id.key()
    )]
    pub realm: UncheckedAccount<'info>,

    /// realm_authority must sign and match Realm.authority
    pub realm_authority: Signer<'info>,
}

/// Configures a Registrar, setting a new previous voter weight plugin
pub fn configure_registrar(
    ctx: Context<ConfigureRegistrar>,
    coefficients: QuadraticCoefficients,
    use_previous_voter_weight_plugin: bool,
) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;

    registrar.quadratic_coefficients = coefficients;

    let remaining_accounts = &ctx.remaining_accounts;

    // If the plugin has a previous plugin, it "inherits" the vote weight from a vote_weight_account owned
    // by the previous plugin. This chain is registered here.
    registrar.previous_voter_weight_plugin_program_id = use_previous_voter_weight_plugin
        .then(|| {
            remaining_accounts
                .first()
                .ok_or(QuadraticError::MissingPreviousVoterWeightPlugin)
                .map(|account| account.key)
        })
        .transpose()?
        .cloned();

    // Verify that realm_authority is the expected authority of the Realm
    // and that the mint matches one of the realm mints too.
    let realm = realm::get_realm_data_for_governing_token_mint(
        &registrar.governance_program_id,
        &ctx.accounts.realm,
        &registrar.governing_token_mint,
    )?;
    require_eq!(
        realm.authority.unwrap(),
        ctx.accounts.realm_authority.key(),
        QuadraticError::InvalidRealmAuthority
    );

    Ok(())
}
