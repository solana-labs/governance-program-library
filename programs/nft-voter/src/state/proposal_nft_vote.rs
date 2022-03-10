use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct ProposalNFTVote {
    /// Proposal which was voted on
    pub proposal: Pubkey,
    /// NFT which was used for the vote
    pub nft: u64,
}
