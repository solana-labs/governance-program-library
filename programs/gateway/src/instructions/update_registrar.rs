use crate::error::GatewayError;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use spl_governance::state::realm;

/// Updates the  Registrar for spl-gov Realm
/// This instruction should only be executed once per realm/governing_token_mint to create the account
#[derive(Accounts)]
pub struct UpdateRegistrar<'info> {
    /// The Gateway Plugin Registrar to be updated
    #[account(mut)]
    pub registrar: Account<'info, Registrar>,

    /// An spl-governance Realm
    ///
    /// Realm is validated in the instruction:
    /// - Realm is owned by the governance_program_id
    /// - realm_authority is realm.authority
    /// CHECK: Owned by spl-governance instance specified in governance_program_id
    #[account(owner = governance_program_id.key())]
    pub realm: UncheckedAccount<'info>,

    /// realm_authority must sign and match Realm.authority
    pub realm_authority: Signer<'info>,

    /// The new Identity.com Gateway gatekeeper network
    /// (See the registry struct docs for details).
    /// CHECK: This can be any public key. The gateway library checks that the provided
    /// Gateway Token belongs to this gatekeeper network, so passing a particular key here is
    /// essentially saying "We trust this gatekeeper network".
    pub gatekeeper_network: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Updates a Registrar gatekeeperNetwork or a previous plugin registrar
pub fn update_registrar(ctx: Context<CreateRegistrar>) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;
    registrar.gatekeeper_network = ctx.accounts.gatekeeper_network.key();

    // If the plugin has a previous plugin registrar, it "inherits" the vote weight from a vote_weight_account owned
    // by the previous plugin. This chain is registered here.
    let previous_vote_weight_plugin_registrar = ctx.remaining_accounts.get(0);
    registrar.previous_voting_weight_plugin_registrar = previous_vote_weight_plugin_registrar.map(|account| account.key());

    // Verify that realm_authority is the expected authority of the Realm
    // and that the mint matches one of the realm mints too.
    let realm = realm::get_realm_data_for_governing_token_mint(
        &registrar.governance_program_id,
        &ctx.accounts.realm,
        &registrar.governing_token_mint,
    )?;
    require!(
        realm.authority.unwrap() == ctx.accounts.realm_authority.key(),
        GatewayError::InvalidRealmAuthority
    );

    Ok(())
}
