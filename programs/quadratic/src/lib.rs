use anchor_lang::prelude::*;

pub mod error;

mod instructions;
use instructions::*;

pub mod state;
mod util;

use state::*;

declare_id!("quadCSapU8nTdLg73KHDnmdxKnJQsh7GUbu5tZfnRRr");

#[program]
pub mod quadratic {
    use super::*;
    pub fn create_registrar(
        ctx: Context<CreateRegistrar>,
        coefficients: QuadraticCoefficients,
        use_previous_voter_weight_plugin: bool,
    ) -> Result<()> {
        log_version();
        instructions::create_registrar(ctx, coefficients, use_previous_voter_weight_plugin)
    }
    pub fn configure_registrar(
        ctx: Context<ConfigureRegistrar>,
        coefficients: QuadraticCoefficients,
        use_previous_voter_weight_plugin: bool,
    ) -> Result<()> {
        log_version();
        instructions::configure_registrar(ctx, coefficients, use_previous_voter_weight_plugin)
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
