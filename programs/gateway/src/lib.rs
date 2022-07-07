use anchor_lang::prelude::*;

pub mod error;

mod instructions;
use instructions::*;

pub mod state;

pub mod tools;

declare_id!("GgathUhdrCWRHowoRKACjgWhYHfxCEdBi5ViqYN6HVxk");

#[program]
pub mod gateway {
    use super::*;
    pub fn create_registrar(
        ctx: Context<CreateRegistrar>,
        use_previous_voter_weight_plugin: bool,
    ) -> Result<()> {
        log_version();
        instructions::create_registrar(ctx, use_previous_voter_weight_plugin)
    }
    pub fn configure_registrar(
        ctx: Context<ConfigureRegistrar>,
        use_previous_voter_weight_plugin: bool,
    ) -> Result<()> {
        log_version();
        instructions::configure_registrar(ctx, use_previous_voter_weight_plugin)
    }
    pub fn create_voter_weight_record(
        ctx: Context<CreateVoterWeightRecord>,
        governing_token_owner: Pubkey,
    ) -> Result<()> {
        log_version();
        instructions::create_voter_weight_record(ctx, governing_token_owner)
    }
    pub fn update_voter_weight_record(ctx: Context<UpdateVoterWeightRecord>) -> Result<()> {
        log_version();
        instructions::update_voter_weight_record(ctx)
    }
}

fn log_version() {
    // TODO: Check if Anchor allows to log it before instruction is deserialized
    msg!("VERSION:{:?}", env!("CARGO_PKG_VERSION"));
}
