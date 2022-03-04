use anchor_lang::prelude::*;

mod error;
// use error::*;

mod instructions;
use instructions::*;

mod governance;
mod state;

// use state::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_voter {

    use super::*;
    pub fn create_registrar(ctx: Context<CreateRegistrar>) -> Result<()> {
        instructions::create_registrar(ctx)
    }
}
