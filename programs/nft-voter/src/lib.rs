use anchor_lang::prelude::*;

mod instructions;
use instructions::*;

pub mod state;

declare_id!("GnftV5kLjd67tvHpNGyodwWveEKivz3ZWvvE3Z4xi2iw");

#[program]
pub mod nft_voter {

    use super::*;
    pub fn create_registrar(ctx: Context<CreateRegistrar>) -> Result<()> {
        instructions::create_registrar(ctx)
    }
}
