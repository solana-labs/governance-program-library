use crate::{error::RegistrarError, registrar_config::RegistrarConfig};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey, system_program,
};

#[derive(Accounts)]
pub struct InitializeRegistrar<'info> {
    #[account(init, payer = payer, space = RegistrarConfig::LEN)]
    pub registrar_config: Account<'info, RegistrarConfig>,
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_registrar(
    ctx: Context<InitializeRegistrar>,
    accepted_tokens: Vec<Pubkey>,
    weights: Vec<u64>,
) -> ProgramResult {
    if accepted_tokens.len() != weights.len() {
        return Err(RegistrarError::InvalidArgument.into());
    }

    if accepted_tokens.len() > MAX_ACCEPTED_TOKENS {
        return Err(RegistrarError::InvalidArgument.into());
    }

    let registrar_info = &mut ctx.accounts.registrar_config;
    let config = RegistrarConfig {
        accepted_tokens,
        weights,
    };
    config.pack_into_slice(&mut registrar_info.data.borrow_mut());
    Ok(())
}
