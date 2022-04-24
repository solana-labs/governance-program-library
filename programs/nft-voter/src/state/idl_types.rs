//! IDL only types which are required in IDL but not exported automatically by Anchor
use anchor_lang::prelude::*;

/// NftVoteRecord exported to IDL without account_discriminator
/// TODO: Once we can support these accounts in Anchor via remaining_accounts then it should be possible to remove it
#[account]
pub struct NftVoteRecord {
    /// Proposal which was voted on
    pub proposal: Pubkey,

    /// The mint of the NFT which was used for the vote
    pub nft_mint: Pubkey,

    /// The voter who casted this vote
    /// It's a Realm member pubkey corresponding to TokenOwnerRecord.governing_token_owner
    pub governing_token_owner: Pubkey,
}
