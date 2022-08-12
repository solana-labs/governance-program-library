use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use solana_program::{entrypoint::ProgramResult, program::invoke};
use spl_governance::state::realm;

use crate::{id, state::nft_voter_token_owner_record::NftVoterTokenOwnerRecord};

/// Deposits tokens into the holding account for a given NFT to boost its voting power
#[derive(Accounts)]
pub struct DepositGovernanceTokens<'info> {
    #[account(
        init_if_needed,
        seeds = [b"nft-voter-token-owner-record".as_ref(),
            realm.key().as_ref(),
            realm_governing_token_mint.key().as_ref(),
            nft_mint.key().as_ref(),
            governing_token_owner.key().as_ref()
        ],
        bump,
        payer = governing_token_owner,
        space = NftVoterTokenOwnerRecord::get_space()
    )]
    pub token_owner_record: Account<'info, NftVoterTokenOwnerRecord>,

    #[account(
        seeds = [ b"nft-power-holding-account".as_ref(),
                realm.key().as_ref(),
                realm_governing_token_mint.key().as_ref(),
                nft_mint.key().as_ref()],
        bump,
        token::mint = realm_governing_token_mint,
        token::authority = governance_program_id
    )]
    pub holding_account_info: Account<'info, TokenAccount>,

    /// The program id of the spl-governance program the realm belongs to
    /// CHECK: Can be any instance of spl-governance and it's not known at the compilation time
    #[account(executable)]
    pub governance_program_id: UncheckedAccount<'info>,

    /// CHECK: Owned by spl-governance instance specified in governance_program_id
    #[account(owner = governance_program_id.key())]
    pub realm: UncheckedAccount<'info>,

    /// Either the realm community mint or the council mint.
    // TODO revert when you can figure out how to correctly set up/verify the owning program
    pub realm_governing_token_mint: Account<'info, Mint>,
    // pub realm_governing_token_mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub governing_token_owner: Signer<'info>,

    //TODO add constraint that the nft is the one configured for a realm collection
    pub nft_mint: Account<'info, Mint>,

    #[account(constraint = governing_token_source_account.owner == governing_token_owner.key())]
    pub governing_token_source_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

/// Deposits tokens into the holding account for a given NFT to boost its voting power
pub fn deposit_governance_tokens(ctx: Context<DepositGovernanceTokens>, amount: u64) -> Result<()> {
    // Deserialize the Realm to validate it
    let _realm = realm::get_realm_data_for_governing_token_mint(
        &ctx.accounts.governance_program_id.key(),
        &ctx.accounts.realm,
        &ctx.accounts.realm_governing_token_mint.key(),
    )?;

    let token_owner_record = &mut ctx.accounts.token_owner_record;
    token_owner_record.set_inner(NftVoterTokenOwnerRecord {
        realm: ctx.accounts.realm.key(),
        governing_token_mint: ctx.accounts.realm_governing_token_mint.key(),
        nft_mint: ctx.accounts.nft_mint.key(),
        governing_token_owner: ctx.accounts.governing_token_owner.key(),
        governing_token_deposit_amount: token_owner_record
            .governing_token_deposit_amount
            .checked_add(amount)
            .unwrap(),
    });

    spl_governance::tools::spl_token::transfer_spl_tokens_signed(
        &ctx.accounts
            .governing_token_source_account
            .to_account_info(),
        &ctx.accounts.holding_account_info.to_account_info(),
        &ctx.accounts.governing_token_owner,
        //TODO WTF is this?
        &[ctx
            .accounts
            .governing_token_owner
            .signer_key()
            .unwrap()
            .as_ref()],
        &ctx.accounts.governance_program_id.key(),
        amount,
        &ctx.accounts.realm_governing_token_mint.to_account_info(),
    )
    .unwrap();
    // spl_governance::tools::spl_token::transfer_spl_tokens(
    //     &ctx.accounts
    //         .governing_token_source_account
    //         .to_account_info(),
    //     &ctx.accounts.holding_account_info.to_account_info(),
    //     &ctx.accounts.governing_token_owner.to_account_info(),
    //     amount,
    //     &ctx.accounts.realm_governing_token_mint.to_account_info(),
    // )
    // .unwrap();

    let token_owner_record = &mut ctx.accounts.token_owner_record;

    token_owner_record.governing_token_deposit_amount = token_owner_record
        .governing_token_deposit_amount
        .checked_add(amount)
        .unwrap();

    Ok(())
}

fn transfer_spl_tokens<'a>(
    source_info: &AccountInfo<'a>,
    destination_info: &AccountInfo<'a>,
    authority_info: &AccountInfo<'a>,
    amount: u64,
    spl_token_info: &AccountInfo<'a>,
) -> ProgramResult {
    let transfer_instruction = spl_token::instruction::transfer(
        &spl_token::id(),
        source_info.key,
        destination_info.key,
        authority_info.key,
        &[authority_info.key],
        amount,
    )
    .unwrap();

    invoke(
        &transfer_instruction,
        &[
            spl_token_info.clone(),
            authority_info.clone(),
            source_info.clone(),
            destination_info.clone(),
        ],
    )?;

    Ok(())
}
