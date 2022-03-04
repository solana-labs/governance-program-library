use anchor_lang::prelude::*;

use crate::state::{Registrar, VoterWeightRecord};

#[derive(Accounts)]
#[instruction(realm:Pubkey, governing_token_mint:Pubkey, governing_token_owner: Pubkey)]
pub struct UpdateVoterWeightRecord<'info> {
    #[account(
        seeds = [b"registrar".as_ref(),realm.as_ref(),  governing_token_mint.as_ref()],
        bump,
    )]
    pub registrar: Account<'info, Registrar>,

    #[account(
        mut,
        seeds = [ b"voter-weight-record".as_ref(),
                realm.as_ref(),
                governing_token_mint.as_ref(),
                governing_token_owner.as_ref()],
        bump,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,
}

pub fn update_voter_weight_record(
    _ctx: Context<UpdateVoterWeightRecord>,
    _realm: Pubkey,
    _governing_token_mint: Pubkey,
    _governing_token_owner: Pubkey,
) -> Result<()> {
    // let realm = get_realm_data(
    //     &ctx.accounts.registrar.governance_program_id,
    //     &ctx.accounts.realm,
    // )?;
    // realm.assert_is_valid_governing_token_mint(&ctx.accounts.realm_governing_token_mint.key())?;

    // // TODO: Assert register matched realm and  realm_governing_token_mint

    // let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    // voter_weight_record.account_discriminator =
    //     spl_governance_addin_api::voter_weight::VoterWeightRecord::ACCOUNT_DISCRIMINATOR;

    // voter_weight_record.realm = ctx.accounts.realm.key();
    // voter_weight_record.governing_token_mint = ctx.accounts.realm_governing_token_mint.key();
    // voter_weight_record.governing_token_owner = governing_token_owner;

    // // Set expiry to expired
    // voter_weight_record.voter_weight_expiry = Some(0);

    Ok(())
}
