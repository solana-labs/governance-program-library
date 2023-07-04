use crate::state::*;
use anchor_lang::prelude::*;

/// Creates Registrar storing NFT governance configuration for spl-gov Realm
/// This instruction should only be executed once per realm/governing_token_mint to create the account
#[derive(Accounts)]

pub struct CreateRegistrar<'info> {
    /// The NFT voting Registrar
    /// There can only be a single registrar per governance Realm and governing mint of the Realm
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

/// Creates a new Registrar which stores NFT voting configuration for given Realm
///
/// To use the registrar, call ConfigureCollection to register NFT collections that may be
/// used for governance
///
/// max_collections is used allocate account size for the maximum number of governing NFT collections
/// Note: Once Solana runtime supports account resizing the max value won't be required
pub fn create_registrar(_ctx: Context<CreateRegistrar>) -> Result<()> {
    Ok(())
}
