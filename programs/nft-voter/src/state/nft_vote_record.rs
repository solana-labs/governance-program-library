use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::program_pack::IsInitialized;

use spl_governance_tools::account::AccountMaxSize;

use crate::id;

/// Vote record indicating the given NFT voted on the Proposal
/// The PDA of the record is ["nft-vote-record",proposal,nft_mint]
/// It guarantees uniques and ensures the same NFT can't vote twice
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct NftVoteRecord {
    /// NftVoteRecord discriminator sha256("account:NftVoteRecord")[..8]
    /// Note: The discriminator is used explicitly because NftVoteRecords
    /// are created and consumed dynamically using remaining_accounts
    /// and Anchor doesn't really support this scenario without going through lots of hoops
    /// Once Anchor has better support for the scenario it shouldn't be necessary
    pub account_discriminator: [u8; 8],

    /// Proposal which was voted on
    pub proposal: Pubkey,

    /// The mint of the NFT which was used for the vote
    pub nft_mint: Pubkey,

    /// The voter who casted this vote
    /// It's a Realm member pubkey corresponding to TokenOwnerRecord.governing_token_owner
    pub governing_token_owner: Pubkey,
}

impl NftVoteRecord {
    /// sha256("account:NftVoteRecord")[..8]
    pub const ACCOUNT_DISCRIMINATOR: [u8; 8] = *b"8906378b";
}

impl AccountMaxSize for NftVoteRecord {}

impl IsInitialized for NftVoteRecord {
    fn is_initialized(&self) -> bool {
        self.account_discriminator == NftVoteRecord::ACCOUNT_DISCRIMINATOR
    }
}

/// Returns NftVoteRecord PDA seeds
pub fn get_nft_vote_record_seeds<'a>(proposal: &'a Pubkey, nft_mint: &'a Pubkey) -> [&'a [u8]; 3] {
    [b"nft-vote-record", proposal.as_ref(), nft_mint.as_ref()]
}

/// Returns NftVoteRecord PDA address
pub fn get_nft_vote_record_address(proposal: &Pubkey, nft_mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&get_nft_vote_record_seeds(proposal, nft_mint), &id()).0
}
