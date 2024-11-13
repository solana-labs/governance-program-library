use {
    crate::{
        error::*,
        state::*,
        tools::spl_token::{get_spl_token_amount, get_spl_token_owner},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        token_2022::{self, CloseAccount},
        token_interface::TokenInterface,
    },
};

// Remaining accounts must be all the token accounts owned by voter account they want to close,
// they should be writable so that they can be closed and sol rented
// would be transfered to sol_destination
#[derive(Accounts)]
pub struct CloseVoter<'info> {
    pub registrar: Box<Account<'info, Registrar>>,

    #[account(
        mut,
        seeds = [voter.registrar.as_ref(), b"voter".as_ref(), voter_authority.key().as_ref()],
        bump = voter.voter_bump,
        close = sol_destination
    )]
    pub voter: Box<Account<'info, Voter>>,

    #[account(
        mut,
        seeds = [registrar.key().as_ref(), b"voter-weight-record".as_ref(), voter_authority.key().as_ref()],
        bump,
        close = sol_destination
    )]
    pub voter_weight_record: Box<Account<'info, VoterWeightRecord>>,

    pub voter_authority: Signer<'info>,

    /// CHECK: Destination may be any address.
    #[account(mut)]
    pub sol_destination: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}

/// Closes the voter account (Optionally, also token vaults, as part of remaining_accounts),
/// allowing one to retrieve rent exemption SOL.
/// Only accounts with no remaining deposits can be closed.
///
/// Tokens must be withdrawn first to be able to close voter and close the token accounts.
pub fn close_voter<'info>(ctx: Context<'_, '_, '_, 'info, CloseVoter<'info>>) -> Result<()> {
    let voter = &ctx.accounts.voter;
    let voter_authority = &ctx.accounts.voter_authority;
    let amount = voter.deposits.iter().fold(0u64, |sum, d| {
        sum.checked_add(d.amount_deposited_native).unwrap()
    });
    require_eq!(amount, 0, TokenVoterError::VotingTokenNonZero);
    let voter_authority_key = voter_authority.key();
    let voter_seeds = voter_seeds!(voter, voter_authority_key);

    for token_account in ctx.remaining_accounts {
        let token_account_clone = &token_account.clone();
        let token_owner = get_spl_token_owner(token_account_clone)?;
        let token_amount = get_spl_token_amount(token_account_clone)?;

        require_keys_eq!(
            token_owner,
            ctx.accounts.voter.key(),
            TokenVoterError::InvalidAuthority
        );
        require_eq!(token_amount, 0, TokenVoterError::VaultTokenNonZero);

        let cpi_accounts = CloseAccount {
            account: token_account.to_account_info(),
            destination: ctx.accounts.sol_destination.to_account_info(),
            authority: ctx.accounts.voter.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        token_2022::close_account(CpiContext::new_with_signer(
            cpi_program,
            cpi_accounts,
            &[voter_seeds],
        ))?;

        token_account.exit(ctx.program_id)?;
    }

    Ok(())
}
