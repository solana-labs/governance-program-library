use {
    crate::{error::*, state::*, tools::spl_token::transfer_spl_tokens_signed_checked, ID},
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    pub registrar: Box<Account<'info, Registrar>>,

    // Checking the PDA address it just an extra precaution,
    // the other constraints must be exhaustive
    #[account(
        mut,
        seeds = [registrar.key().as_ref(), b"voter".as_ref(), voter_authority.key().as_ref()],
        bump = voter.voter_bump,
        has_one = registrar,
    )]
    pub voter: Box<Account<'info, Voter>>,

    #[account(mut)]
    pub voter_authority: Signer<'info>,

    /// The token_owner_record for the voter_authority. This is needed
    /// to be able to forbid withdraws while the voter is engaged with
    /// a vote or has an open proposal.
    ///
    /// CHECK: token_owner_record is validated in the instruction:
    /// - owned by registrar.governance_program_id
    /// - for the registrar.realm
    /// - for the registrar.realm_governing_token_mint
    /// - governing_token_owner is voter_authority
    pub token_owner_record: UncheckedAccount<'info>,

    /// Tokens of this mint must be included in the Voting Mint Configs
    pub mint: InterfaceAccount<'info, Mint>,

    /// Withdraws must update the voter weight record, to prevent a stale
    /// record being used to vote after the withdraw.
    #[account(
        mut,
        seeds = [registrar.key().as_ref(), b"voter-weight-record".as_ref(), voter_authority.key().as_ref()],
        bump = voter.voter_weight_record_bump,
        constraint = voter_weight_record.realm == registrar.realm,
        constraint = voter_weight_record.governing_token_owner == voter_authority.key(),
        constraint = voter_weight_record.governing_token_mint == registrar.governing_token_mint,
    )]
    pub voter_weight_record: Box<Account<'info, VoterWeightRecord>>,

    #[account(
        mut,
        associated_token::authority = voter,
        associated_token::mint = mint,
        associated_token::token_program = token_program,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        init_if_needed,
        associated_token::authority = voter_authority,
        associated_token::mint = mint,
        associated_token::token_program = token_program,
        payer = voter_authority
    )]
    pub destination: Box<InterfaceAccount<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Interface<'info, TokenInterface>,
}

/// Withdraws tokens from a deposit entry.
///
/// `deposit_entry_index`: The deposit entry to withdraw from.
/// `amount`: is in units of the native currency being withdrawn.
pub fn withdraw<'info>(
    ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>,
    deposit_entry_index: u8,
    amount: u64,
) -> Result<()> {
    let voter_authority_key = &ctx.accounts.voter_authority.key();
    let voter = &mut ctx.accounts.voter;

    {
        transfer_spl_tokens_signed_checked(
            &ctx.accounts.vault.to_account_info(),
            &ctx.accounts.destination.to_account_info(),
            &voter.to_account_info(),
            voter_seeds_no_seeds!(voter, voter_authority_key),
            &ID,
            amount,
            &ctx.accounts.token_program.to_account_info(),
            &ctx.accounts.mint.to_account_info(),
            ctx.remaining_accounts,
        )?;
    }

    // Load the accounts.
    let registrar = &ctx.accounts.registrar;

    // Get the exchange rate for the token being withdrawn,
    // fails if mint does not exist in the registrar config.
    let mint_idx = registrar.voting_mint_config_index(ctx.accounts.destination.mint)?;

    // Governance may forbid withdraws, for example when engaged in a vote.
    // Not applicable for tokens that don't contribute to voting power.
    let token_owner_record = voter.load_token_owner_record(
        &ctx.accounts.token_owner_record.to_account_info(),
        registrar,
        voter_authority_key,
    )?;
    token_owner_record.assert_can_withdraw_governing_tokens()?;

    let deposit_entry = voter.active_deposit_mut(deposit_entry_index)?;

    require_eq!(
        mint_idx,
        deposit_entry.voting_mint_config_idx as usize,
        TokenVoterError::MintNotFound
    );

    // Bookkeeping for withdrawn funds.
    require_gte!(
        deposit_entry.amount_deposited_native,
        amount,
        TokenVoterError::TokenAmountOverflow
    );

    deposit_entry.amount_deposited_native = deposit_entry
        .amount_deposited_native
        .checked_sub(amount)
        .unwrap();

    if deposit_entry.amount_deposited_native == 0 {
        deposit_entry.is_used = false;
    }

    let clock = Clock::get()?;
    let current_slot_hash = clock.slot;

    // Using the slot hash to enforce withdrawal and depositing to be not
    // in the same slot to prevent flash loan style governance attacks
    require_neq!(
        current_slot_hash,
        deposit_entry.deposit_slot_hash,
        TokenVoterError::CannotWithdraw
    );

    // Update the voter weight record
    let voter_weight_record = &mut ctx.accounts.voter_weight_record;
    voter_weight_record.voter_weight = voter.weight(registrar)?;
    // Voter Weight Expiry is always set to None after a deposit
    // since no other action other than deposit and withdraw could invalidate it
    voter_weight_record.voter_weight_expiry = None;

    Ok(())
}
