use anchor_lang::{
    account,
    prelude::{Context, Signer},
    Accounts,
};

use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

use crate::state::{CollectionConfig, Registrar};

#[derive(Accounts)]
pub struct ConfigureCollection<'info> {
    /// Registrar for which we configure this Collection
    #[account(mut)]
    pub registrar: Account<'info, Registrar>,
    /// Authority of the Realm
    #[account(mut)]
    pub realm_authority: Signer<'info>,
    // Collection which is going to be used for voting
    #[account(
    constraint = collection.owner == token_program.key() @ ErrorCode::AccountOwnedByWrongProgram
  )]
    pub collection: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn configure_collection(
    ctx: Context<ConfigureCollection>,
    weight: u16,
    size: u32,
) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;
    let collection_account = &ctx.accounts.collection;

    // TODO:
    // check max vote weight
    // Validate multiplier
    // Ensure realm.authority signed

    registrar.collection_configs.push(CollectionConfig {
        collection: collection_account.key(),
        weight: weight,
        reserved: [0; 8],
        size: size,
    });

    Ok(())
}
