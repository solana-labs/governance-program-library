use anchor_lang::prelude::*;
use anchor_lang::{Accounts};
use anchor_spl::token::{TokenAccount, Token};
use mpl_token_metadata::state::{Collection, Metadata};
use spl_governance_addin_api::voter_weight::VoterWeightAction;
use std::mem::size_of;
use crate::state::*;
use crate::error::NftVoterError;
use crate::ErrorCode::AccountOwnedByWrongProgram;

#[derive(Accounts)]
#[instruction(realm:Pubkey, governing_token_mint:Pubkey, governing_token_owner: Pubkey)]
pub struct VoteWithNFT<'info> {
    /// Record that nft from nft_account was used to vote on the proposal
    #[account(
        init,
        seeds = [
            registrar.key().as_ref(), 
            b"nft-vote".as_ref(), 
            proposal.key().as_ref(),
            nft_account.mint.as_ref()
            ],
        bump,
        payer = payer,
        space = 8 + size_of::<ProposalNFTVote>()
    )]
    pub proposal_vote_record: Account<'info, ProposalNFTVote>,
    /// The voting registrar
    #[account()]
    pub registrar: Account<'info, Registrar>,
    /// Proposal which is voted on
    pub proposal: UncheckedAccount<'info>,
    /// Account holding the NFT
    #[account(
        constraint = nft_account.amount > 0 @ NftVoterError::InsufficientAmountOnNFTAccount,
        constraint = nft_account.owner == token_program.key() @ AccountOwnedByWrongProgram
    )]
    pub nft_account: Account<'info, TokenAccount>,
    /// Metadata account of the NFT
    pub nft_metadata: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [ b"voter-weight-record".as_ref(),
                realm.as_ref(),
                governing_token_mint.as_ref(),
                governing_token_owner.as_ref()],
        bump,
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
pub fn vote_with_nft(ctx: Context<VoteWithNFT>, _realm:Pubkey, _governing_token_mint:Pubkey, _governing_token_owner: Pubkey) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    let voter_weight_record = &mut ctx.accounts.voter_weight_record;
    let nft_metadata = &ctx.accounts.nft_metadata;
    let metadata = Metadata::from_account_info(nft_metadata)?;
    let collection: Collection = metadata.collection.ok_or(NftVoterError::NotPartOfCollection)?;
    let collection_idx = registrar.collection_config_index(collection.key)?;
    let collection_config = &registrar.collection_configs[collection_idx];
    let proposal = &ctx.accounts.proposal;

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
    voter_weight_record.weight_action_target = Some(proposal.key());

    Ok(())
}
