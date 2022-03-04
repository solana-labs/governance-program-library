use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::state::VoterWeightRecord;

#[derive(Accounts)]
#[instruction(governing_token_owner: Pubkey)]
pub struct CreateVoterWeightRecord<'info> {
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
    // let realm_data = get_realm_data(program_id, realm_info)?;
    // realm_data.assert_is_valid_governing_token_mint(governing_token_mint_info.key)?;

    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    voter_weight_record.account_discriminator =
        spl_governance_addin_api::voter_weight::VoterWeightRecord::ACCOUNT_DISCRIMINATOR;

    // Set expiry to expired
    voter_weight_record.voter_weight_expiry = Some(0);

    voter_weight_record.governing_token_owner = governing_token_owner;

    Ok(())
}
