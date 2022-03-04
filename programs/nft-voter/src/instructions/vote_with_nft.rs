use anchor_lang::prelude::*;
use anchor_lang::{Accounts};
use anchor_spl::token::{TokenAccount, Token};
use mpl_token_metadata::state::Metadata;
// use spl_governance::state::realm;
use std::mem::size_of;
use crate::error::ErrorCode;
use crate::state::*;
// use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct VoteWithNFT<'info> {
    #[account(
        init_if_needed,
        seeds = [registrar.key().as_ref(), b"nft-vote".as_ref(), proposal.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + size_of::<ProposalNFTVote>()
    )]
    pub proposal_vote_record: Account<'info, ProposalNFTVote>,
    /// The voting registrar
    #[account()]
    pub registrar: Account<'info, Registrar>,
    /// Proposal which is voted on
    pub proposal: UncheckedAccount<'info>,
    /// Account holding the NFT
    #[account(
        constraint = nft_account.amount > 0 @ ErrorCode::InsufficientAmountOnNFTAccount,
        constraint =  nft_account.owner == token_program.key() @ anchor_lang::error::ErrorCode::AccountOwnedByWrongProgram
    )]
    pub nft_account: Account<'info, TokenAccount>,
    /// Metadata account of the NFT
    pub nft_metadata: UncheckedAccount<'info>,
    /// Voter is a signer  
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
}

/// Casts vote with the NFT
/// 
/// User presents his NFT - nft_token_account
/// Program checks if the user has the NFT
/// Program checks if the NFT is part of the verified collection defined in the registrar
/// Program initializes the PDA ['nft-vote', realmId, nftId] which represents persons vote and populates it (with checking if it
/// already exists)
/// Program updates the VoterWeightRecord
///
pub fn vote_with_nft(ctx: Context<VoteWithNFT>) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    let nft_metadata = &ctx.accounts.nft_metadata;
    let metadata = Metadata::from_account_info(nft_metadata)?;

    if 
    metadata.collection.ok_or(anchor_lang::error::ErrorCode::AccountOwnedByWrongProgram)?.key
    !=  registrar.collection.key  
    {
      return Err(ErrorCode::InvalidCollection.into());
    }
    // let registrar = &mut ctx.accounts.registrar;
    // registrar.governance_program_id = ctx.accounts.governance_program_id.key();
    // registrar.realm = ctx.accounts.realm.key();
    // registrar.realm_governing_token_mint = ctx.accounts.realm_governing_token_mint.key();

    // // Verify that "realm_authority" is the expected authority on "realm"
    // // and that the mint matches one of the realm mints too.
    // let realm = realm::get_realm_data_for_governing_token_mint(
    //     &registrar.governance_program_id,
    //     &ctx.accounts.realm.to_account_info(),
    //     &registrar.realm_governing_token_mint,
    // )?;
    // require!(
    //     realm.authority.unwrap() == ctx.accounts.realm_authority.key(),
    //     ErrorCode::InvalidRealmAuthority
    // );

    Ok(())
}
