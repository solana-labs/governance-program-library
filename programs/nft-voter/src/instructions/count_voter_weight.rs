use anchor_lang::prelude::*;

use crate::{
    error::NftVoterError,
    state::{
        nft_vote_record::register_nft_vote_records, voter_weight_counter::VoterWeightCounter,
        Registrar,
    },
    tools::anchor::is_new_account,
};

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

    if is_new_account(&voter_weight_counter.to_account_info()) {
        voter_weight_counter.registrar = ctx.accounts.registrar.key();
        voter_weight_counter.proposal = proposal;
        voter_weight_counter.governing_token_owner = ctx.accounts.governing_token_owner.key();
    } else {
        require!(
            voter_weight_counter.registrar == ctx.accounts.registrar.key(),
            NftVoterError::InvalidRegistrarForVoterWeightCounter
        );
        require!(
            voter_weight_counter.proposal == proposal,
            NftVoterError::InvalidProposalForVoterWeightCounter
        );
        require!(
            voter_weight_counter.governing_token_owner == ctx.accounts.governing_token_owner.key(),
            NftVoterError::InvalidTokenOwnerForVoterWeightCounter
        );
    }

    // Record voting NFTs and get total weight
    let voter_weight = register_nft_vote_records(
        &ctx.accounts.registrar,
        &ctx.accounts.governing_token_owner.key(),
        &proposal,
        ctx.remaining_accounts,
        &ctx.accounts.payer.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
    )?;

    voter_weight_counter.voter_weight = voter_weight_counter
        .voter_weight
        .checked_add(voter_weight)
        .unwrap();

    Ok(())
}
