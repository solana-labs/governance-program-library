use crate::{
    error::NftVoterError,
    state::{Registrar, VoterWeightRecord, resolve_nft_vote_weight_and_mint},
};
use anchor_lang::prelude::*;
use itertools::Itertools;


use crate::governance::VoterWeightAction;


#[derive(Accounts)]
#[instruction(voter_weight_action:VoterWeightAction)]
pub struct UpdateVoterWeightRecord<'info> {
    /// The NFT voting Registrar
    pub registrar: Account<'info, Registrar>,

    #[account(
        mut,
        constraint = voter_weight_record.realm == registrar.realm 
        @ NftVoterError::InvalidVoterWeightRecordRealm,

        constraint = voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ NftVoterError::InvalidVoterWeightRecordMint,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,
}

pub fn update_voter_weight_record(
    ctx: Context<UpdateVoterWeightRecord>,
    voter_weight_action: VoterWeightAction,
) -> Result<()> {

    let registrar = &ctx.accounts.registrar;
    let governing_token_owner = &ctx.accounts.voter_weight_record.governing_token_owner;

    // CastVote can't be evaluated using this instruction 
    require!(
        voter_weight_action != VoterWeightAction::CastVote,
        NftVoterError::CastVoteIsNotAllowed
    );

    let mut voter_weight = 0u64;

    // Ensure all nfts are unique
    let mut unique_nft_mints = vec![];

    for (nft_info, nft_metadata_info) in ctx.remaining_accounts.iter().tuples() {
        let (nft_vote_weight, _) = resolve_nft_vote_weight_and_mint(
            registrar,
            governing_token_owner,
            nft_info,
            nft_metadata_info,
            &mut unique_nft_mints)?;
            
        voter_weight = voter_weight.checked_add(nft_vote_weight as u64).unwrap();
    };

    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    voter_weight_record.voter_weight = voter_weight;

    // Record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);

    // Set the action to make it specific and prevent being used for voting
    voter_weight_record.weight_action = Some(voter_weight_action.into());
    voter_weight_record.weight_action_target = None;

    Ok(())
}
