use crate::error::NftVoterError;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;

/// Casts NFT vote. The NFTs used for voting are tracked using NftVoteRecord accounts
/// This instruction updates VoterWeightRecord which is valid for the current Slot and the target Proposal only
/// and hance the instruction has to be executed inside the same transaction as spl-gov.CastVote
///
/// CastNftVote instruction and NftVoteRecord are not directional. They don't record vote choice (ex Yes/No)
/// VoteChoice is recorded by spl-gov in VoteRecord and this CastNftVote only tracks voting NFTs
///
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
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,

    /// The token owner who casts the vote
    #[account(
        address = voter_weight_record.governing_token_owner @ NftVoterError::InvalidTokenOwnerForVoterWeightRecord
    )]
    pub governing_token_owner: Signer<'info>,

    /// The account which pays for the transaction
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Casts vote with the given NFTs
pub fn cast_nft_vote<'a, 'b, 'c, 'info>(
    ctx: Context<'a, 'b, 'c, 'info, CastNftVote<'info>>,
    proposal: Pubkey,
) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    let governing_token_owner = &ctx.accounts.governing_token_owner.key();

    // Record voting NFTs and get total weight
    let voter_weight = register_nft_vote_records(
        &registrar,
        governing_token_owner,
        &proposal,
        ctx.remaining_accounts,
        &ctx.accounts.payer.to_account_info(),
        &ctx.accounts.system_program.to_account_info(),
    )?;

    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    voter_weight_record.voter_weight = voter_weight;

    // The record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);

    // The record is only valid for casting vote on the given Proposal
    voter_weight_record.weight_action = Some(VoterWeightAction::CastVote);
    voter_weight_record.weight_action_target = Some(proposal);

    Ok(())
}
