use anchor_lang::prelude::*;

use crate::id;

/// Vote record indicating the given NFT voted on the Proposal
#[account]
#[derive(Default)]
pub struct NftVoteRecord {
    /// Proposal which was voted on
    pub proposal: Pubkey,

    /// The mint of the NFT which was used for the vote
    pub nft_mint: u64,

    /// The voter (Realm member) who casted this vote
    pub governing_token_owner: Pubkey,
}

/// Returns NftVoteRecord PDA seeds
pub fn get_nft_vote_record_seeds<'a>(proposal: &'a Pubkey, nft_mint: &'a Pubkey) -> [&'a [u8]; 3] {
    [b"nft-vote", proposal.as_ref(), nft_mint.as_ref()]
}

/// Returns NftVoteRecord PDA address
pub fn get_nft_vote_record_address(proposal: &Pubkey, nft_mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&get_nft_vote_record_seeds(proposal, nft_mint), &id()).0
}
