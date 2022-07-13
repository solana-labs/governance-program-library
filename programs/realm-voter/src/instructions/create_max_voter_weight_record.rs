use anchor_lang::prelude::*;

use crate::state::{max_voter_weight_record::MaxVoterWeightRecord, Registrar};

/// Creates MaxVoterWeightRecord used by spl-governance
/// This instruction should only be executed once per realm/governing_token_mint to create the account
#[derive(Accounts)]
pub struct CreateMaxVoterWeightRecord<'info> {
    // The Registrar the MaxVoterWeightRecord account belongs to
    pub registrar: Account<'info, Registrar>,

    #[account(
        init,
        seeds = [ b"max-voter-weight-record".as_ref(),
                registrar.realm.key().as_ref(),
                registrar.governing_token_mint.key().as_ref()],
        bump,
        payer = payer,
        space = MaxVoterWeightRecord::get_space()
    )]
    pub max_voter_weight_record: Account<'info, MaxVoterWeightRecord>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_max_voter_weight_record(ctx: Context<CreateMaxVoterWeightRecord>) -> Result<()> {
    let max_voter_weight_record = &mut ctx.accounts.max_voter_weight_record;
    let registrar = &ctx.accounts.registrar;

    max_voter_weight_record.realm = registrar.realm;
    max_voter_weight_record.governing_token_mint = registrar.governing_token_mint;

    // Set expiry to expired
    max_voter_weight_record.max_voter_weight_expiry = Some(0);

    Ok(())
}
