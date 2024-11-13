use {
    crate::{
        error::*,
        state::*,
        tools::spl_token::{get_current_mint_fee, transfer_checked_spl_tokens},
    },
    anchor_lang::{prelude::*, solana_program::sysvar::instructions as tx_instructions},
    anchor_spl::{
        associated_token::AssociatedToken,
        token_interface::{Mint, TokenAccount, TokenInterface},
    },
    spl_governance::state::token_owner_record,
};

/// Deposits and creates vault based on the tokens configured in mint_configs
#[derive(Accounts)]
pub struct Deposit<'info> {
    pub registrar: Box<Account<'info, Registrar>>,

    #[account(
        mut,
        seeds = [registrar.key().as_ref(), b"voter".as_ref(), deposit_authority.key().as_ref()],
        bump = voter.voter_bump,
        has_one = registrar)]
    pub voter: Box<Account<'info, Voter>>,

    #[account(
        init_if_needed,
        associated_token::authority = voter,
        associated_token::mint = mint,
        associated_token::token_program = token_program,
        payer = deposit_authority
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [registrar.key().as_ref(), b"voter-weight-record".as_ref(), deposit_authority.key().as_ref()],
        bump = voter.voter_weight_record_bump,
        constraint = voter_weight_record.realm == registrar.realm
        @ TokenVoterError::InvalidVoterWeightRecordRealm,

        constraint = voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ TokenVoterError::InvalidVoterWeightRecordMint,
    )]
    pub voter_weight_record: Box<Account<'info, VoterWeightRecord>>,

    /// TokenOwnerRecord for any of the configured spl-governance instances
    /// CHECK: Owned by any of the spl-governance instances specified in registrar.governance_program_configs
    pub token_owner_record: UncheckedAccount<'info>,

    /// Tokens of this mint must be included in the Voting Mint Configs
    pub mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mut,
        associated_token::authority = deposit_authority,
        associated_token::mint = mint,
        associated_token::token_program = token_program,
    )]
    pub deposit_token: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut)]
    pub deposit_authority: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK: Address constraint is set
    #[account(address = tx_instructions::ID)]
    pub instructions: UncheckedAccount<'info>,
}

/// Adds tokens to a deposit entry.
///
/// Tokens will be transfered from deposit_token to vault using the deposit_authority.
///
/// The deposit entry must have been initialized with create_deposit_entry.
///
/// `deposit_entry_index`: Index of the deposit entry.
/// `amount`: Number of native tokens to transfer.
pub fn deposit<'info>(
    ctx: Context<'_, '_, '_, 'info, Deposit<'info>>,
    deposit_entry_index: u8,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }

    // Deposit tokens into the vault and increase the amount too.
    // fail early if amount is insufficient.
    {
        transfer_checked_spl_tokens(
            &ctx.accounts.deposit_token.to_account_info(),
            &ctx.accounts.vault.to_account_info(),
            &ctx.accounts.deposit_authority.to_account_info(),
            amount,
            &ctx.accounts.token_program.to_account_info(),
            &ctx.accounts.mint.to_account_info(),
            ctx.remaining_accounts,
        )?;
    }

    let registrar = &ctx.accounts.registrar;
    let voter = &mut ctx.accounts.voter;

    let deposit_entry = match voter.active_deposit_mut(deposit_entry_index) {
        Ok(d_entry) => Some(d_entry),
        Err(_) => None,
    };

    let deposit_amount = amount
        .checked_sub(get_current_mint_fee(
            &ctx.accounts.mint.to_account_info(),
            amount,
        )?)
        .unwrap();

    // Get the exchange rate entry associated with this deposit,
    // fails if registrar.voting_mint_configs does not exist.
    let mint_idx = registrar.voting_mint_config_index(ctx.accounts.deposit_token.mint)?;

    require_eq!(
        mint_idx,
        deposit_entry_index as usize,
        TokenVoterError::OutOfBoundsDepositEntryIndex
    );

    let clock = Clock::get()?;
    let current_slot_hash = clock.slot;

    match deposit_entry {
        Some(d_entry) => {
            require_eq!(
                mint_idx,
                d_entry.voting_mint_config_idx as usize,
                TokenVoterError::MintIndexMismatch
            );

            d_entry.amount_deposited_native = d_entry
                .amount_deposited_native
                .checked_add(deposit_amount)
                .unwrap();

            // Deposit is only valid as of the current slot
            d_entry.deposit_slot_hash = current_slot_hash;
            d_entry.is_used = true;
        }
        None => {
            let deposit_entry = DepositEntry {
                deposit_slot_hash: current_slot_hash,
                amount_deposited_native: deposit_amount,
                voting_mint_config_idx: mint_idx as u8,
                is_used: true,
                reserved: [0; 38],
            };
            voter.deposits[mint_idx] = deposit_entry;
        }
    }

    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    let governance_program_id = &ctx.accounts.registrar.governance_program_id;

    let token_owner_record = token_owner_record::get_token_owner_record_data(
        governance_program_id,
        &ctx.accounts.token_owner_record,
    )?;

    // Ensure VoterWeightRecord and TokenOwnerRecord are for the same governing_token_owner
    require_eq!(
        token_owner_record.governing_token_owner,
        voter_weight_record.governing_token_owner,
        TokenVoterError::GoverningTokenOwnerMustMatch
    );

    // Setup voter_weight
    voter_weight_record.voter_weight = voter.weight(registrar)?;

    // Voter Weight Expiry is always set to None after a deposit
    // since no other action other than deposit and withdraw could invalidate it
    voter_weight_record.voter_weight_expiry = None;

    // Set action and target to None to indicate the weight is valid for any action and target
    voter_weight_record.weight_action = None;
    voter_weight_record.weight_action_target = None;
    Ok(())
}
