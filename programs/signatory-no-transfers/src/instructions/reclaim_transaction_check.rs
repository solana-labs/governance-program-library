use anchor_lang::prelude::*;

use crate::{error::TransactionCheckerError, state::TransactionsChecked};

pub fn reclaim_transaction_check(ctx: Context<ReclaimTransactionCheck>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.transaction_check.payer,
        ctx.accounts.payer.key(),
        TransactionCheckerError::WrongBeneficiary
    );

    Ok(())
}

/**
 * Instruction to reclaim TransactionsChecked lamports.
 *
 * May only be called on behalf of the original payer.
 */
#[derive(Accounts)]
pub struct ReclaimTransactionCheck<'info> {
    #[account(mut, close = payer)]
    transaction_check: Account<'info, TransactionsChecked>,

    /// CHECK: Account to receive lamports
    #[account(mut)]
    payer: AccountInfo<'info>,
}
