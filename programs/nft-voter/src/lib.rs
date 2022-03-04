use anchor_lang::prelude::*;

mod error;
// use error::*;

mod instructions;
use instructions::*;

mod state;

// use state::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_voter {

    use super::*;
    pub fn create_registrar(ctx: Context<CreateRegistrar>) -> Result<()> {
        instructions::create_registrar(ctx)
    }

    pub fn configure_collection(ctx: Context<ConfigureCollection>, multiplier: u64) -> Result<()> {
        instructions::configure_collection(ctx, multiplier)
    }

    pub fn vote_with_nft(ctx: Context<VoteWithNFT>) -> Result<()> {
        instructions::vote_with_nft(ctx)
    }

}
