use crate::{
    error::RegistrarError,
    registrar_config::RegistrarConfig,
    voter_weight_record::VoterWeightRecord,
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey, sysvar::clock::Clock,
};
use spl_token::instruction::transfer;

#[derive(Accounts)]
pub struct DepositGovernanceToken<'info> {
    #[account(mut)]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,
    #[account(mut)]
    pub voter_token_account: AccountInfo<'info>,
    #[account(mut)]
    pub governance_token_mint: AccountInfo<'info>,
    pub registrar_config: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
}

pub fn deposit_governance_token(
    ctx: Context<DepositGovernanceToken>,
    amount: u64,
) -> ProgramResult {
    let registrar_config = RegistrarConfig::unpack_from_slice(&ctx.accounts.registrar_config.data.borrow())?;

    // Check if the token is an accepted governance token
    let token_mint = ctx.accounts.governance_token_mint.key();
    if !registrar_config.accepted_tokens.contains(&token_mint) {
        return Err(RegistrarError::InvalidArgument.into());
    }

    // Transfer tokens from the voter's account to the governance program's account
    transfer_tokens(
        &ctx.accounts.voter_token_account,
        &ctx.accounts.governance_token_mint, // Replace with the governance program's token account
        amount,
        &ctx.accounts.token_program,
        &ctx.accounts.authority,
    )?;

    // Update the VoterWeightRecord account
    let voter_weight_record = &mut ctx.accounts.voter_weight_record;
    let clock = Clock::get()?;

    if voter_weight_record.last_deposit_or_withdrawal_slot == clock.slot {
        return Err(RegistrarError::InvalidOperation.into());
    }

    let weight_increase = amount * registrar_config.weights[registrar_config.accepted_tokens.iter().position(|&token| token == token_mint).ok_or(RegistrarError::InvalidArgument)?];
    voter_weight_record.weight = voter_weight_record.weight.checked_add(weight_increase).ok_or(RegistrarError::Overflow)?;
    voter_weight_record.last_deposit_or_withdrawal_slot = clock.slot;
    voter_weight_record.serialize(&mut ctx.accounts.voter_weight_record.data.borrow_mut()[..])?;

    // Update the MaxVoterWeightRecord account
    // ...

    Ok(())
}

fn transfer_tokens(
    source_account: &AccountInfo,
    destination_account: &AccountInfo,
    amount: u64,
    token_program: &AccountInfo,
    authority: &AccountInfo,
) -> ProgramResult {
    let transfer_instruction = transfer(
        token_program.key,
        source_account.key,
        destination_account.key,
        authority.key,
        &[],
        amount,
    )?;

    invoke(
        &transfer_instruction,
        &[
            source_account.clone(),
            destination_account.clone(),
            authority.clone(),
            token_program.clone(),
        ],
    )?;

    Ok(())
}
