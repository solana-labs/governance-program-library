use anchor_lang::{Accounts, prelude::{Signer, Context}, account};

use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token};

use crate::{state::{Registrar, Collection}};


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
  pub token_program: Program<'info, Token>
  
}

pub fn configure_collection(
  ctx: Context<ConfigureCollection>,
  multiplier: u64
) -> Result<()> {
  let registrar = &mut ctx.accounts.registrar;
  let collection_account = &ctx.accounts.collection;

  // TODO: 
  // check max vote weight
  // Validate multiplier

  registrar.collection = Collection {
    key: collection_account.key(),
    multiplier
  };

  Ok(())
}
