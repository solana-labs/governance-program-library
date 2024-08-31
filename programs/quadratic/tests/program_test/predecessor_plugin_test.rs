use std::str::FromStr;
use std::sync::Arc;

use anchor_lang::prelude::Pubkey;

use gpl_quadratic::state::*;
use solana_sdk::{signature::Keypair, signer::Signer, transport::TransportError};
use spl_governance_addin_mock::instruction::*;

use crate::program_test::{
    governance_test::RealmCookie,
    program_test_bench::{ProgramTestBench, WalletCookie},
    quadratic_voter_test::VoterWeightRecordCookie,
};
use gpl_quadratic::state::VoterWeightRecord;
use solana_program_test::{BanksClientError, ProgramTest};

pub struct PredecessorPluginTest {
    pub bench: Arc<ProgramTestBench>,
}

impl PredecessorPluginTest {
    pub fn program_id() -> Pubkey {
        Pubkey::from_str("GovAddinMock1111111111111111111111111111111").unwrap()
    }

    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("spl_governance_addin_mock", Self::program_id(), None);
    }

    #[allow(dead_code)]
    pub fn new(bench: Arc<ProgramTestBench>) -> Self {
        PredecessorPluginTest { bench }
    }

    #[allow(dead_code)]
    pub async fn with_voter_weight_record(
        &self,
        realm_cookie: &RealmCookie,
        voter_cookie: &WalletCookie,
        voter_weight: u64,
    ) -> Result<VoterWeightRecordCookie, BanksClientError> {
        let governing_token_owner = voter_cookie.address;
        let voter_weight_record_account = Keypair::new();

        let setup_voter_weight_record_ix = setup_voter_weight_record(
            &Self::program_id(),
            &realm_cookie.address,
            &realm_cookie.account.community_mint,
            &voter_cookie.address,
            &voter_weight_record_account.pubkey(),
            &self.bench.payer.pubkey(),
            voter_weight,
            Some(0),
            None,
            None,
        );

        self.bench
            .process_transaction(
                &[setup_voter_weight_record_ix],
                Some(&[&voter_weight_record_account]),
            )
            .await?;

        let account = VoterWeightRecord {
            realm: realm_cookie.address,
            governing_token_mint: realm_cookie.account.community_mint,
            governing_token_owner,
            voter_weight: 0,
            voter_weight_expiry: Some(0),
            weight_action: None,
            weight_action_target: None,
            reserved: [0; 8],
        };

        Ok(VoterWeightRecordCookie {
            address: voter_weight_record_account.pubkey(),
            account,
        })
    }
}
