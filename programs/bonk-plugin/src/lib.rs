use anchor_lang::{prelude::*, solana_program::pubkey};

mod instructions;
use instructions::*;
use state::VoterWeightAction;

pub mod error;
pub mod state;
pub mod utils;

declare_id!("7yJT49ajgYyuhWYzQMzwEt9u9Zbbt7r8Ft2wq1bhhfyy");

#[constant]
pub const SPL_TOKEN_STAKING_PROGRAM_ID: Pubkey =
    pubkey!("STAKEkKzbdeKkqzKpLkNQD3SUuLgshDKCD7U8duxAbB");

#[program]
pub mod bonk_plugin {
    use super::*;

    pub fn create_registrar(ctx: Context<CreateRegistrar>) -> Result<()> {
        log_version();
        create_registrar_handler(ctx)
    }

    pub fn create_voter_weight_record(
        ctx: Context<CreateVoterWeightRecord>,
        governing_token_owner: Pubkey,
    ) -> Result<()> {
        log_version();
        create_voter_weight_record_handler(ctx, governing_token_owner)
    }

    pub fn update_voter_weight_record(
        ctx: Context<UpdateVoterWeightRecord>,
        stake_receipts_count: u8,
        action_target: Pubkey,
        action: VoterWeightAction,
    ) -> Result<()> {
        log_version();
        update_voter_weight_record_handler(ctx, stake_receipts_count, action_target, action)
    }
}

fn log_version() {
    msg!("VERSION:{:?}", env!("CARGO_PKG_VERSION"));
}
