use anchor_lang::{
    account,
    prelude::{Context, Signer},
    Accounts,
};

use anchor_lang::prelude::*;

use crate::state::Registrar;

#[derive(Accounts)]
pub struct ConfigureCollection<'info> {
    /// Registrar for which we configure this Collection
    #[account(mut)]
    pub registrar: Account<'info, Registrar>,

    pub realm_authority: Signer<'info>,
    // TODO: add collection
}

#[allow(dead_code)]
pub fn configure_collection(
    ctx: Context<ConfigureCollection>,
    _collection_creator: Pubkey,
) -> Result<()> {
    let _registrar = &mut ctx.accounts.registrar;

    // TODO:
    // - Check collection
    // - Set up collection data
    // - check max vote weight for the overflow
    // - add collection to the registrar
    Ok(())
}
