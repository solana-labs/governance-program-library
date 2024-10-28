use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use spl_governance::state::realm;

use crate::{
    error::BonkPluginError, state::*, utils::stake_pool::StakePool, SPL_TOKEN_STAKING_PROGRAM_ID,
};

/// Creates Registrar storing Stake Pool details for Bonk
/// This instruction should only be executed once per realm/governing_token_mint to create the account
#[derive(Accounts)]
#[instruction()]
pub struct CreateRegistrar<'info> {
    #[account(
    init,
    seeds = [
      b"registrar".as_ref(),
      realm.key().as_ref(),
      governing_token_mint.key().as_ref()],
    bump,
    payer = payer,
    space = 8 + Registrar::INIT_SPACE
  )]
    pub registrar: Account<'info, Registrar>,

    #[account(executable)]
    /// CHECK: Can be any instance of spl-governance and it's not known at the compilation time
    pub governance_program_id: UncheckedAccount<'info>,

    /// CHECK: The account data is not used
    pub previous_voter_weight_plugin_program_id: Option<UncheckedAccount<'info>>,

    /// CHECK: Owned by spl-governance instance specified in governance_program_id
    #[account(owner = governance_program_id.key())]
    pub realm: UncheckedAccount<'info>,

    /// CHECK: Owned by SPL Staking Program
    #[account(
        owner = SPL_TOKEN_STAKING_PROGRAM_ID,
    )]
    pub stake_pool: AccountInfo<'info>,

    pub governing_token_mint: Account<'info, Mint>,
    pub realm_authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_registrar_handler(ctx: Context<CreateRegistrar>) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;
    registrar.governance_program_id = ctx.accounts.governance_program_id.key();
    registrar.realm = ctx.accounts.realm.key();
    registrar.realm_authority = ctx.accounts.realm_authority.key();
    registrar.governing_token_mint = ctx.accounts.governing_token_mint.key();
    registrar.stake_pool = ctx.accounts.stake_pool.key();

    if let Some(previous_voter_weight_plugin_program_info) =
        &ctx.accounts.previous_voter_weight_plugin_program_id
    {
        registrar.previous_voter_weight_plugin_program_id =
            Some(previous_voter_weight_plugin_program_info.key());
    }

    // Verify that realm_authority is the expected authority of the Realm
    // and that the mint matches one of the realm mints too
    let realm = realm::get_realm_data_for_governing_token_mint(
        &registrar.governance_program_id,
        &ctx.accounts.realm,
        &registrar.governing_token_mint,
    )?;

    let stake_pool = StakePool::deserialize_checked(&ctx.accounts.stake_pool)?;

    require!(
        realm.authority.unwrap() == ctx.accounts.realm_authority.key(),
        BonkPluginError::InvalidRealmAuthority
    );

    require!(
        stake_pool.mint == ctx.accounts.governing_token_mint.key(),
        BonkPluginError::InvalidGoverningToken
    );

    Ok(())
}
