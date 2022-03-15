use anchor_lang::prelude::*;

use crate::state::Registrar;

/// Disposes NftVoteRecord and recovers the rent from the accounts   
/// It can only be executed when voting on the target Proposal ended or voter withdrew vote from the Proposal
#[derive(Accounts)]
#[instruction(realm:Pubkey, governing_token_mint:Pubkey, governing_token_owner: Pubkey)]
pub struct RelinquishNftVote<'info> {
    #[account(
        seeds = [b"registrar".as_ref(),realm.as_ref(),  governing_token_mint.as_ref()],
        bump,
    )]
    pub registrar: Account<'info, Registrar>,
}

pub fn relinquish_nft_vote(
    _ctx: Context<RelinquishNftVote>,
    _realm: Pubkey,
    _governing_token_mint: Pubkey,
    _governing_token_owner: Pubkey,
) -> Result<()> {
    // TODO: Validate registrar vs VoterWeightRecord
    // TODO: Validate governing_token_owner

    // TODO: remove proposal/vote record
    // TODO: relinquish from spl_gov or ensure the proposal is not in voting state

    Ok(())
}
