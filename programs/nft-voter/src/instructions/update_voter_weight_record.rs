use crate::{
    error::NftLockerError,
    state::{Registrar, VoterWeightRecord},
};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount};
use spl_governance_addin_api::voter_weight::VoterWeightAction;

#[derive(Accounts)]
#[instruction(voter_weight_action:VoterWeightAction)]
pub struct UpdateVoterWeightRecord<'info> {
    pub registrar: Account<'info, Registrar>,

    #[account(
        mut,
        constraint = voter_weight_record.realm == registrar.realm 
        @ NftLockerError::InvalidVoterWeightRecordRealm,

        constraint = voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ NftLockerError::InvalidVoterWeightRecordMint,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,

    pub nft_mint: Account<'info, Mint>,
    pub nft_token: Account<'info, TokenAccount>,
    pub nft_metadata: UncheckedAccount<'info>,
}

pub fn update_voter_weight_record(
    ctx: Context<UpdateVoterWeightRecord>,
    voter_weight_action: VoterWeightAction,
) -> Result<()> {

    require!(
        voter_weight_action != VoterWeightAction::CastVote,
        NftLockerError::CastVoteIsNotAllowed
    );

    // TODO: Validate voter_weight_record.owner owns NFTs
    // TODO: Check collection is verified

    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    // TODO: Pass NFTs and evaluate the same way as Vote does
    voter_weight_record.voter_weight = 10;

    // Record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);

    // Set the action to make it specific and prevent being used for voting
    voter_weight_record.weight_action = Some(voter_weight_action);
    voter_weight_record.weight_action_target = None;

    Ok(())
}
