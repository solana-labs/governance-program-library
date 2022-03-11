use std::{str::FromStr, sync::Arc};

use anchor_lang::prelude::Pubkey;
use solana_program_test::{BanksClientError, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer};
use spl_governance::{
    instruction::{create_realm, create_proposal, create_governance, create_token_owner_record},
    state::{
        enums::{GovernanceAccountType, MintMaxVoteWeightSource, ProposalState, VoteThresholdPercentage, VoteTipping},
        realm::{get_realm_address, RealmConfig, RealmV2}, proposal::{ProposalV2, get_proposal_address}, governance::{get_governance_address}, token_owner_record::get_token_owner_record_address,
    },
};

use super::{program_test_bench::ProgramTestBench, tools::clone_keypair, nft_voter_test::VoterWeightRecordCookie};

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

pub struct ProposalCookie {
    pub address: Pubkey,
    pub account: ProposalV2,
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

    #[allow(dead_code)]
    pub async fn with_proposal(&mut self, realm_cookie: &RealmCookie, voter_weight_record_cookie: &VoterWeightRecordCookie) -> Result<ProposalCookie, BanksClientError> {
        
        let token_account_cookie = self.bench.with_token_account(&realm_cookie.account.community_mint).await?;
        
        let token_owner_record = get_token_owner_record_address(
            &self.program_id, 
            &realm_cookie.address, 
            &realm_cookie.account.community_mint, 
            &self.bench.payer.pubkey()
        );

        let create_tor_ix = create_token_owner_record(
            &self.program_id, 
            &realm_cookie.address, 
            &self.bench.payer.pubkey(),
            &realm_cookie.account.community_mint, 
            &self.bench.payer.pubkey()
        );

        self.bench
            .process_transaction(&[create_tor_ix], None)
            .await?;

        let governance_address = get_governance_address(
            &self.program_id, 
            &realm_cookie.address, 
            &token_account_cookie.address
        );

        let create_governance_ix = create_governance(
            &self.program_id,
            &realm_cookie.address,
            Some(&token_account_cookie.address),
            &token_owner_record,
            &self.bench.payer.pubkey(),
            &realm_cookie.realm_authority.pubkey(),
            Some(voter_weight_record_cookie.voter_weight_record),
            spl_governance::state::governance::GovernanceConfig { 
                vote_threshold_percentage: VoteThresholdPercentage::YesVote(60), 
                min_community_weight_to_create_proposal: 1, 
                min_transaction_hold_up_time: 10, 
                max_voting_time: 600, 
                vote_tipping: VoteTipping::Disabled, 
                proposal_cool_off_time: 0, 
                min_council_weight_to_create_proposal: 0 });

        self.bench
            .process_transaction(&[create_governance_ix], Some(&[&realm_cookie.realm_authority]))
            .await?;

        let proposal_address = get_proposal_address(
            &self.program_id,
            &governance_address, 
            &realm_cookie.account.community_mint,
            &[0]);

        let create_proposal_ix = create_proposal(
            &self.program_id, 
            &governance_address, 
            &self.bench.payer.pubkey(), 
            &self.bench.payer.pubkey(), 
            &self.bench.payer.pubkey(), 
            Some(voter_weight_record_cookie.voter_weight_record), 
            &realm_cookie.address, 
            String::from("Proposal #1"), 
            String::from("Proposal #1 link"), 
            &realm_cookie.account.community_mint, 
            spl_governance::state::proposal::VoteType::SingleChoice, 
            vec!["Yes".to_string(), "No".to_string()], 
            true, 
            1_u32
        );

        self.bench
            .process_transaction(&[create_proposal_ix], None)
            .await?;

        let account = ProposalV2 {
            account_type: GovernanceAccountType::GovernanceV2,
            governing_token_mint: realm_cookie.account.community_mint,
            state: ProposalState::Voting,
            governance: governance_address,
            token_owner_record: token_owner_record,
            signatories_count: 1,
            signatories_signed_off_count: 1,
            vote_type: spl_governance::state::proposal::VoteType::SingleChoice,
            options: vec![],
            deny_vote_weight: Some(1),
            veto_vote_weight: None,
            abstain_vote_weight: None,
            start_voting_at: None,
            draft_at: 1,
            signing_off_at: None,
            voting_at: None,
            voting_at_slot: None,
            voting_completed_at: None,
            executing_at: None,
            closed_at: None,
            execution_flags: spl_governance::state::enums::InstructionExecutionFlags::None,
            max_vote_weight: None,
            max_voting_time: None,
            vote_threshold_percentage: None,
            reserved: [0;64],
            name: String::from("Proposal #1"),
            description_link: String::from("Proposal #1 link"),
            
        };

        Ok(ProposalCookie {
            address: proposal_address,
            account,
        })
    }
}
