use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::program_pack::IsInitialized;

use spl_governance_tools::account::{get_account_data, AccountMaxSize};

use crate::{error::NftVoterError, id};

/// Vote record indicating the given NFT voted on the Proposal
/// The PDA of the record is ["nft-vote-record",proposal,asset_mint]
/// It guarantees uniques and ensures the same NFT can't vote twice
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct AssetVoteRecord {
    /// AssetVoteRecord discriminator sha256("account:AssetVoteRecord")[..8]
    /// Note: The discriminator is used explicitly because AssetVoteRecords
    /// are created and consumed dynamically using remaining_accounts
    /// and Anchor doesn't really support this scenario without going through lots of hoops
    /// Once Anchor has better support for the scenario it shouldn't be necessary
    pub account_discriminator: [u8; 8],

    /// Proposal which was voted on
    pub proposal: Pubkey,

    /// The mint of the NFT which was used for the vote
    pub asset_mint: Pubkey,

    /// The voter who casted this vote
    /// It's a Realm member pubkey corresponding to TokenOwnerRecord.governing_token_owner
    pub governing_token_owner: Pubkey,

    /// Reserved for future upgrades
    pub reserved: [u8; 8],
}

impl AssetVoteRecord {
    /// sha256("account:AssetVoteRecord")[..8]
    pub const ACCOUNT_DISCRIMINATOR: [u8; 8] = [14, 166, 191, 239, 186, 156, 140, 83];
}

impl AccountMaxSize for AssetVoteRecord {}

impl IsInitialized for AssetVoteRecord {
    fn is_initialized(&self) -> bool {
        self.account_discriminator == AssetVoteRecord::ACCOUNT_DISCRIMINATOR
    }
}

/// Returns AssetVoteRecord PDA seeds
pub fn get_nft_vote_record_seeds<'a>(proposal: &'a Pubkey, asset_mint: &'a Pubkey) -> [&'a [u8]; 3] {
    [b"nft-vote-record", proposal.as_ref(), asset_mint.as_ref()]
}

/// Returns AssetVoteRecord PDA address
pub fn get_nft_vote_record_address(proposal: &Pubkey, asset_mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&get_nft_vote_record_seeds(proposal, asset_mint), &id()).0
}

/// Deserializes account and checks owner program
pub fn get_nft_vote_record_data(nft_vote_record_info: &AccountInfo) -> Result<AssetVoteRecord> {
    Ok(get_account_data::<AssetVoteRecord>(
        &id(),
        nft_vote_record_info,
    )?)
}

pub fn get_nft_vote_record_data_for_proposal_and_token_owner(
    nft_vote_record_info: &AccountInfo,
    proposal: &Pubkey,
    governing_token_owner: &Pubkey,
) -> Result<AssetVoteRecord> {
    let nft_vote_record = get_nft_vote_record_data(nft_vote_record_info)?;

    require!(
        nft_vote_record.proposal == *proposal,
        NftVoterError::InvalidProposalForNftVoteRecord
    );

    require!(
        nft_vote_record.governing_token_owner == *governing_token_owner,
        NftVoterError::InvalidTokenOwnerForNftVoteRecord
    );

    Ok(nft_vote_record)
}
