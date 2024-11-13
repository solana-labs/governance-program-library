use {
    crate::{error::*, state::*},
    anchor_lang::prelude::*,
    anchor_spl::token_interface::Mint,
    spl_governance::state::realm,
};

/// Creates Registrar storing Realm Voter configuration for spl-governance Realm
/// This instruction should only be executed once per realm/governing_token_mint to create the account
#[derive(Accounts)]
#[instruction(max_mints: u8)]
pub struct CreateRegistrar<'info> {
    /// The Realm Voter Registrar
    /// There can only be a single registrar per governance Realm and governing mint of the Realm
    #[account(
        init,
        seeds = [b"registrar".as_ref(), realm.key().as_ref(), governing_token_mint.key().as_ref()],
        bump,
        payer = payer,
        space = Registrar::get_space(max_mints)
    )]
    pub registrar: Account<'info, Registrar>,

    /// The program id of the spl-governance program the realm belongs to
    /// CHECK: Can be any instance of spl-governance and it's not known at the compilation time
    #[account(executable)]
    pub governance_program_id: UncheckedAccount<'info>,

    /// An spl-governance Realm
    ///
    /// Realm is validated in the instruction:
    /// - Realm is owned by the governance_program_id
    /// - governing_token_mint must be the community or council mint
    /// - realm_authority is realm.authority
    ///
    /// CHECK: Owned by spl-governance instance specified in governance_program_id
    #[account(owner = governance_program_id.key())]
    pub realm: UncheckedAccount<'info>,

    /// Either the realm community mint or the council mint.
    /// It must match Realm.community_mint or Realm.config.council_mint
    ///
    /// Note: Once the Realm voter plugin is enabled the governing_token_mint is used only as identity
    /// for the voting population and the tokens of that are no longer used
    pub governing_token_mint: InterfaceAccount<'info, Mint>,

    /// realm_authority must sign and match Realm.authority
    pub realm_authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Creates a new Registrar which stores Realms voter configuration for the given Realm
///
/// max_mints is used to allocate account size for the maximum number of configured mint instances
pub fn create_registrar(ctx: Context<CreateRegistrar>, max_mints: u8) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;
    registrar.governance_program_id = ctx.accounts.governance_program_id.key();
    registrar.realm = ctx.accounts.realm.key();
    registrar.governing_token_mint = ctx.accounts.governing_token_mint.key();
    registrar.voting_mint_configs = vec![];
    registrar.max_mints = max_mints;

    // Verify that realm_authority is the expected authority of the Realm
    // and that the mint matches one of the realm mints too
    let realm = realm::get_realm_data_for_governing_token_mint(
        &registrar.governance_program_id,
        &ctx.accounts.realm,
        &registrar.governing_token_mint,
    )?;

    require_eq!(
        realm.authority.unwrap(),
        ctx.accounts.realm_authority.key(),
        TokenVoterError::InvalidRealmAuthority
    );

    Ok(())
}
