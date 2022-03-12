use anchor_lang::prelude::*;
use spl_governance_addin_api::voter_weight::VoterWeightAction;

pub mod error;

mod instructions;
use instructions::*;

pub mod governance;
pub mod state;

pub mod tools;

declare_id!("FDfF7jzJDCEkFWNi3is487k8rFPJxFkU821t2pQ1vDr1");

#[program]
pub mod nft_voter {

    use super::*;
    pub fn create_registrar(ctx: Context<CreateRegistrar>, max_collections: u8) -> Result<()> {
        instructions::create_registrar(ctx, max_collections)
    }
    pub fn create_voter_weight_record(
        ctx: Context<CreateVoterWeightRecord>,
        governing_token_owner: Pubkey,
    ) -> Result<()> {
        instructions::create_voter_weight_record(ctx, governing_token_owner)
    }
    pub fn create_max_voter_weight_record(ctx: Context<CreateMaxVoterWeightRecord>) -> Result<()> {
        instructions::create_max_voter_weight_record(ctx)
    }
    pub fn update_voter_weight_record(
        ctx: Context<UpdateVoterWeightRecord>,
        voter_weight_action: VoterWeightAction,
    ) -> Result<()> {
        instructions::update_voter_weight_record(ctx, voter_weight_action)
    }
    pub fn relinquish_vote(
        ctx: Context<RelinquishVote>,
        realm: Pubkey,
        governing_token_mint: Pubkey,
        governing_token_owner: Pubkey,
    ) -> Result<()> {
        instructions::relinquish_vote(ctx, realm, governing_token_mint, governing_token_owner)
    }
    pub fn configure_collection(
        ctx: Context<ConfigureCollection>,
        weight: u16,
        size: u32,
    ) -> Result<()> {
        instructions::configure_collection(ctx, weight, size)
    }

    pub fn vote_with_nft(
        ctx: Context<VoteWithNFT>,
        proposal: Pubkey,
    ) -> Result<()> {
        instructions::vote_with_nft(ctx, proposal)
    }
}
