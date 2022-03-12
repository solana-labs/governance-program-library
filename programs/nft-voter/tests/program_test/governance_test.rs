use std::{str::FromStr, sync::Arc};

use anchor_lang::prelude::Pubkey;
use solana_program_test::{BanksClientError, ProgramTest};
use solana_sdk::{signature::Keypair, signer::Signer};
use spl_governance::{
    instruction::{
        create_governance, create_proposal, create_realm, create_token_owner_record,
        deposit_governing_tokens,
    },
    state::{
        enums::{
            GovernanceAccountType, MintMaxVoteWeightSource, ProposalState, VoteThresholdPercentage,
            VoteTipping,
        },
        governance::get_governance_address,
        proposal::{get_proposal_address, ProposalV2},
        realm::{get_realm_address, RealmConfig, RealmV2},
        token_owner_record::get_token_owner_record_address,
    },
};

use super::{
    program_test_bench::{MintCookie, ProgramTestBench},
    tools::clone_keypair,
};

pub struct RealmCookie {
    pub address: Pubkey,
    pub account: RealmV2,
    pub realm_authority: Keypair,
    pub community_mint_cookie: MintCookie,
    pub council_mint_cookie: Option<MintCookie>,
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
        let realm_authority = Keypair::new();

        let community_mint_cookie = self.bench.with_mint().await?;
        let council_mint_cookie = self.bench.with_mint().await?;

        self.next_id += 1;
        let realm_name = format!("Realm #{}", self.next_id).to_string();

        let min_community_weight_to_create_governance = 1;
        let community_mint_max_vote_weight_source = MintMaxVoteWeightSource::FULL_SUPPLY_FRACTION;

        let realm = get_realm_address(&self.program_id, &realm_name);

        let create_realm_ix = create_realm(
            &self.program_id,
            &realm_authority.pubkey(),
            &community_mint_cookie.address,
            &self.bench.payer.pubkey(),
            Some(council_mint_cookie.address),
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
            community_mint: community_mint_cookie.address,

            name: realm_name,
            reserved: [0; 6],
            authority: Some(realm_authority.pubkey()),
            config: RealmConfig {
                council_mint: Some(council_mint_cookie.address),
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
            community_mint_cookie,
            council_mint_cookie: Some(council_mint_cookie),
        })
    }

    #[allow(dead_code)]
    pub async fn with_proposal(
        &mut self,
        realm_cookie: &RealmCookie,
    ) -> Result<ProposalCookie, BanksClientError> {
        let token_account_cookie = self
            .bench
            .with_token_account(&realm_cookie.account.community_mint)
            .await?;

        let token_owner = self.bench.payer.pubkey();
        let council_mint_cookie = realm_cookie.council_mint_cookie.as_ref().unwrap();
        let governing_token_mint = council_mint_cookie.address;

        let governing_token_account_cookie = self
            .bench
            .with_tokens(council_mint_cookie, &token_owner, 1)
            .await?;

        let token_owner_record = get_token_owner_record_address(
            &self.program_id,
            &realm_cookie.address,
            &governing_token_mint,
            &token_owner,
        );

        let create_tor_ix = create_token_owner_record(
            &self.program_id,
            &realm_cookie.address,
            &self.bench.payer.pubkey(),
            &governing_token_mint,
            &self.bench.payer.pubkey(),
        );

        self.bench
            .process_transaction(&[create_tor_ix], None)
            .await?;

        let deposit_ix = deposit_governing_tokens(
            &self.program_id,
            &realm_cookie.address,
            &governing_token_account_cookie.address,
            &token_owner,
            &token_owner,
            &self.bench.payer.pubkey(),
            1,
            &governing_token_mint,
        );

        self.bench.process_transaction(&[deposit_ix], None).await?;

        let governance_address = get_governance_address(
            &self.program_id,
            &realm_cookie.address,
            &token_account_cookie.address,
        );

        let create_governance_ix = create_governance(
            &self.program_id,
            &realm_cookie.address,
            Some(&token_account_cookie.address),
            &token_owner_record,
            &self.bench.payer.pubkey(),
            &realm_cookie.realm_authority.pubkey(),
            None,
            spl_governance::state::governance::GovernanceConfig {
                vote_threshold_percentage: VoteThresholdPercentage::YesVote(60),
                min_community_weight_to_create_proposal: 1,
                min_transaction_hold_up_time: 0,
                max_voting_time: 600,
                vote_tipping: VoteTipping::Disabled,
                proposal_cool_off_time: 0,
                min_council_weight_to_create_proposal: 1,
            },
        );

        self.bench
            .process_transaction(
                &[create_governance_ix],
                Some(&[&realm_cookie.realm_authority]),
            )
            .await?;

        let proposal_address = get_proposal_address(
            &self.program_id,
            &governance_address,
            &governing_token_mint,
            &[0],
        );

        let create_proposal_ix = create_proposal(
            &self.program_id,
            &governance_address,
            &token_owner_record,
            &token_owner,
            &self.bench.payer.pubkey(),
            None,
            &realm_cookie.address,
            String::from("Proposal #1"),
            String::from("Proposal #1 link"),
            &governing_token_mint,
            spl_governance::state::proposal::VoteType::SingleChoice,
            vec!["Yes".to_string(), "No".to_string()],
            true,
            0_u32,
        );

        self.bench
            .process_transaction(&[create_proposal_ix], None)
            .await?;

        let account = ProposalV2 {
            account_type: GovernanceAccountType::GovernanceV2,
            governing_token_mint: governing_token_mint,
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
            reserved: [0; 64],
            name: String::from("Proposal #1"),
            description_link: String::from("Proposal #1 link"),
        };

        Ok(ProposalCookie {
            address: proposal_address,
            account,
        })
    }
}
