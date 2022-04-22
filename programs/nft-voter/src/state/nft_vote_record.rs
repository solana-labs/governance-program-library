use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use itertools::Itertools;
use solana_program::program_pack::IsInitialized;

use spl_governance_tools::account::{
    create_and_serialize_account_signed, get_account_data, AccountMaxSize,
};

use crate::{error::NftVoterError, id, state::resolve_nft_vote_weight_and_mint, state::*};

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

    /// Reserved for future upgrades
    pub reserved: [u8; 8],
}

impl NftVoteRecord {
    /// sha256("account:NftVoteRecord")[..8]
    pub const ACCOUNT_DISCRIMINATOR: [u8; 8] = [137, 6, 55, 139, 251, 126, 254, 99];
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

/// Deserializes account and checks owner program
pub fn get_nft_vote_record_data(nft_vote_record_info: &AccountInfo) -> Result<NftVoteRecord> {
    Ok(get_account_data::<NftVoteRecord>(
        &id(),
        nft_vote_record_info,
    )?)
}

pub fn get_nft_vote_record_data_for_proposal_and_token_owner(
    nft_vote_record_info: &AccountInfo,
    proposal: &Pubkey,
    governing_token_owner: &Pubkey,
) -> Result<NftVoteRecord> {
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

/// Register NFT votes by saving NftVoteRecord for the given proposal and each nft mint
/// Returns total voter weight implied by the given set of NFTs and Registrar configuration
pub fn register_nft_vote_records<'a>(
    registrar: &Registrar,
    governing_token_owner: &Pubkey,
    proposal: &Pubkey,
    // an array of triplets (nft_info, nft_metadata_info, nft_vote_record_info)
    nft_accounts: &[AccountInfo<'a>],
    payer_info: &AccountInfo<'a>,
    system_info: &AccountInfo<'a>,
) -> Result<u64> {
    let mut voter_weight = 0u64;

    // Ensure all voting nfts in the batch are unique
    let mut unique_nft_mints = vec![];

    let rent = Rent::get()?;

    for (nft_info, nft_metadata_info, nft_vote_record_info) in nft_accounts.iter().tuples() {
        let (nft_vote_weight, nft_mint) = resolve_nft_vote_weight_and_mint(
            registrar,
            governing_token_owner,
            nft_info,
            nft_metadata_info,
            &mut unique_nft_mints,
        )?;

        voter_weight = voter_weight.checked_add(nft_vote_weight as u64).unwrap();

        // Create NFT vote record to ensure the same NFT hasn't been already used for voting
        // Note: The correct PDA of the NftVoteRecord is validated in create_and_serialize_account_signed() below
        // It ensures the NftVoteRecord is for ('nft-vote-record', proposal, nft_mint) seeds
        require!(
            nft_vote_record_info.data_is_empty(),
            NftVoterError::NftAlreadyVoted
        );

        // Note: proposal.governing_token_mint must match voter_weight_record.governing_token_mint
        // We don't verify it here because spl-gov does the check in cast_vote
        // and it would reject voter_weight_record if governing_token_mint doesn't match

        let nft_vote_record = NftVoteRecord {
            account_discriminator: NftVoteRecord::ACCOUNT_DISCRIMINATOR,
            proposal: *proposal,
            nft_mint,
            governing_token_owner: *governing_token_owner,
            reserved: [0; 8],
        };

        // Anchor doesn't natively support dynamic account creation using remaining_accounts
        // and we have to take it on the manual drive
        create_and_serialize_account_signed(
            payer_info,
            nft_vote_record_info,
            &nft_vote_record,
            &get_nft_vote_record_seeds(&proposal, &nft_mint),
            &id(),
            system_info,
            &rent,
        )?;
    }

    Ok(voter_weight)
}
