use crate::error::TokenHaverError;
use crate::state::*;
use anchor_lang::system_program::Transfer;
use anchor_lang::{prelude::*, system_program};
use spl_governance::state::realm;

/// Configures mints for Registrar
#[derive(Accounts)]
#[instruction(mints: Vec<Pubkey>)]
pub struct ConfigureMints<'info> {
    /// The Registrar for the given realm and governing_token_mint
    #[account(mut)]
    pub registrar: Account<'info, Registrar>,

    #[account(
        address = registrar.realm @ TokenHaverError::InvalidRealmForRegistrar,
        owner = registrar.governance_program_id
     )]
    /// CHECK: Owned by spl-governance instance specified in registrar.governance_program_id
    pub realm: UncheckedAccount<'info>,

    // will pay in the event of a resize
    pub payer: Signer<'info>,

    /// Authority of the Realm must sign and match realm.authority
    pub realm_authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn configure_mints(ctx: Context<ConfigureMints>, mints: Vec<Pubkey>) -> Result<()> {
    let new_size = Registrar::get_space(mints.len() as u8);

    let rent = Rent::get()?;
    let new_minimum_balance = rent.minimum_balance(new_size);

    let lamports_diff =
        new_minimum_balance.saturating_sub(ctx.accounts.registrar.to_account_info().lamports());

    // if lamports_diff is positive, we need to fund the account
    if lamports_diff > 0 {
        // Create a CPI context for the transfer
        let cpi_accounts = Transfer {
            from: ctx.accounts.payer.to_account_info().clone(),
            to: ctx.accounts.registrar.to_account_info().clone(),
        };

        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Perform the transfer
        system_program::transfer(cpi_ctx, lamports_diff)?;
    }

    let registrar = &mut ctx.accounts.registrar;
    registrar.to_account_info().realloc(new_size, false)?;

    registrar.mints = mints;

    let realm = realm::get_realm_data_for_governing_token_mint(
        &registrar.governance_program_id,
        &ctx.accounts.realm,
        &registrar.governing_token_mint,
    )?;

    require_eq!(
        realm.authority.unwrap(),
        ctx.accounts.realm_authority.key(),
        TokenHaverError::InvalidRealmAuthority
    );

    Ok(())
}
