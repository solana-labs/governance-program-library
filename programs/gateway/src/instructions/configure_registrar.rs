use crate::error::GatewayError;
use crate::state::*;
use anchor_lang::prelude::*;
use spl_governance::state::realm;

/// Configures the Gateway Registrar,
/// allowing the gatekeeper network or previous plugin to be updated
#[derive(Accounts)]
#[instruction(use_previous_voter_weight_plugin:bool)]
pub struct ConfigureRegistrar<'info> {
    /// The Gateway Plugin Registrar to be updated
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
        address = registrar.realm @ GatewayError::InvalidRealmForRegistrar,
        owner = registrar.governance_program_id.key()
    )]
    pub realm: UncheckedAccount<'info>,

    /// realm_authority must sign and match Realm.authority
    pub realm_authority: Signer<'info>,

    /// The new Identity.com Gateway gatekeeper network
    /// (See the registry struct docs for details).
    /// CHECK: This can be any public key. The gateway library checks that the provided
    /// Gateway Token belongs to this gatekeeper network, so passing a particular key here is
    /// essentially saying "We trust this gatekeeper network".
    pub gatekeeper_network: UncheckedAccount<'info>,
}

/// Configures a Registrar, updating the gatekeeperNetwork or the previous plugin program ID
pub fn configure_registrar(
    ctx: Context<ConfigureRegistrar>,
    use_previous_voter_weight_plugin: bool,
) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;
    registrar.gatekeeper_network = ctx.accounts.gatekeeper_network.key();

    let remaining_accounts = &ctx.remaining_accounts;

    // If the plugin has a previous plugin, it "inherits" the vote weight from a vote_weight_account owned
    // by the previous plugin. This chain is registered here.
    registrar.previous_voter_weight_plugin_program_id = use_previous_voter_weight_plugin
        .then(|| {
            remaining_accounts
                .first()
                .ok_or(GatewayError::MissingPreviousVoterWeightPlugin)
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
        GatewayError::InvalidRealmAuthority
    );

    Ok(())
}
