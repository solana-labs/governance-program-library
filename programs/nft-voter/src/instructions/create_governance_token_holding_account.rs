use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{
    error::NftVoterError,
    state::Registrar,
    tools::{
        governance::NFT_POWER_HOLDING_ACCOUNT_SEED_PREFIX,
        token_metadata::get_token_metadata_for_mint,
    },
};

/// Creates a governance token holding account for a given NFT to boost its voting power
/// This instruction should only be executed once per realm/governing_token_mint/nft
/// to create the account
#[derive(Accounts)]
pub struct CreateGovernanceTokenHoldingAccount<'info> {
    /// Associated fungible token account for the NFT being backed
    #[account(
        init,
        seeds = [ &NFT_POWER_HOLDING_ACCOUNT_SEED_PREFIX,
                registrar.realm.as_ref(),
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

    pub registrar: Account<'info, Registrar>,

    /// Either the realm community mint or the council mint.
    pub realm_governing_token_mint: Account<'info, Mint>,

    // pub realm_governing_token_mint: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Mint of the NFT for which the holding account is being created
    pub nft_mint: Account<'info, Mint>,

    /// Metadata of the NFT for which the holding account is being created. The
    /// NFT must have a verified collection configured for the realm.
    pub nft_metadata: UncheckedAccount<'info>,

    /// Associated token program that will own the holding account
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// Token program of the governance token mint
    pub token_program: Program<'info, Token>,

    /// System program required for creating the holding account
    pub system_program: Program<'info, System>,

    /// Rent required for creating the holding account
    pub rent: Sysvar<'info, Rent>,
}

/// Deposits tokens into the holding account for a given NFT to boost its voting power
pub fn create_governance_token_holding_account(
    ctx: Context<CreateGovernanceTokenHoldingAccount>,
) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    let nft_mint = &ctx.accounts.nft_mint;
    let nft_metadata = get_token_metadata_for_mint(
        &ctx.accounts.nft_metadata.to_account_info(),
        &nft_mint.key(),
    )?;

    // The NFT must have a collection and the collection must be verified
    let nft_collection = nft_metadata
        .collection
        .ok_or(NftVoterError::MissingMetadataCollection)?;

    require!(
        nft_collection.verified,
        NftVoterError::CollectionMustBeVerified
    );

    require!(
        registrar
            .collection_configs
            .iter()
            .map(|c| c.collection.key())
            .any(|c| c.key() == nft_collection.key),
        NftVoterError::InvalidNftCollection
    );

    Ok(())
}
