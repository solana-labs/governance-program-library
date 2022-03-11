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
    ctx: Context<UpdateVoterWeightRecord>,
    _realm: Pubkey,
    _governing_token_mint: Pubkey,
    _governing_token_owner: Pubkey,
) -> Result<()> {
    // TODO: Validate registrar vs VoterWeightRecord
    // TODO: Validate governing_token_owner

    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    // TODO: Pass NFTs and evaluate the same way as Vote does
    voter_weight_record.voter_weight = 10;

    // Record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);
    // TODO: Always set action
    voter_weight_record.weight_action = None;
    voter_weight_record.weight_action_target = None;

    Ok(())
}
