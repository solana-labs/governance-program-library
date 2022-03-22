use anchor_lang::prelude::*;
use spl_governance::state::{enums::ProposalState, governance, proposal};
use spl_governance_tools::account::dispose_account;

use crate::state::{get_nft_vote_record_data_for_proposal_and_token_owner, Registrar};

/// Disposes NftVoteRecord and recovers the rent from the accounts   
/// It can only be executed when voting on the target Proposal ended or voter withdrew vote from the Proposal
#[derive(Accounts)]
pub struct RelinquishNftVote<'info> {
    /// The NFT voting Registrar
    pub registrar: Account<'info, Registrar>,

    /// CHECK: Owned by spl-governance instance specified in registrar.governance_program_id
    pub governance: UncheckedAccount<'info>,

    /// CHECK: Owned by spl-governance instance specified in registrar.governance_program_id
    pub proposal: UncheckedAccount<'info>,

    /// The token owner who cast the original vote
    #[account(mut)]
    pub governing_token_owner: Signer<'info>,

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

    // If the Proposal is not in Voting state then we can dispose  NftVoteRecords
    if proposal.state != ProposalState::Voting {
        for nft_vote_record_info in ctx.remaining_accounts.iter() {
            // Ensure NftVoteRecord is for the given Proposal and TokenOwner
            let _nft_vote_record = get_nft_vote_record_data_for_proposal_and_token_owner(
                nft_vote_record_info,
                &ctx.accounts.proposal.key(),
                &ctx.accounts.governing_token_owner.key(),
            )?;

            dispose_account(nft_vote_record_info, &ctx.accounts.beneficiary);
        }
    }

    // TODO: Validate registrar vs VoterWeightRecord
    // TODO: Validate governing_token_owner

    // TODO: remove proposal/vote record
    // TODO: relinquish from spl_gov or ensure the proposal is not in voting state

    Ok(())
}
