use {
    crate::{error::*, state::*},
    anchor_lang::prelude::*,
    anchor_lang::solana_program::sysvar::instructions as tx_instructions,
};

/// Creates VoterWeightRecord used by spl-gov
/// This instruction should only be executed once per realm/governing_token_mint/governing_token_owner
/// to create the account
#[derive(Accounts)]
pub struct CreateVoterWeightRecord<'info> {
    // The Registrar the VoterWeightRecord account belongs to
    pub registrar: Box<Account<'info, Registrar>>,

    #[account(
        init,
        seeds = [registrar.key().as_ref(), b"voter".as_ref(), voter_authority.key().as_ref()],
        bump,
        payer = voter_authority,
        space = Voter::get_space(registrar.max_mints),
    )]
    pub voter: Box<Account<'info, Voter>>,

    #[account(
        init,
        seeds = [registrar.key().as_ref(), b"voter-weight-record".as_ref(), voter_authority.key().as_ref()],
        bump,
        payer = voter_authority,
        space = VoterWeightRecord::get_space()
    )]
    pub voter_weight_record: Box<Account<'info, VoterWeightRecord>>,

    #[account(mut)]
    pub voter_authority: Signer<'info>,

    pub system_program: Program<'info, System>,

    /// CHECK: Address constraint is set
    #[account(address = tx_instructions::ID)]
    pub instructions: UncheckedAccount<'info>,
}

pub fn create_voter_weight_record(ctx: Context<CreateVoterWeightRecord>) -> Result<()> {
    // Forbid creating voter accounts from CPI. The goal is to make automation
    // impossible that weakens some of the limitations intentionally imposed on
    // tokens.
    {
        let ixns = ctx.accounts.instructions.to_account_info();
        let current_index = tx_instructions::load_current_index_checked(&ixns)? as usize;
        let current_ixn = tx_instructions::load_instruction_at_checked(current_index, &ixns)?;
        require_keys_eq!(
            current_ixn.program_id,
            *ctx.program_id,
            TokenVoterError::ForbiddenCpi
        );
    }

    let voter = &mut ctx.accounts.voter;
    let voter_authority = &ctx.accounts.voter_authority;
    let registrar = &ctx.accounts.registrar;

    voter.voter_bump = ctx.bumps.voter;
    voter.voter_weight_record_bump = ctx.bumps.voter_weight_record;
    voter.voter_authority = voter_authority.key();
    voter.registrar = registrar.key();
    voter.deposits = DepositEntry::init_deposits(registrar.max_mints as usize);

    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    voter_weight_record.account_discriminator =
        spl_governance_addin_api::voter_weight::VoterWeightRecord::ACCOUNT_DISCRIMINATOR;
    voter_weight_record.realm = registrar.realm.key();
    voter_weight_record.governing_token_mint = registrar.governing_token_mint.key();
    voter_weight_record.governing_token_owner = voter_authority.key();

    // Set expiry to expired
    voter_weight_record.voter_weight_expiry = Some(0);
    Ok(())
}
