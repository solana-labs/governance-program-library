use crate::state::*;
use anchor_lang::prelude::*;

/// Creates VoterWeightRecord used by spl-gov
/// This instruction should only be executed once per realm/governing_token_mint/governing_token_owner
/// to create the account
#[derive(Accounts)]
#[instruction(governing_token_owner: Pubkey)]
pub struct CreateVoterWeightRecord<'info> {
    // The Registrar the VoterWeightRecord account belongs to
    pub registrar: Account<'info, Registrar>,

    #[account(
        init,
        seeds = [ b"voter-weight-record".as_ref(),
                registrar.realm.key().as_ref(),
                registrar.governing_token_mint.key().as_ref(),
                governing_token_owner.as_ref()],
        bump,
        payer = payer,
        space = VoterWeightRecord::get_space()
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_voter_weight_record(
    ctx: Context<CreateVoterWeightRecord>,
    governing_token_owner: Pubkey,
) -> Result<()> {
    let voter_weight_record = &mut ctx.accounts.voter_weight_record;
    let registrar = &ctx.accounts.registrar;

    voter_weight_record.realm = registrar.realm.key();
    voter_weight_record.governing_token_mint = registrar.governing_token_mint.key();
    voter_weight_record.governing_token_owner = governing_token_owner;

    // Set expiry to expired
    voter_weight_record.voter_weight_expiry = Some(0);

    Ok(())
}
