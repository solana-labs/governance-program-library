use anchor_lang::prelude::*;

pub mod error;

mod instructions;
use instructions::*;

pub mod state;

pub mod tools;

declare_id!("GRmVtfLq2BPeWs5EDoQoZc787VYkhdkA11k63QM1Xemz");

#[program]
pub mod realm_voter {

    use super::*;
    pub fn create_registrar(
        ctx: Context<CreateRegistrar>,
        max_governance_programs: u8,
    ) -> Result<()> {
        log_version();
        instructions::create_registrar(ctx, max_governance_programs)
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
    pub fn configure_voter_weights(
        ctx: Context<ConfigureVoterWeights>,
        realm_member_vote_weight: u64,
        max_voter_weight: u64,
    ) -> Result<()> {
        log_version();
        instructions::configure_voter_weights(ctx, realm_member_vote_weight, max_voter_weight)
    }

    pub fn configure_governance_program(ctx: Context<ConfigureGovernanceProgram>) -> Result<()> {
        log_version();
        instructions::configure_governance_program(ctx)
    }
}

fn log_version() {
    // TODO: Check if Anchor allows to log it before instruction is deserialized
    msg!("VERSION:{:?}", env!("CARGO_PKG_VERSION"));
}
