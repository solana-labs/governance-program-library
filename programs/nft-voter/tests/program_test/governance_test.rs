use std::{str::FromStr, sync::Arc};

use anchor_lang::prelude::Pubkey;
use solana_program_test::{BanksClientError, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer};
use spl_governance::{
    instruction::create_realm,
    state::{
        enums::{GovernanceAccountType, MintMaxVoteWeightSource},
        realm::{get_realm_address, RealmConfig, RealmV2},
    },
};

use super::{program_test_bench::ProgramTestBench, tools::clone_keypair};

pub struct RealmCookie {
    pub address: Pubkey,
    pub account: RealmV2,
    pub realm_authority: Keypair,
}

impl RealmCookie {
    pub fn get_realm_authority(&self) -> Keypair {
        clone_keypair(&self.realm_authority)
    }
}

pub struct GovernanceTest {
    pub program_id: Pubkey,
    pub bench: Arc<ProgramTestBench>,
    pub next_id: u8,
}

impl GovernanceTest {
    pub fn program_id() -> Pubkey {
        Pubkey::from_str("Governance111111111111111111111111111111111").unwrap()
    }

    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("spl_governance", Self::program_id(), None);
    }

    #[allow(dead_code)]
    pub fn new(bench: Arc<ProgramTestBench>) -> Self {
        GovernanceTest {
            bench,
            program_id: Self::program_id(),
            next_id: 0,
        }
    }

    #[allow(dead_code)]
    pub async fn with_realm(&mut self) -> Result<RealmCookie, BanksClientError> {
        let governing_token_mint = Keypair::new();
        let realm_authority = Keypair::new();

        self.bench
            .create_mint(&governing_token_mint, &realm_authority.pubkey(), None)
            .await?;

        let council_token_mint = Keypair::new();
        self.bench
            .create_mint(&council_token_mint, &realm_authority.pubkey(), None)
            .await?;

        self.next_id += 1;
        let realm_name = format!("Realm #{}", self.next_id).to_string();

        let min_community_weight_to_create_governance = 1;
        let community_mint_max_vote_weight_source = MintMaxVoteWeightSource::FULL_SUPPLY_FRACTION;

        let realm = get_realm_address(&self.program_id, &realm_name);

        let create_realm_ix = create_realm(
            &self.program_id,
            &realm_authority.pubkey(),
            &governing_token_mint.pubkey(),
            &self.bench.payer.pubkey(),
            Some(council_token_mint.pubkey()),
            None,
            None,
            realm_name.clone(),
            min_community_weight_to_create_governance,
            community_mint_max_vote_weight_source.clone(),
        );

        self.bench
            .process_transaction(&[create_realm_ix], None)
            .await?;

        let account = RealmV2 {
            account_type: GovernanceAccountType::RealmV2,
            community_mint: governing_token_mint.pubkey(),

            name: realm_name,
            reserved: [0; 6],
            authority: Some(realm_authority.pubkey()),
            config: RealmConfig {
                council_mint: Some(council_token_mint.pubkey()),
                reserved: [0; 6],
                min_community_weight_to_create_governance,
                community_mint_max_vote_weight_source,
                use_community_voter_weight_addin: false,
                use_max_community_voter_weight_addin: false,
            },
            voting_proposal_count: 0,
            reserved_v2: [0; 128],
        };

        Ok(RealmCookie {
            address: realm,
            account,
            realm_authority,
        })
    }
}
