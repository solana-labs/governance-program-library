use std::rc::Rc;

use anchor_lang::prelude::*;
use anchor_lang::{Accounts};
use itertools::Itertools;
use spl_governance::tools::spl_token::{get_spl_token_mint, get_spl_token_owner};
use spl_governance_addin_api::voter_weight::VoterWeightAction;
use spl_governance_tools::account::create_and_serialize_account_signed;
use crate::{state::*, id};
use crate::error::NftVoterError;
use crate::tools::token_metadata::get_token_metadata_for_mint;

#[derive(Accounts)]
#[instruction(proposal: Pubkey)]
pub struct CastNftVote<'info> {
    /// The NFT voting registrar
    pub registrar: Account<'info, Registrar>,

    #[account(
        mut,
        constraint = voter_weight_record.realm == registrar.realm 
        @ NftVoterError::InvalidVoterWeightRecordRealm,

        constraint = voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ NftVoterError::InvalidVoterWeightRecordMint,

        constraint = voter_weight_record.governing_token_owner == governing_token_owner.key()
        @ NftVoterError::InvalidVoterWeightRecordOwner,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,

    /// The token owner who casts the vote
    #[account(mut)]
    pub governing_token_owner: Signer<'info>,
    
    /// The account which pays for the transaction 
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,

}

/// Casts vote with the NFT
pub fn cast_nft_vote(ctx: Context<CastNftVote>, proposal:Pubkey) -> Result<()> {
    let registrar = &ctx.accounts.registrar;

    let mut voter_weight = 0u64;

    // Ensure all nfts are unique
    let mut unique_nft_mints = vec![];

    let rent = Rent::get()?;

    for (nft_info, nft_metadata_info,nft_vote_record_info) in ctx.remaining_accounts.iter().tuples() {
        let nft_owner = get_spl_token_owner(nft_info)?;

    

        // voter_weight_record.governing_token_owner must be the owner of the NFT
        require!(
            nft_owner == ctx.accounts.voter_weight_record.governing_token_owner,
            NftVoterError::VoterDoesNotOwnNft
        );

        // Ensure the same NFT was not provided more than once
        let nft_mint = get_spl_token_mint(nft_info)?;
        if unique_nft_mints.contains(&nft_mint) 
        {
            return Err(NftVoterError::DuplicatedNftDetected.into());
        }

        unique_nft_mints.push(nft_mint);

        let nft_metadata = get_token_metadata_for_mint(nft_metadata_info, &nft_mint)?;

        // The NFT must have a collection and the collection must be verified 
        let collection = nft_metadata.collection.unwrap();

        require!(
            collection.verified,
            NftVoterError::CollectionMustBeVerified
        );

        let collection_config = registrar.get_collection_config(collection.key)?;                                                

        voter_weight = voter_weight.checked_add(collection_config.weight as u64).unwrap();

        // Vote update

        require!(
            nft_vote_record_info.data_is_empty(),
            NftVoterError::NftAlreadyVoted
        );

        let nft_vote_record = NftVoteRecord {
            account_discriminator: NftVoteRecord::ACCOUNT_DISCRIMINATOR,
            proposal,
            nft_mint,
            governing_token_owner: nft_owner,
        };

        // Anchor doesn't natively support dynamic account creation using remaining_accounts
        // and we have to take it on manual drive

        create_and_serialize_account_signed(
            &ctx.accounts.payer,
            &nft_vote_record_info,
            &nft_vote_record,
            &get_nft_vote_record_seeds(&proposal,&nft_mint),
            &id(),
            &ctx.accounts.system_program,
            &rent)?;

    };

    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    voter_weight_record.voter_weight = voter_weight;

    // Record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);

    voter_weight_record.weight_action = Some(VoterWeightAction::CastVote);
    voter_weight_record.weight_action_target = Some(proposal);

    Ok(())
}
