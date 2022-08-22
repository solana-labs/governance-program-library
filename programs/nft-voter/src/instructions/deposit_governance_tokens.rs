use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use spl_governance::state::realm;

use crate::{
    state::delegator_token_owner_record::DelegatorTokenOwnerRecord,
    tools::governance::NFT_POWER_HOLDING_ACCOUNT_SEED_PREFIX,
};

/// Deposits tokens into the holding account for a given NFT to boost its voting power
#[derive(Accounts)]
pub struct DepositGovernanceTokens<'info> {
    /// Record tracking what amount of the tokens in the holding
    /// account belong to this delegator
    #[account(
        init_if_needed,
        seeds = [&DelegatorTokenOwnerRecord::SEED_PREFIX,
            realm.key().as_ref(),
            realm_governing_token_mint.key().as_ref(),
            nft_mint.key().as_ref(),
            governing_token_owner.key().as_ref()
        ],
        bump,
        payer = governing_token_owner,
        space = DelegatorTokenOwnerRecord::SPACE
    )]
    pub token_owner_record: Account<'info, DelegatorTokenOwnerRecord>,

    /// Associated fungible token account for the NFT being backed
    #[account(
        mut,
        seeds = [ &NFT_POWER_HOLDING_ACCOUNT_SEED_PREFIX,
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
    pub realm_governing_token_mint: Account<'info, Mint>,

    /// Delegator, payer, and wallet that should receive the deposited tokens
    /// upon withdrawal
    #[account(mut)]
    pub governing_token_owner: Signer<'info>,

    /// Mint of the NFT being backed.
    // We dont need to check that the NFT has a collection or that the collection
    // is one configured for the realm, because a) this already happened when
    // creating the holding account, and b) we have a constraint here that the
    // holding account's seeds include this mint
    pub nft_mint: Account<'info, Mint>,

    /// Associated token account owned by governing_token_owner from which
    /// tokens are being withdrawn for the deposit
    #[account(mut, constraint = governing_token_source_account.owner == governing_token_owner.key())]
    pub governing_token_source_account: Account<'info, TokenAccount>,

    /// System program required for creating the DelegatorTokenOwnerRecord
    pub system_program: Program<'info, System>,

    /// Token program required for withdrawing (mutating) the source and holding accounts
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

    spl_governance::tools::spl_token::transfer_spl_tokens(
        &ctx.accounts
            .governing_token_source_account
            .to_account_info(),
        &ctx.accounts.holding_account_info.to_account_info(),
        &ctx.accounts.governing_token_owner.to_account_info(),
        amount,
        &ctx.accounts.realm_governing_token_mint.to_account_info(),
    )
    .unwrap();

    let token_owner_record = &mut ctx.accounts.token_owner_record;
    token_owner_record.set_inner(DelegatorTokenOwnerRecord {
        realm: ctx.accounts.realm.key(),
        governing_token_mint: ctx.accounts.realm_governing_token_mint.key(),
        nft_mint: ctx.accounts.nft_mint.key(),
        governing_token_owner: ctx.accounts.governing_token_owner.key(),
        governing_token_deposit_amount: token_owner_record
            .governing_token_deposit_amount
            .checked_add(amount)
            .unwrap(),
    });

    Ok(())
}
