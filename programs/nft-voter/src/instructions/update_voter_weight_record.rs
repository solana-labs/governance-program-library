use crate::{
    error::NftVoterError,
    state::{Registrar, VoterWeightRecord}, tools::token_metadata::{ get_token_metadata_for_mint},
};
use anchor_lang::prelude::*;

use spl_governance::tools::spl_token::{get_spl_token_owner, get_spl_token_mint};
use spl_governance_addin_api::voter_weight::VoterWeightAction;

#[derive(Accounts)]
#[instruction(voter_weight_action:VoterWeightAction)]
pub struct UpdateVoterWeightRecord<'info> {
    pub registrar: Account<'info, Registrar>,

    #[account(
        mut,
        constraint = voter_weight_record.realm == registrar.realm 
        @ NftVoterError::InvalidVoterWeightRecordRealm,

        constraint = voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ NftVoterError::InvalidVoterWeightRecordMint,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,

    pub nft_token: UncheckedAccount<'info>,

    pub nft_metadata: UncheckedAccount<'info>,
}

pub fn update_voter_weight_record(
    ctx: Context<UpdateVoterWeightRecord>,
    voter_weight_action: VoterWeightAction,
) -> Result<()> {

    // CastVote can't be evaluated using this instruction 
    require!(
        voter_weight_action != VoterWeightAction::CastVote,
        NftVoterError::CastVoteIsNotAllowed
    );

    let nft_token_owner = get_spl_token_owner(&ctx.accounts.nft_token.to_account_info())?;

    // voter_weight_record.governing_token_owner must be the owner of the NFT
    require!(
        nft_token_owner == ctx.accounts.voter_weight_record.governing_token_owner,
        NftVoterError::VoterDoesNotOwnNft
    );

    let nft_token_mint = get_spl_token_mint(&ctx.accounts.nft_token.to_account_info())?;
    let nft_metadata = get_token_metadata_for_mint(&ctx.accounts.nft_metadata,nft_token_mint)?;

    // The NFT must have a collection and the collection must be verified 
    let collection = nft_metadata.collection.unwrap();

    require!(
        collection.verified,
        NftVoterError::CollectionMustBeVerified
    );

    let registrar = &mut ctx.accounts.registrar;

    let collection_config = registrar                                                   
        .collection_configs
        .iter()
        .find(|cc| cc.collection == collection.key)
        .ok_or(NftVoterError::CollectionNotFound)?;


    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    voter_weight_record.voter_weight = collection_config.weight as u64;

    // Record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);

    // Set the action to make it specific and prevent being used for voting
    voter_weight_record.weight_action = Some(voter_weight_action);
    voter_weight_record.weight_action_target = None;

    Ok(())
}
