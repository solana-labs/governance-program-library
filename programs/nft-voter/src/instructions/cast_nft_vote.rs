use anchor_lang::prelude::*;
use anchor_lang::{Accounts};
use anchor_spl::token::{TokenAccount};
use mpl_token_metadata::state::{Collection};
use spl_governance::tools::spl_token::{get_spl_token_mint, get_spl_token_owner};
use spl_governance_addin_api::voter_weight::VoterWeightAction;
use spl_governance_tools::account::create_and_serialize_account_signed;
use crate::{state::*, id};
use crate::error::NftVoterError;
use crate::tools::token_metadata::get_token_metadata_for_mint;

#[derive(Accounts)]
#[instruction(proposal: Pubkey)]
pub struct CastNftVote<'info> {
    /// The voting registrar
    pub registrar: Account<'info, Registrar>,

    /// Record that nft from nft_account was used to vote on the proposal
    #[account(mut)]
    pub nft_vote_record: UncheckedAccount<'info>,

    /// Account holding the NFT
    #[account(
        constraint = nft_token.amount > 0 @ NftVoterError::InsufficientAmountOnNFTAccount,
    )]
    pub nft_token: Account<'info, TokenAccount>,

    /// Metadata account of the NFT
    /// CHECK: token-metadata
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

}

/// Casts vote with the NFT
pub fn cast_nft_vote(ctx: Context<CastNftVote>, proposal:Pubkey) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    let voter_weight_record = &mut ctx.accounts.voter_weight_record;
    
    let nft_token = &ctx.accounts.nft_token;
    let nft_owner = get_spl_token_owner(&nft_token.to_account_info())?;
    
     require!(
        nft_owner == voter_weight_record.governing_token_owner,
        NftVoterError::VoterDoesNotOwnNft
    );

    let nft_mint = get_spl_token_mint(&nft_token.to_account_info())?;
    let nft_metadata = get_token_metadata_for_mint(&ctx.accounts.nft_metadata,&nft_mint)?;

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



    let nft_vote_record = NftVoteRecord {
        account_discriminator: NftVoteRecord::ACCOUNT_DISCRIMINATOR,
        proposal,
        nft_mint,
        governing_token_owner: nft_owner,
    };

    // Anchor doesn't natively support dynamic account creation using remaining_accounts
    // and we have to take it on manual drive
    let rent = Rent::get()?;

    create_and_serialize_account_signed(
        &ctx.accounts.payer.to_account_info(),
        &ctx.accounts.nft_vote_record.to_account_info(),
        &nft_vote_record,
        &get_nft_vote_record_seeds(&proposal,&nft_mint),
        &id(),
        &ctx.accounts.system_program.to_account_info(),
        &rent)?;

    Ok(())
}
