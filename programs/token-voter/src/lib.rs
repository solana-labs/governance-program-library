use anchor_lang::prelude::*;

pub mod error;

mod instructions;
use instructions::*;

pub mod state;

mod governance;
pub mod tools;

#[macro_use]
extern crate static_assertions;

declare_id!("3JhBg9bSPcfWGFa3t8LH7ooVtrjm45yCkHpxYXMXstUM");

#[program]
pub mod token_voter {
    use super::*;

    pub fn create_registrar(ctx: Context<CreateRegistrar>, max_mints: u8) -> Result<()> {
        log_version();
        instructions::create_registrar(ctx, max_mints)
    }

    pub fn resize_registrar(ctx: Context<ResizeRegistrar>, max_mints: u8) -> Result<()> {
        log_version();
        instructions::resize_registrar(ctx, max_mints)
    }

    pub fn create_voter_weight_record(ctx: Context<CreateVoterWeightRecord>) -> Result<()> {
        log_version();
        instructions::create_voter_weight_record(ctx)
    }

    pub fn create_max_voter_weight_record(ctx: Context<CreateMaxVoterWeightRecord>) -> Result<()> {
        log_version();
        instructions::create_max_voter_weight_record(ctx)
    }

    pub fn configure_mint_config(
        ctx: Context<ConfigureVotingMintConfig>,
        digit_shift: i8,
    ) -> Result<()> {
        log_version();
        instructions::configure_mint_config(ctx, digit_shift)
    }

    pub fn deposit<'info>(
        ctx: Context<'_, '_, '_, 'info, Deposit<'info>>,
        deposit_entry_index: u8,
        amount: u64,
    ) -> Result<()> {
        log_version();
        instructions::deposit(ctx, deposit_entry_index, amount)
    }

    pub fn withdraw<'info>(
        ctx: Context<'_, '_, '_, 'info, Withdraw<'info>>,
        deposit_entry_index: u8,
        amount: u64,
    ) -> Result<()> {
        log_version();
        instructions::withdraw(ctx, deposit_entry_index, amount)
    }

    pub fn close_voter<'info>(ctx: Context<'_, '_, '_, 'info, CloseVoter<'info>>) -> Result<()> {
        log_version();
        instructions::close_voter(ctx)
    }
}

fn log_version() {
    msg!("VERSION:{:?}", env!("CARGO_PKG_VERSION"));
}
