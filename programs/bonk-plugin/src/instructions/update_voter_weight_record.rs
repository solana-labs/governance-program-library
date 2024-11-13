use crate::error::BonkPluginError;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::Accounts;
use gpl_shared::compose::resolve_input_voter_weight;
use gpl_shared::generic_voter_weight::GenericVoterWeight;

#[derive(Accounts)]
#[instruction(
    stake_receipts_count: u8,
    action_target: Pubkey,
    action: VoterWeightAction
)]
pub struct UpdateVoterWeightRecord<'info> {
    pub registrar: Account<'info, Registrar>,

    /// An account that is either of type TokenOwnerRecordV2 or VoterWeightRecord
    /// depending on whether the registrar includes a predecessor or not
    /// CHECK: Checked in the code depending on the registrar
    #[account()]
    pub input_voter_weight: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = voter_weight_record.realm == registrar.realm
        @ BonkPluginError::InvalidVoterWeightRecordRealm,

        constraint = voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ BonkPluginError::InvalidVoterWeightRecordMint,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,

    #[account(
        mut,
        seeds = [
          b"stake-deposit-record".as_ref(),
          voter_weight_record.key().as_ref(),
        ],
        bump = stake_deposit_record.bump,
        realloc = stake_deposit_record.realloc_bytes(
            stake_receipts_count,
            action_target,
            action
        ),
        realloc::payer = payer,
        realloc::zero = false
    )]
    pub stake_deposit_record: Account<'info, StakeDepositRecord>,

    /// CHECK: The account is validated in the instruction
    #[account(
        owner = registrar.governance_program_id
    )]
    voter_token_owner_record: UncheckedAccount<'info>,

    /// CHECK: The account is validated in the instruction
    #[account(
        owner = registrar.governance_program_id
    )]
    governance: UncheckedAccount<'info>,

    /// CHECK: The account is validated in the instruction
    proposal: Option<UncheckedAccount<'info>>,

    pub voter_authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn update_voter_weight_record_handler(
    ctx: Context<UpdateVoterWeightRecord>,
    stake_receipts_count: u8,
    action_target: Pubkey,
    action: VoterWeightAction,
) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    let voter_weight_record = &mut ctx.accounts.voter_weight_record;
    let stake_deposit_record = &mut ctx.accounts.stake_deposit_record;
    let proposal_info = &ctx.accounts.proposal;
    let governance_info = &ctx.accounts.governance;
    let input_voter_weight_account = ctx.accounts.input_voter_weight.to_account_info();
    let clone_record = voter_weight_record.clone();

    let governing_token_owner = resolve_governing_token_owner(
        registrar,
        &ctx.accounts.voter_token_owner_record,
        &ctx.accounts.voter_authority,
        voter_weight_record,
    )?;

    let mut voter_weight = 0u64;

    if voter_weight_record.weight_action_target != Some(action_target)
        || voter_weight_record.weight_action != Some(action)
    {
        stake_deposit_record.deposits = [].to_vec();
    }

    let receipts_len = ctx.remaining_accounts.len() as u8;

    require_eq!(
        stake_receipts_count,
        receipts_len,
        BonkPluginError::ReceiptsCountMismatch
    );

    let input_voter_weight_record =
        resolve_input_voter_weight(&input_voter_weight_account, &clone_record, registrar)?;

    voter_weight = voter_weight
        .checked_add(input_voter_weight_record.get_voter_weight())
        .unwrap();

    let new_deposit_len =
        stake_deposit_record.new_deposit_len(stake_receipts_count, action_target, action);

    stake_deposit_record.deposits_len = new_deposit_len;

    for stake_deposit_receipt_info in ctx.remaining_accounts {
        let vote_weight = resolve_stake_deposit_weight(
            registrar,
            proposal_info,
            governance_info,
            &governing_token_owner,
            stake_deposit_receipt_info,
            &mut stake_deposit_record.deposits,
            action,
            action_target,
        )?;

        voter_weight = voter_weight.checked_add(vote_weight).unwrap();
    }

    if voter_weight_record.weight_action_target == Some(action_target)
        && voter_weight_record.weight_action == Some(action)
    {
        voter_weight_record.voter_weight = voter_weight_record
            .voter_weight
            .checked_sub(stake_deposit_record.previous_voter_weight)
            .unwrap();

        voter_weight_record.voter_weight = voter_weight_record
            .voter_weight
            .checked_add(voter_weight)
            .unwrap();
    } else {
        voter_weight_record.voter_weight = voter_weight;
    }

    // The record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);

    // The record is only valid for casting vote on the given Proposal
    voter_weight_record.weight_action = Some(action);
    voter_weight_record.weight_action_target = Some(action_target);

    stake_deposit_record.weight_action_target = Some(action_target);
    stake_deposit_record.previous_voter_weight = input_voter_weight_record.get_voter_weight();
    stake_deposit_record.weight_action = Some(action);

    Ok(())
}
