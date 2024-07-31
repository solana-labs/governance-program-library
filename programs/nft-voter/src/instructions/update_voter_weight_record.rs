use crate::error::NftVoterError;
use crate::state::*;
use anchor_lang::prelude::*;
use itertools::Itertools;

/// Updates VoterWeightRecord to evaluate governance power for non voting use cases: CreateProposal, CreateGovernance etc...
/// This instruction updates VoterWeightRecord which is valid for the current Slot and the given target action only
/// and hance the instruction has to be executed inside the same transaction as the corresponding spl-gov instruction
///
/// Note: UpdateVoterWeight is not cumulative the same way as CastNftVote and hence voter_weight for non voting scenarios
/// can only be used with max 5 NFTs due to Solana transaction size limit
/// It could be supported in future version by introducing bookkeeping accounts to track the NFTs
/// which were already used to calculate the total weight
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

    match voter_weight_action {
        // voter_weight for CastVote action can't be evaluated using this instruction
        VoterWeightAction::CastVote => return err!(NftVoterError::CastVoteIsNotAllowed),
        VoterWeightAction::CommentProposal
        | VoterWeightAction::CreateGovernance
        | VoterWeightAction::CreateProposal
        | VoterWeightAction::SignOffProposal => {}
    }

    let mut voter_weight = 0u64;

    // Ensure all nfts are unique
    let mut unique_nft_mints = vec![];

    for (nft_info, nft_metadata_info) in ctx.remaining_accounts.iter().tuples() {
        let (nft_vote_weight, _) = resolve_nft_vote_weight_and_mint(
            registrar,
            governing_token_owner,
            nft_info,
            nft_metadata_info,
            &mut unique_nft_mints,
        )?;

        voter_weight = voter_weight.checked_add(nft_vote_weight).unwrap();
    }

    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    voter_weight_record.voter_weight = voter_weight;

    // Record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);

    // Set the action to make it specific and prevent being used for voting
    voter_weight_record.weight_action = Some(voter_weight_action);
    voter_weight_record.weight_action_target = None;

    Ok(())
}
