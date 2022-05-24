use anchor_lang::prelude::*;

pub mod error;

mod instructions;
use instructions::*;

pub mod state;

pub mod tools;

use crate::state::*;

declare_id!("Ggat9fNP9SsZEfxGJqbdZUasBGppksaZUBfL92ntrFEp");

#[program]
pub mod gateway {

    use crate::state::VoterWeightAction;

    use super::*;
    pub fn create_registrar(ctx: Context<CreateRegistrar>) -> Result<()> {
        log_version();
        instructions::create_registrar(ctx)
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
    pub fn update_voter_weight_record(
        ctx: Context<UpdateVoterWeightRecord>,
        voter_weight_action: VoterWeightAction,
    ) -> Result<()> {
        log_version();
        instructions::update_voter_weight_record(ctx, voter_weight_action)
    }
    pub fn cast_vote<'a, 'b, 'c, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, CastVote<'info>>,
        proposal: Pubkey,
    ) -> Result<()> {
        log_version();
        instructions::cast_vote(ctx, proposal)
    }
}

fn log_version() {
    // TODO: Check if Anchor allows to log it before instruction is deserialized
    msg!("VERSION:{:?}", env!("CARGO_PKG_VERSION"));
}
