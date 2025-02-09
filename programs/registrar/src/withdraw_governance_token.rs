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
pub struct WithdrawGovernanceToken<'info> {
    #[account(mut)]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,
    #[account(mut)]
    pub voter_token_account: AccountInfo<'info>,
    pub governance_token_mint: AccountInfo<'info>,
    pub registrar_config: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    pub spl_governance_program: AccountInfo<'info>,
    pub authority: AccountInfo<'info>,
}

pub fn withdraw_governance_token(
    ctx: Context<WithdrawGovernanceToken>,
    amount: u64,
) -> ProgramResult {
    let voter_weight_record = &mut ctx.accounts.voter_weight_record;
    let clock = Clock::get()?;

    // Check if the withdrawal is allowed by the spl-governance program
    // ...

    // Check if the current slot is the same as the last deposit or withdrawal slot
    if voter_weight_record.last_deposit_or_withdrawal_slot == clock.slot {
        return Err(RegistrarError::InvalidOperation.into());
    }

    let registrar_config = RegistrarConfig::unpack_from_slice(&ctx.accounts.registrar_config.data.borrow())?;

    // Check if the token is an accepted governance token
    let token_mint = ctx.accounts.governance_token_mint.key();
    if !registrar_config.accepted_tokens.contains(&token_mint) {
        return Err(RegistrarError::InvalidArgument.into());
    }

    let weight_decrease = amount * registrar_config.weights[registrar_config.accepted_tokens.iter().position(|&token| token == token_mint).ok_or(RegistrarError::InvalidArgument)?];
    if voter_weight_record.weight < weight_decrease {
        return Err(RegistrarError::InsufficientFunds.into());
    }

    // Transfer tokens from the governance program's account to the voter's account
    transfer_tokens(
        &ctx.accounts.governance_token_mint, // Replace with the governance program's token account
        &ctx.accounts.voter_token_account,
        amount,
        &ctx.accounts.token_program,
        &ctx.accounts.authority,
    )?;

    // Update the VoterWeightRecord account
    voter_weight_record.weight = voter_weight_record.weight.checked_sub(weight_decrease).ok_or(RegistrarError::Overflow)?;
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
