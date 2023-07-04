use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateRegistrar<'info> {
    #[account(
        init,
        seeds = [b"registrar".as_ref()],
        bump,
        payer = payer,
        space = Registrar::get_space(1)
    )]
    pub registrar: Account<'info, Registrar>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_registrar(_ctx: Context<CreateRegistrar>) -> Result<()> {
    Ok(())
}
