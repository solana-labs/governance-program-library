use anchor_lang::prelude::*;

pub mod error;

mod instructions;
use instructions::*;

pub mod state;

pub mod tools;

declare_id!("GSqds6KYQf5tXEHwrDszu6AqkVXinFCKDwUfTLzp1jEH");

#[program]
pub mod squads_voter {

    use super::*;
    pub fn create_registrar(ctx: Context<CreateRegistrar>, max_squads: u8) -> Result<()> {
        log_version();
        instructions::create_registrar(ctx, max_squads)
    }
    pub fn create_voter_weight_record(
        ctx: Context<CreateVoterWeightRecord>,
        governing_token_owner: Pubkey,
    ) -> Result<()> {
        log_version();
        instructions::create_voter_weight_record(ctx, governing_token_owner)
    }
    pub fn create_max_voter_weight_record(ctx: Context<CreateMaxVoterWeightRecord>) -> Result<()> {
        log_version();
        instructions::create_max_voter_weight_record(ctx)
    }
    pub fn update_voter_weight_record(ctx: Context<UpdateVoterWeightRecord>) -> Result<()> {
        log_version();
        instructions::update_voter_weight_record(ctx)
    }
    pub fn update_max_voter_weight_record(ctx: Context<UpdateMaxVoterWeightRecord>) -> Result<()> {
        log_version();
        instructions::update_max_voter_weight_record(ctx)
    }

    pub fn configure_squad(ctx: Context<ConfigureSquad>, weight: u64) -> Result<()> {
        log_version();
        instructions::configure_squad(ctx, weight)
    }
}

fn log_version() {
    // TODO: Check if Anchor allows to log it before instruction is deserialized
    msg!("VERSION:{:?}", env!("CARGO_PKG_VERSION"));
}
