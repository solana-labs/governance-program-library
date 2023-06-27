use anchor_lang::{prelude::*, solana_program::program::invoke_signed};
use spl_governance::{
    instruction::sign_off_proposal,
    state::{
        governance::get_governance_data_for_realm, proposal::get_proposal_data_for_governance,
    },
};

use crate::{error::TransactionCheckerError, state::TransactionsChecked};

/**
 * Instruction to sign off on a proposal.
 *
 * This instruction can only be called once every transaction on the proposal has been
 * checked.
 */
#[derive(Accounts)]
pub struct Sign<'info> {
    /// CHECK: Realms program
    #[account(executable)]
    realms_program: UncheckedAccount<'info>,

    /// CHECK: Realm account
    #[account(mut)]
    realm: UncheckedAccount<'info>,

    /// CHECK: Governance account
    #[account(mut)]
    governance: UncheckedAccount<'info>,

    /// CHECK: Proposal account
    #[account(mut)]
    proposal: UncheckedAccount<'info>,

    /// CHECK: Signing account
    /**
     * Account corresponding to the requested proposal signer.
     *
     * This is the address which will sign the proposal.
     */
    #[account(
        seeds = [ b"signatory".as_ref() ],
        bump,
    )]
    signatory: AccountInfo<'info>,

    /// CHECK: Signatory record account
    #[account(mut)]
    signatory_record: AccountInfo<'info>,

    #[account(
        seeds = [
            b"transactions-checked".as_ref(),
            proposal.key().as_ref(),
        ],
        bump,
    )]
    transactions_checked: Account<'info, TransactionsChecked>,
}

pub fn sign(ctx: Context<Sign>) -> Result<()> {
    get_governance_data_for_realm(
        ctx.accounts.realms_program.key,
        &ctx.accounts.governance,
        ctx.accounts.realm.key,
    )?;
    let proposal = get_proposal_data_for_governance(
        ctx.accounts.realms_program.key,
        &ctx.accounts.proposal,
        ctx.accounts.governance.key,
    )?;

    ctx.accounts
        .transactions_checked
        .assert_full_checked(&proposal)?;

    require!(
        !ctx.accounts.transactions_checked.reject,
        TransactionCheckerError::ProposalRejected
    );

    let ixn = sign_off_proposal(
        ctx.accounts.realms_program.key,
        ctx.accounts.realm.key,
        ctx.accounts.governance.key,
        ctx.accounts.proposal.key,
        ctx.accounts.signatory.key,
        None,
    );

    invoke_signed(
        &ixn,
        &[
            ctx.accounts.realm.to_account_info(),
            ctx.accounts.governance.to_account_info(),
            ctx.accounts.proposal.to_account_info(),
            ctx.accounts.signatory.to_account_info(),
            ctx.accounts.signatory_record.to_account_info(),
        ],
        &[&[b"signatory".as_ref(), &[ctx.bumps["signatory"]]]],
    )?;

    Ok(())
}

pub fn signing_authority_address() -> Pubkey {
    Pubkey::find_program_address(&[b"signatory".as_ref()], &crate::id()).0
}
