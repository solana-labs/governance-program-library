use std::{str::FromStr, sync::Arc};

use anchor_lang::prelude::Pubkey;
use solana_program_test::ProgramTest;
use solana_sdk::{signature::Keypair, signer::Signer, transport::TransportError};
use spl_governance::{
    instruction::{
        create_governance, create_proposal, create_realm, create_token_owner_record,
        deposit_governing_tokens, relinquish_vote, set_governance_delegate, sign_off_proposal,
    },
    state::{
        enums::{
            GovernanceAccountType, MintMaxVoterWeightSource, ProposalState, VoteThreshold,
            VoteTipping,
        },
        governance::get_governance_address,
        proposal::{get_proposal_address, ProposalV2},
        realm::{get_realm_address, GoverningTokenConfigAccountArgs, RealmConfig, RealmV2},
        realm_config::GoverningTokenType,
        token_owner_record::{
            get_token_owner_record_address, TokenOwnerRecordV2, TOKEN_OWNER_RECORD_LAYOUT_VERSION,
        },
    },
};

use crate::program_test::{
    program_test_bench::{MintCookie, ProgramTestBench, WalletCookie},
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

pub struct TokenOwnerRecordCookie {
    pub address: Pubkey,
    pub account: TokenOwnerRecordV2,
}

pub struct GovernanceTest {
    pub program_id: Pubkey,
    pub bench: Arc<ProgramTestBench>,
    pub next_id: u8,
    pub community_voter_weight_addin: Option<Pubkey>,
    pub max_community_voter_weight_addin: Option<Pubkey>,
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
    pub fn new(
        bench: Arc<ProgramTestBench>,
        community_voter_weight_addin: Option<Pubkey>,
        max_community_voter_weight_addin: Option<Pubkey>,
    ) -> Self {
        GovernanceTest {
            bench,
            program_id: Self::program_id(),
            next_id: 0,
            community_voter_weight_addin,
            max_community_voter_weight_addin,
        }
    }

    #[allow(dead_code)]
    pub async fn with_realm(&mut self) -> Result<RealmCookie, TransportError> {
        let realm_authority = Keypair::new();

        let community_mint_cookie = self.bench.with_mint().await?;
        let council_mint_cookie = self.bench.with_mint().await?;

        self.next_id += 1;
        let realm_name = format!("Realm #{}", self.next_id).to_string();

        let min_community_weight_to_create_governance = 1;
        let community_mint_max_voter_weight_source = MintMaxVoterWeightSource::FULL_SUPPLY_FRACTION;

        let realm_key = get_realm_address(&self.program_id, &realm_name);

        let community_token_config_args = GoverningTokenConfigAccountArgs {
            voter_weight_addin: self.community_voter_weight_addin,
            max_voter_weight_addin: self.max_community_voter_weight_addin,
            token_type: GoverningTokenType::default(),
        };

        let create_realm_ix = create_realm(
            &self.program_id,
            &realm_authority.pubkey(),
            &community_mint_cookie.address,
            &self.bench.payer.pubkey(),
            Some(council_mint_cookie.address),
            Some(community_token_config_args),
            None,
            realm_name.clone(),
            min_community_weight_to_create_governance,
            community_mint_max_voter_weight_source.clone(),
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
                community_mint_max_voter_weight_source,
                legacy1: 0,
                legacy2: 0,
            },
            reserved_v2: [0; 128],
            legacy1: 0,
        };

        Ok(RealmCookie {
            address: realm_key,
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
    ) -> Result<ProposalCookie, TransportError> {
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

        let proposal_owner_record_key = get_token_owner_record_address(
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

        let governance_key = get_governance_address(
            &self.program_id,
            &realm_cookie.address,
            &token_account_cookie.address,
        );

        let create_governance_ix = create_governance(
            &self.program_id,
            &realm_cookie.address,
            Some(&token_account_cookie.address),
            &proposal_owner_record_key,
            &self.bench.payer.pubkey(),
            &realm_cookie.realm_authority.pubkey(),
            None,
            spl_governance::state::governance::GovernanceConfig {
                min_community_weight_to_create_proposal: 1,
                min_transaction_hold_up_time: 0,

                min_council_weight_to_create_proposal: 1,
                community_vote_threshold: VoteThreshold::YesVotePercentage(60),
                voting_base_time: 600,
                community_vote_tipping: VoteTipping::Strict,
                council_vote_threshold: VoteThreshold::YesVotePercentage(60),
                council_veto_vote_threshold: VoteThreshold::Disabled,
                council_vote_tipping: VoteTipping::Disabled,
                community_veto_vote_threshold: VoteThreshold::Disabled,
                voting_cool_off_time: 0,
                deposit_exempt_proposal_count: 10,
            },
        );

        self.bench
            .process_transaction(
                &[create_governance_ix],
                Some(&[&realm_cookie.realm_authority]),
            )
            .await?;

        let proposal_governing_token_mint = realm_cookie.account.community_mint;
        let proposal_seed = Pubkey::new_unique();

        let proposal_key = get_proposal_address(
            &self.program_id,
            &governance_key,
            &proposal_governing_token_mint,
            &proposal_seed,
        );

        let create_proposal_ix = create_proposal(
            &self.program_id,
            &governance_key,
            &proposal_owner_record_key,
            &token_owner,
            &self.bench.payer.pubkey(),
            None,
            &realm_cookie.address,
            String::from("Proposal #1"),
            String::from("Proposal #1 link"),
            &proposal_governing_token_mint,
            spl_governance::state::proposal::VoteType::SingleChoice,
            vec!["Yes".to_string()],
            true,
            &proposal_seed,
        );

        let sign_off_proposal_ix = sign_off_proposal(
            &self.program_id,
            &realm_cookie.address,
            &governance_key,
            &proposal_key,
            &token_owner,
            Some(&proposal_owner_record_key),
        );

        self.bench
            .process_transaction(&[create_proposal_ix, sign_off_proposal_ix], None)
            .await?;

        let account = ProposalV2 {
            account_type: GovernanceAccountType::GovernanceV2,
            governing_token_mint: proposal_governing_token_mint,
            state: ProposalState::Voting,
            governance: governance_key,
            token_owner_record: proposal_owner_record_key,
            signatories_count: 1,
            signatories_signed_off_count: 1,
            vote_type: spl_governance::state::proposal::VoteType::SingleChoice,
            options: vec![],
            deny_vote_weight: Some(1),
            veto_vote_weight: 0,
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
            reserved: [0; 64],
            name: String::from("Proposal #1"),
            description_link: String::from("Proposal #1 link"),
            reserved1: 0,
            vote_threshold: None,
        };

        Ok(ProposalCookie {
            address: proposal_key,
            account,
        })
    }

    #[allow(dead_code)]
    pub async fn with_token_owner_record(
        &mut self,
        realm_cookie: &RealmCookie,
        token_owner_cookie: &WalletCookie,
    ) -> Result<TokenOwnerRecordCookie, TransportError> {
        let token_owner_record_key = get_token_owner_record_address(
            &self.program_id,
            &realm_cookie.address,
            &realm_cookie.account.community_mint,
            &token_owner_cookie.address,
        );

        let create_tor_ix = create_token_owner_record(
            &self.program_id,
            &realm_cookie.address,
            &token_owner_cookie.address,
            &realm_cookie.account.community_mint,
            &self.bench.payer.pubkey(),
        );

        self.bench
            .process_transaction(&[create_tor_ix], None)
            .await?;

        let account = TokenOwnerRecordV2 {
            account_type: GovernanceAccountType::TokenOwnerRecordV2,
            realm: realm_cookie.address,
            governing_token_mint: realm_cookie.account.community_mint,
            governing_token_owner: token_owner_cookie.address,
            governing_token_deposit_amount: 0,
            unrelinquished_votes_count: 0,
            outstanding_proposal_count: 0,
            reserved: [0; 6],
            governance_delegate: None,
            reserved_v2: [0; 128],
            version: TOKEN_OWNER_RECORD_LAYOUT_VERSION,
        };

        Ok(TokenOwnerRecordCookie {
            address: token_owner_record_key,
            account,
        })
    }

    #[allow(dead_code)]
    pub async fn relinquish_vote(
        &mut self,
        proposal_cookie: &ProposalCookie,
        token_owner_cookie: &WalletCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
    ) -> Result<(), TransportError> {
        let relinquish_vote_ix = relinquish_vote(
            &self.program_id,
            &token_owner_record_cookie.account.realm,
            &proposal_cookie.account.governance,
            &proposal_cookie.address,
            &token_owner_record_cookie.address,
            &proposal_cookie.account.governing_token_mint,
            Some(token_owner_record_cookie.account.governing_token_owner),
            Some(self.bench.payer.pubkey()),
        );

        self.bench
            .process_transaction(&[relinquish_vote_ix], Some(&[&token_owner_cookie.signer]))
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn set_governance_delegate(
        &mut self,
        realm_cookie: &RealmCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        token_owner_authority_cookie: &WalletCookie,
        new_governance_delegate: &Option<Pubkey>,
    ) {
        let set_governance_delegate_ix = set_governance_delegate(
            &self.program_id,
            &token_owner_authority_cookie.address,
            &realm_cookie.address,
            &token_owner_record_cookie.account.governing_token_mint,
            &token_owner_record_cookie.account.governing_token_owner,
            new_governance_delegate,
        );

        self.bench
            .process_transaction(
                &[set_governance_delegate_ix],
                Some(&[&token_owner_authority_cookie.signer]),
            )
            .await
            .unwrap();
    }

    #[allow(dead_code)]
    pub async fn get_proposal(&mut self, proposal_key: &Pubkey) -> ProposalV2 {
        self.bench
            .get_borsh_account::<ProposalV2>(proposal_key)
            .await
    }

    #[allow(dead_code)]
    pub async fn get_token_owner_record(
        &mut self,
        token_owner_record_key: &Pubkey,
    ) -> TokenOwnerRecordV2 {
        self.bench
            .get_borsh_account::<TokenOwnerRecordV2>(token_owner_record_key)
            .await
    }
}
