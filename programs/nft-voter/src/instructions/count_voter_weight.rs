use anchor_lang::prelude::*;

use crate::state::{voter_weight_counter::VoterWeightCounter, Registrar};

#[derive(Accounts)]
#[instruction(proposal: Pubkey)]
pub struct CountVoterWeight<'info> {
    /// The NFT voting registrar
    pub registrar: Account<'info, Registrar>,

    #[account(
        init_if_needed,
        seeds = [b"voter-weight-counter".as_ref(), proposal.as_ref(), governing_token_owner.key().as_ref()],
        bump,
        payer = payer,
        space = VoterWeightCounter::get_space()
    )]
    pub voter_weight_counter: Account<'info, VoterWeightCounter>,

    /// The token owner whose voter weight is being counted
    pub governing_token_owner: Signer<'info>,

    /// The account which pays for the transaction
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn count_voter_weight<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, CountVoterWeight<'info>>,
    proposal: Pubkey,
) -> Result<()> {
    let voter_weight_counter = &mut ctx.accounts.voter_weight_counter;

    voter_weight_counter.proposal = proposal;
    voter_weight_counter.governing_token_owner = ctx.accounts.governing_token_owner.key();

    Ok(())
}
