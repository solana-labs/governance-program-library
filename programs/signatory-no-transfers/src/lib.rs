pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod no_transfers_signatory {
    use super::*;

    pub fn check_transaction(ctx: Context<CheckTransaction>, option: u8) -> Result<()> {
        instructions::check_transaction(ctx, option)
    }

    pub fn reclaim_transaction_check(ctx: Context<ReclaimTransactionCheck>) -> Result<()> {
        instructions::reclaim_transaction_check(ctx)
    }

    pub fn sign(ctx: Context<Sign>) -> Result<()> {
        instructions::sign(ctx)
    }
}
