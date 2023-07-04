use std::sync::Arc;

use anchor_lang::prelude::{AccountMeta, Pubkey};

use gpl_nft_voter::state::max_voter_weight_record::{
    get_max_voter_weight_record_address, MaxVoterWeightRecord,
};
use gpl_nft_voter::state::*;

use spl_governance::instruction::cast_vote;
use spl_governance::state::vote_record::{self, Vote, VoteChoice};

use gpl_nft_voter::state::{
    get_nft_vote_record_address, get_registrar_address, CollectionConfig, NftVoteRecord, Registrar,
};

use solana_program_test::{BanksClientError, ProgramTest};
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::program_test::governance_test::GovernanceTest;
use crate::program_test::program_test_bench::ProgramTestBench;

use crate::program_test::governance_test::{ProposalCookie, TokenOwnerRecordCookie};
use crate::program_test::program_test_bench::WalletCookie;
use crate::program_test::token_metadata_test::{NftCookie, TokenMetadataTest};
use crate::program_test::tools::NopOverride;

pub struct MaxVoterWeightRecordCookie {
    pub address: Pubkey,
    pub account: MaxVoterWeightRecord,
}

pub struct CollectionConfigCookie {
    pub collection_config: CollectionConfig,
}

pub struct ConfigureCollectionArgs {
    pub weight: u64,
    pub size: u32,
}

impl Default for ConfigureCollectionArgs {
    fn default() -> Self {
        Self { weight: 1, size: 3 }
    }
}

#[derive(Debug, PartialEq)]
pub struct NftVoteRecordCookie {
    pub address: Pubkey,
    pub account: NftVoteRecord,
}

pub struct CastNftVoteArgs {
    pub cast_spl_gov_vote: bool,
}

impl Default for CastNftVoteArgs {
    fn default() -> Self {
        Self {
            cast_spl_gov_vote: true,
        }
    }
}

pub struct NftVoterTest {
    pub program_id: Pubkey,
    pub bench: Arc<ProgramTestBench>,
    pub governance: GovernanceTest,
    pub token_metadata: TokenMetadataTest,
}

impl NftVoterTest {
    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("gpl_nft_voter", gpl_nft_voter::id(), None);
    }
}
