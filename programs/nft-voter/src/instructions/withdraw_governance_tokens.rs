use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use itertools::Itertools;
use spl_governance::state::{proposal::ProposalV2, realm, enums::ProposalState};
use spl_governance_tools::account::get_account_data;
use std::collections::HashSet;


use crate::{
    error::NftVoterError, state::{
        get_nft_vote_record_address,
        delegator_token_owner_record::DelegatorTokenOwnerRecord},
    tools::governance::NFT_POWER_HOLDING_ACCOUNT_SEED_PREFIX,
};

/// Withdraws tokens from the holding account for a given NFT
#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct WithdrawGovernanceTokens<'info> {
    #[account(
        mut,
        seeds = [b"delegator-token-owner-record".as_ref(),
            realm.key().as_ref(),
            realm_governing_token_mint.key().as_ref(),
            nft_mint.key().as_ref(),
            governing_token_owner.key().as_ref()
        ],
        bump,
        constraint = token_owner_record.governing_token_deposit_amount >= amount
    )]
    pub token_owner_record: Account<'info, DelegatorTokenOwnerRecord>,

    #[account(
        mut,
        seeds = [ &NFT_POWER_HOLDING_ACCOUNT_SEED_PREFIX,
                realm.key().as_ref(),
                realm_governing_token_mint.key().as_ref(),
                nft_mint.key().as_ref()],
        bump,
        token::mint = realm_governing_token_mint,
        token::authority = governance_program_id,
        constraint = holding_account_info.amount >= amount
    )]
    pub holding_account_info: Account<'info, TokenAccount>,

    /// The program id of the spl-governance program the realm belongs to
    /// CHECK: Can be any instance of spl-governance and it's not known at the compilation time
    #[account(executable)]
    pub governance_program_id: UncheckedAccount<'info>,

    /// CHECK: Owned by spl-governance instance specified in governance_program_id
    #[account(owner = governance_program_id.key())]
    pub realm: UncheckedAccount<'info>,

    /// Either the realm community mint or the council mint.
    pub realm_governing_token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub governing_token_owner: Signer<'info>,

    //TODO add constraint that the nft is the one configured for a realm collection
    pub nft_mint: Account<'info, Mint>,

    #[account(mut, constraint = governing_token_source_account.owner == governing_token_owner.key())]
    pub governing_token_source_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/// Withdraws tokens from the holding account for a given NFT to boost its voting power
pub fn withdraw_governance_tokens(
    ctx: Context<WithdrawGovernanceTokens>,
    amount: u64,
) -> Result<()> {
    let realm = realm::get_realm_data_for_governing_token_mint(
        &ctx.accounts.governance_program_id.key(),
        &ctx.accounts.realm,
        &ctx.accounts.realm_governing_token_mint.key(),
    )?;


    require_eq!(
        realm.voting_proposal_count * 2,
        ctx.remaining_accounts.len() as u16,
        NftVoterError::InvalidProposalsForWithdraw,
    );

    let mut proposal_uniqueness_tracker: HashSet<Pubkey> = HashSet::new();
    for (open_proposal, nft_voting_record) in ctx.remaining_accounts.iter().tuples() {
        let op = get_account_data::<ProposalV2>(&ctx.accounts.governance_program_id.key(), open_proposal)?;

        require!(
            !proposal_uniqueness_tracker.insert(open_proposal.key()),
            NftVoterError::DuplicateProposals
        );

        
        let verified_nvr_address = get_nft_vote_record_address(
            &open_proposal.key(),
            &ctx.accounts.nft_mint.key(),
        );

        // TODO: add an explicit error message
        require_keys_eq!(
            verified_nvr_address,
            nft_voting_record.key(),
        );

        if op.state != ProposalState::Voting {
            return err!(NftVoterError::ProposalNotVoting);
        }

        require!(
            nft_voting_record.data_is_empty(),
            NftVoterError::NftAlreadyVoted
        );


    }


    spl_governance::tools::spl_token::transfer_spl_tokens_signed(
        &ctx.accounts.holding_account_info.to_account_info(),
        &ctx.accounts
            .governing_token_source_account
            .to_account_info(),
        &ctx.accounts.realm.to_account_info(),
        &spl_governance::state::realm::get_realm_address_seeds(&realm.name),
        &ctx.accounts.governance_program_id.key(),
        amount,
        &ctx.accounts.realm_governing_token_mint.to_account_info(),
    )
    .unwrap();

    let token_owner_record = &mut ctx.accounts.token_owner_record;
    token_owner_record.governing_token_deposit_amount = token_owner_record
        .governing_token_deposit_amount
        .checked_sub(amount)
        .unwrap();

    Ok(())
}
