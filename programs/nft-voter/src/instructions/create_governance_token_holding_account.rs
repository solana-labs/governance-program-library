use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use spl_governance::state::realm;

/// Creates a governance token holding account for a given NFT to boost its voting power
/// This instruction should only be executed once per realm/governing_token_mint/nft
/// to create the account
#[derive(Accounts)]
pub struct CreateGovernanceTokenHoldingAccount<'info> {
    //TODO add docs
    #[account(
        init,
        seeds = [ b"nft-power-holding-account".as_ref(),
                realm.key().as_ref(),
                realm_governing_token_mint.key().as_ref(),
                nft_mint.key().as_ref()],
        bump,
        payer = payer,
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
    pub payer: Signer<'info>,

    //TODO add constraint that the nft is the one configured for a realm collection
    pub nft_mint: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

/// Deposits tokens into the holding account for a given NFT to boost its voting power
pub fn create_governance_token_holding_account(
    ctx: Context<CreateGovernanceTokenHoldingAccount>,
) -> Result<()> {
    // Deserialize the Realm to validate it
    let _realm = realm::get_realm_data_for_governing_token_mint(
        &ctx.accounts.governance_program_id.key(),
        &ctx.accounts.realm,
        &ctx.accounts.realm_governing_token_mint.key(),
    )?;

    Ok(())
}
