use anchor_lang::prelude::*;
use spl_governance::state::{
    governance::get_governance_data_for_realm, proposal::get_proposal_data_for_governance,
    proposal_transaction::get_proposal_transaction_data_for_proposal,
};
use spl_token::instruction::TokenInstruction;

use crate::{error::TransactionCheckerError, state::TransactionsChecked};

fn assert_transaction_matches(
    transaction: &Pubkey,
    option: u8,
    transaction_number: u16,
    proposal: &Pubkey,
    program_id: &Pubkey,
) -> Result<()> {
    let expected_address = Pubkey::find_program_address(
        &[
            spl_governance::PROGRAM_AUTHORITY_SEED,
            proposal.as_ref(),
            &option.to_le_bytes(),
            &transaction_number.to_le_bytes(),
        ],
        program_id,
    )
    .0;

    require_keys_eq!(
        expected_address,
        *transaction,
        TransactionCheckerError::WrongTransaction
    );

    Ok(())
}

/**
 * Instruction to check a transaction.
 *
 * This instruction must be called for every transaction attached to a proposal.
 * To minimize complexity, transactions must be checked in order for each proposal option.
 * That is, within each option, transactions must be checked in order, but options can
 * be freely interleaved.
 */
#[derive(Accounts)]
#[instruction(option: u8)]
pub struct CheckTransaction<'info> {
    /// CHECK: Realms program
    #[account(executable)]
    realms_program: UncheckedAccount<'info>,

    /// CHECK: Realm account
    realm: UncheckedAccount<'info>,

    /// CHECK: Governance account
    governance: UncheckedAccount<'info>,

    /// CHECK: Proposal account
    proposal: UncheckedAccount<'info>,

    /**
     * State account to keep track of which transactions have been cleared.
     */
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + std::mem::size_of::<TransactionsChecked>(),
        seeds = [
            b"transactions-checked".as_ref(),
            proposal.key().as_ref(),
        ],
        bump,
    )]
    transactions_checked: Account<'info, TransactionsChecked>,

    /// CHECK: Transaction account
    transaction: UncheckedAccount<'info>,

    #[account(mut)]
    payer: Signer<'info>,

    system_program: Program<'info, System>,
}

pub fn check_transaction(ctx: Context<CheckTransaction>, option: u8) -> Result<()> {
    if ctx.accounts.transactions_checked.payer == Pubkey::default() {
        ctx.accounts.transactions_checked.payer = ctx.accounts.payer.key();
    }

    assert_transaction_matches(
        ctx.accounts.transaction.key,
        option,
        ctx.accounts
            .transactions_checked
            .next_transaction_to_check(option),
        ctx.accounts.proposal.key,
        ctx.accounts.realms_program.key,
    )?;

    let governance = get_governance_data_for_realm(
        ctx.accounts.realms_program.key,
        &ctx.accounts.governance,
        ctx.accounts.realm.key,
    )?;
    get_proposal_data_for_governance(
        ctx.accounts.realms_program.key,
        &ctx.accounts.proposal,
        ctx.accounts.governance.key,
    )?;

    let transaction = get_proposal_transaction_data_for_proposal(
        ctx.accounts.realms_program.key,
        &ctx.accounts.transaction,
        ctx.accounts.proposal.key,
    )?;

    require_eq!(
        transaction.option_index,
        option,
        TransactionCheckerError::TransactionWrongOption
    );
    require_eq!(
        transaction.transaction_index,
        ctx.accounts
            .transactions_checked
            .next_transaction_to_check(option),
        TransactionCheckerError::WrongTransaction
    );

    for instruction in transaction.instructions.iter() {
        if instruction.program_id == spl_token::ID {
            if let Ok(instruction_data) = TokenInstruction::unpack(&instruction.data) {
                if matches!(
                    instruction_data,
                    TokenInstruction::Transfer { .. } | TokenInstruction::TransferChecked { .. }
                ) {
                    let from_account = instruction.accounts[0].pubkey;
                    if from_account == governance.governed_account {
                        ctx.accounts.transactions_checked.reject = true;
                    }
                }
            }
        }
    }

    ctx.accounts.transactions_checked.mark_as_checked(option);

    Ok(())
}
