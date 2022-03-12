use anchor_lang::prelude::*;
use anchor_lang::{Accounts};
use anchor_spl::token::{TokenAccount, Token};
use mpl_token_metadata::state::{Collection};
use spl_governance::tools::spl_token::{get_spl_token_mint, get_spl_token_owner};
use spl_governance_addin_api::voter_weight::VoterWeightAction;
use std::mem::size_of;
use crate::state::*;
use crate::error::NftVoterError;
use crate::ErrorCode::AccountOwnedByWrongProgram;
use crate::tools::token_metadata::get_token_metadata_for_mint;

#[derive(Accounts)]
#[instruction(proposal: Pubkey)]
pub struct VoteWithNFT<'info> {
    /// The voting registrar
    #[account()]
    pub registrar: Account<'info, Registrar>,
    /// Record that nft from nft_account was used to vote on the proposal
    #[account(
        init,
        seeds = [
            b"nft-vote".as_ref(), 
            proposal.as_ref(),
            nft_token.mint.as_ref()
            ],
        bump,
        payer = payer,
        space = 8 + size_of::<ProposalNFTVoteRecord>()
    )]
    pub proposal_nft_vote_record: Account<'info, ProposalNFTVoteRecord>,
    /// Account holding the NFT
    #[account(
        constraint = nft_token.amount > 0 @ NftVoterError::InsufficientAmountOnNFTAccount,
        constraint = nft_token.owner == token_program.key() @ AccountOwnedByWrongProgram
    )]
    pub nft_token: Account<'info, TokenAccount>,
    /// Metadata account of the NFT
    pub nft_metadata: UncheckedAccount<'info>,
    #[account(
        mut,
        constraint = voter_weight_record.realm == registrar.realm 
        @ NftVoterError::InvalidVoterWeightRecordRealm,

        constraint = voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ NftVoterError::InvalidVoterWeightRecordMint,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,
    /// Voter is a signer  
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}

/// Casts vote with the NFT
pub fn vote_with_nft(ctx: Context<VoteWithNFT>, proposal:Pubkey) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    let voter_weight_record = &mut ctx.accounts.voter_weight_record;
    
    let nft_token = &ctx.accounts.nft_token;
    let nft_token_owner = get_spl_token_owner(&nft_token.to_account_info())?;
    
     require!(
        nft_token_owner == voter_weight_record.governing_token_owner,
        NftVoterError::VoterDoesNotOwnNft
    );

    let nft_token_mint = get_spl_token_mint(&nft_token.to_account_info())?;
    let nft_metadata = get_token_metadata_for_mint(&ctx.accounts.nft_metadata,&nft_token_mint)?;

    let collection: Collection = nft_metadata.collection.ok_or(NftVoterError::NotPartOfCollection)?;
    let collection_idx = registrar.collection_config_index(collection.key)?;
    let collection_config = &registrar.collection_configs[collection_idx];

    require!(
        registrar.is_in_collection_configs(collection.key)?,
        NftVoterError::InvalidCollection
    );

    require!(
        collection.verified,
        NftVoterError::UnverifiedCollection
    );

    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);
    // TODO: add multiplication with number of NFTs used for voting
    voter_weight_record.voter_weight = collection_config.weight as u64;
    voter_weight_record.weight_action = Some(VoterWeightAction::CastVote);
    voter_weight_record.weight_action_target = Some(proposal);

    Ok(())
}
