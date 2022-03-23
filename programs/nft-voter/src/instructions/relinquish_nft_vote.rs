use anchor_lang::prelude::*;
use spl_governance::state::{vote_record,token_owner_record};
use spl_governance::state::{enums::ProposalState, governance, proposal};
use spl_governance_tools::account::dispose_account;
use crate::error::NftVoterError;
use crate::{state::*};

use crate::state::{get_nft_vote_record_data_for_proposal_and_token_owner, Registrar};

/// Disposes NftVoteRecord and recovers the rent from the accounts   
/// It can only be executed when voting on the target Proposal ended or voter withdrew vote from the Proposal
#[derive(Accounts)]
pub struct RelinquishNftVote<'info> {
    /// The NFT voting Registrar
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

    /// CHECK: Owned by spl-governance instance specified in registrar.governance_program_id
    pub governance: UncheckedAccount<'info>,

    /// CHECK: Owned by spl-governance instance specified in registrar.governance_program_id
    pub proposal: UncheckedAccount<'info>,

    /// The token owner who cast the original vote
    #[account(mut)]
    pub governing_token_owner: Signer<'info>,

    /// CHECK: Owned by spl-governance instance specified in registrar.governance_program_id
    pub vote_record: UncheckedAccount<'info>,

    /// CHECK: The beneficiary who receives lamports from the disposed NftVoterRecord accounts can be any account
    #[account(mut)]
    pub beneficiary: UncheckedAccount<'info>,
}

pub fn relinquish_nft_vote(ctx: Context<RelinquishNftVote>) -> Result<()> {
    let registrar = &ctx.accounts.registrar;

    // Ensure the Governance belongs to Registrar.realm and is owned by Registrar.governance_program_id
    let _governance = governance::get_governance_data_for_realm(
        &registrar.governance_program_id,
        &ctx.accounts.governance,
        &registrar.realm,
    )?;

    // Ensure the Proposal belongs to Governance from Registrar.realm and Registrar.governing_token_mint and is owned by Registrar.governance_program_id
    let proposal = proposal::get_proposal_data_for_governance_and_governing_mint(
        &registrar.governance_program_id,
        &ctx.accounts.proposal,
        &ctx.accounts.governance.key(),
        &registrar.governing_token_mint,
    )?;

    // If the Proposal is still in Voting state then we can only Relinquish the NFT votes if the Vote was withdrawn in spl-gov first
    // When vote is withdrawn in spl-gov then VoteRecord is disposed and we have to assert it doesn't exist 
    if proposal.state == ProposalState::Voting {
        let vote_record_info = &ctx.accounts.vote_record.to_account_info();

        // Ensure the given VoteRecord address matches the expected PDA
        let token_owner_record_key = token_owner_record::get_token_owner_record_address(
            &registrar.governance_program_id,
            &registrar.realm,
            &registrar.governing_token_mint,
            &ctx.accounts.governing_token_owner.key());

        let vote_record_key = vote_record::get_vote_record_address(
            &registrar.governance_program_id,
            &ctx.accounts.proposal.key(),
            &token_owner_record_key);
        
        require!(
            vote_record_key == vote_record_info.key(),
            NftVoterError::InvalidVoteRecordForNftVoteRecord
        );

        require!(
            // VoteRecord doesn't exist if data is empty or account_type is 0 when the account was disposed in the same Tx
            vote_record_info.data_is_empty() || vote_record_info.try_borrow_data().unwrap()[0] == 0,
            NftVoterError::VoteRecordMustBeRelinquished
        );
    }

    // Dispose all NftVoteRecords
    for nft_vote_record_info in ctx.remaining_accounts.iter() {
        // Ensure NftVoteRecord is for the given Proposal and TokenOwner
        let _nft_vote_record = get_nft_vote_record_data_for_proposal_and_token_owner(
            nft_vote_record_info,
            &ctx.accounts.proposal.key(),
            &ctx.accounts.governing_token_owner.key(),
        )?;

        dispose_account(nft_vote_record_info, &ctx.accounts.beneficiary);
    }

    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    // Reset VoterWeightRecord and set expiry to expired
    voter_weight_record.voter_weight = 0;
    voter_weight_record.voter_weight_expiry = Some(0);

    Ok(())
}
