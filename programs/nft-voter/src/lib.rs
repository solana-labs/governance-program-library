use anchor_lang::prelude::*;

declare_id!("GnftV5kLjd67tvHpNGyodwWveEKivz3ZWvvE3Z4xi2iw");

/// Registrar which stores NFT voting configuration for the given Realm
#[account]
#[derive(Debug, PartialEq)]
pub struct Registrar {}

pub const DISCRIMINATOR_SIZE: usize = 8;

#[derive(Accounts)]
pub struct CreateRegistrar<'info> {
    #[account(
        init,
        seeds = [b"registrar".as_ref()],
        bump,
        payer = payer,
        space = DISCRIMINATOR_SIZE
    )]
    pub registrar: Account<'info, Registrar>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[program]
pub mod nft_voter {

    use super::*;
    pub fn create_registrar(_ctx: Context<CreateRegistrar>) -> Result<()> {
        Ok(())
    }
}
