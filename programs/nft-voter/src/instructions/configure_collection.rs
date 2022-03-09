use anchor_lang::{
    account,
    prelude::{Context, Signer},
    Accounts,
};

use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use spl_governance::state::realm;

use crate::error::NftLockerError;
use crate::state::{CollectionConfig, Registrar};

#[derive(Accounts)]
pub struct ConfigureCollection<'info> {
    /// Registrar for which we configure this Collection
    #[account(mut,
        constraint = registrar.realm == realm.key() @ NftLockerError::InvalidRegistrarRealm
    )]
    pub registrar: Account<'info, Registrar>,

    /// CHECK: Owned by spl-governance instance specified in registrar.governance_program_id
    pub realm: UncheckedAccount<'info>,

    /// Authority of the Realm
    #[account(mut)]
    pub realm_authority: Signer<'info>,

    // Collection which is going to be used for voting
    // #[account(
    // constraint = collection.owner == token_program.key() @ ErrorCode::AccountOwnedByWrongProgram
    // )]
    pub collection: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn configure_collection(
    ctx: Context<ConfigureCollection>,
    weight: u16,
    size: u32,
) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;

    let realm = realm::get_realm_data_for_governing_token_mint(
        &registrar.governance_program_id,
        &ctx.accounts.realm.to_account_info(),
        &registrar.governing_token_mint,
    )?;

    require!(
        realm.authority.unwrap() == ctx.accounts.realm_authority.key(),
        NftLockerError::InvalidRealmAuthority
    );

    let collection_account = &ctx.accounts.collection;

    // TODO:
    // check max vote weight
    // Validate multiplier
    // Ensure realm.authority signed

    registrar.collection_configs.push(CollectionConfig {
        collection: collection_account.key(),
        weight,
        reserved: [0; 8],
        size,
    });

    Ok(())
}
