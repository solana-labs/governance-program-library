use anchor_lang::prelude::*;

use crate::id;

#[account]
#[derive(Default)]
pub struct ProposalNFTVoteRecord {
    /// Proposal which was voted on
    pub proposal: Pubkey,
    /// NFT which was used for the vote
    pub nft: u64,
}

/// Returns ProposalNFTVote PDA seeds
pub fn get_proposal_nft_vote_record_seeds<'a>(
    registrar: &'a Pubkey,
    proposal: &'a Pubkey,
    nft_mint: &'a Pubkey,
) -> [&'a [u8]; 4] {
    [registrar.as_ref(), b"nft-vote", proposal.as_ref(), nft_mint.as_ref()]
}

/// Returns ProposalNFTVote PDA address
pub fn get_proposal_nft_vote_record_address(registrar: &Pubkey, proposal: &Pubkey, nft_mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&get_proposal_nft_vote_record_seeds(registrar, proposal, nft_mint), &id()).0
}
