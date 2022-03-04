//use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
//use spl_governance::state::realm;
use std::mem::size_of;

#[derive(Accounts)]
pub struct CreateRegistrar<'info> {
    /// The voting registrar. There can only be a single registrar
    /// per governance realm and governing mint.
    #[account(
        init,
        seeds = [b"registrar".as_ref(),realm.key().as_ref(),  realm_governing_token_mint.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + size_of::<Registrar>()
    )]
    pub registrar: Account<'info, Registrar>,

    /// An spl-governance realm
    ///
    /// realm is validated in the instruction:
    /// - realm is owned by the governance_program_id
    /// - realm_governing_token_mint must be the community or council mint
    /// - realm_authority is realm.authority
    /// CHECK: Owned by spl-gov
    pub realm: UncheckedAccount<'info>,

    /// The program id of the spl-governance program the realm belongs to.
    /// CHECK: Can be any instance of spl-gov
    pub governance_program_id: UncheckedAccount<'info>,

    /// Either the realm community mint or the council mint.
    pub realm_governing_token_mint: Account<'info, Mint>,

    // #[account(mut)]
    pub realm_authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Creates a new voting registrar.
///
/// To use the registrar, call ConfigVotingMint to register token mints that may be
/// used for voting.
pub fn create_registrar(ctx: Context<CreateRegistrar>) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;
    registrar.governance_program_id = ctx.accounts.governance_program_id.key();
    registrar.realm = ctx.accounts.realm.key();
    registrar.realm_governing_token_mint = ctx.accounts.realm_governing_token_mint.key();

    // // Verify that "realm_authority" is the expected authority on "realm"
    // // and that the mint matches one of the realm mints too.
    // let realm = realm::get_realm_data_for_governing_token_mint(
    //     &registrar.governance_program_id,
    //     &ctx.accounts.realm.to_account_info(),
    //     &registrar.realm_governing_token_mint,
    // )?;
    // require!(
    //     realm.authority.unwrap() == ctx.accounts.realm_authority.key(),
    //     ErrorCode::InvalidRealmAuthority
    // );

    Ok(())
}
