use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use spl_governance::state::realm::get_realm_data;

use crate::state::{Registrar, VoterWeightRecord};

#[derive(Accounts)]
#[instruction(governing_token_owner: Pubkey)]
pub struct CreateVoterWeightRecord<'info> {
    #[account(
        seeds = [b"registrar".as_ref(),realm.key().as_ref(), realm_governing_token_mint.key().as_ref()],
        bump,

    )]
    pub registrar: Account<'info, Registrar>,

    #[account(
        init,
        seeds = [ b"voter-weight-record".as_ref(),
                realm.key().as_ref(),
                realm_governing_token_mint.key().as_ref(),
                governing_token_owner.as_ref()],
        bump,
        payer = payer
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,

    /// CHECK: Owned by spl-gov
    pub realm: UncheckedAccount<'info>,

    /// Either the realm community mint or the council mint.
    pub realm_governing_token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_voter_weight_record(
    ctx: Context<CreateVoterWeightRecord>,
    governing_token_owner: Pubkey,
) -> Result<()> {
    let realm = get_realm_data(
        &ctx.accounts.registrar.governance_program_id,
        &ctx.accounts.realm,
    )?;
    realm.assert_is_valid_governing_token_mint(&ctx.accounts.realm_governing_token_mint.key())?;

    // TODO: Assert register matched realm and  realm_governing_token_mint

    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    voter_weight_record.account_discriminator =
        spl_governance_addin_api::voter_weight::VoterWeightRecord::ACCOUNT_DISCRIMINATOR;

    voter_weight_record.realm = ctx.accounts.realm.key();
    voter_weight_record.governing_token_mint = ctx.accounts.realm_governing_token_mint.key();
    voter_weight_record.governing_token_owner = governing_token_owner;

    // Set expiry to expired
    voter_weight_record.voter_weight_expiry = Some(0);

    Ok(())
}
