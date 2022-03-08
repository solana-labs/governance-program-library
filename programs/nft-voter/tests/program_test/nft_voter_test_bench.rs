use std::str::FromStr;
use std::sync::Arc;

use anchor_lang::prelude::Pubkey;
use solana_program_test::ProgramTest;
use solana_sdk::instruction::Instruction;
use solana_sdk::signer::Signer;

use solana_sdk::signature::Keypair;
use spl_governance::instruction::create_realm;
use spl_governance::state::enums::MintMaxVoteWeightSource;
use spl_governance::state::realm::get_realm_address;

use crate::program_test::governance_test_bench::GovernanceTestBench;
use crate::program_test::program_test_bench::ProgramTestBench;

use super::token_metadata_test_bench::TokenMetadataTestBench;

const COLLECTION_PUBKEY: &str = "2tNsB373yxWfqznG1TE3GtkXtBtkdG6QtKvyWahju31s";

pub struct NftVoterTestBench {
    pub bench: Arc<ProgramTestBench>,
    pub governance_bench: GovernanceTestBench,
    pub token_metadata: TokenMetadataTestBench,
}

pub struct RegistrarCookie {
    pub registrar: Pubkey,
    pub realm: Pubkey,
    pub realm_governing_token_mint: Pubkey,
    pub realm_authority: Keypair,
}

pub struct VoterWeightRecordCookie {
    pub voter_weight_record: Pubkey,
    pub governing_token_owner: Pubkey,
}

impl NftVoterTestBench {
    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("gpl_nft_voter", gpl_nft_voter::id(), None);
    }

    #[allow(dead_code)]
    pub async fn start_new() -> Self {
        let mut program_test = ProgramTest::default();

        NftVoterTestBench::add_program(&mut program_test);
        GovernanceTestBench::add_program(&mut program_test);
        TokenMetadataTestBench::add_program(&mut program_test);

        let bench = ProgramTestBench::start_new(program_test).await;
        let bench_rc = Arc::new(bench);

        let governance_bench = GovernanceTestBench::new();
        let token_metadata_bench = TokenMetadataTestBench::new(bench_rc.clone());

        Self {
            bench: bench_rc,
            governance_bench,
            token_metadata: token_metadata_bench,
        }
    }

    #[allow(dead_code)]
    pub async fn with_registrar(&mut self) -> RegistrarCookie {
        let realm_governing_token_mint = Keypair::new();
        let realm_authority = Keypair::new();

        self.bench
            .create_mint(&realm_governing_token_mint, &realm_authority.pubkey(), None)
            .await;

        let name = "realm".to_string();

        let realm = get_realm_address(&self.governance_bench.program_id, &name);

        let create_realm_ix = create_realm(
            &self.governance_bench.program_id,
            &realm_authority.pubkey(),
            &realm_governing_token_mint.pubkey(),
            &self.bench.payer.pubkey(),
            None,
            None,
            None,
            name.clone(),
            1,
            MintMaxVoteWeightSource::FULL_SUPPLY_FRACTION,
        );

        self.bench
            .process_transaction(&[create_realm_ix], None)
            .await;

        let (registrar, _) = Pubkey::find_program_address(
            &[
                b"registrar".as_ref(),
                &realm.to_bytes(),
                &realm_governing_token_mint.pubkey().to_bytes(),
            ],
            &gpl_nft_voter::id(),
        );

        let data =
            anchor_lang::InstructionData::data(&gpl_nft_voter::instruction::CreateRegistrar {});

        let accounts = gpl_nft_voter::accounts::CreateRegistrar {
            registrar,
            realm,
            governance_program_id: self.governance_bench.program_id,
            realm_governing_token_mint: realm_governing_token_mint.pubkey(),
            realm_authority: realm_authority.pubkey(),
            payer: self.bench.context.borrow().payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        };

        let instructions = vec![Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        }];

        // print!("ACCOUNTS {:?}", instructions);

        self.bench
            .process_transaction(&instructions, Some(&[&realm_authority]))
            .await;

        RegistrarCookie {
            registrar,
            realm,
            realm_governing_token_mint: realm_governing_token_mint.pubkey(),
            realm_authority,
        }
    }

    #[allow(dead_code)]
    pub async fn with_voter_weight_record(
        &mut self,
        registrar_cookie: &RegistrarCookie,
    ) -> VoterWeightRecordCookie {
        let governing_token_owner = self.bench.context.borrow().payer.pubkey();

        let (voter_weight_record, _) = Pubkey::find_program_address(
            &[
                b"voter-weight-record".as_ref(),
                registrar_cookie.realm.as_ref(),
                registrar_cookie.realm_governing_token_mint.as_ref(),
                governing_token_owner.as_ref(),
            ],
            &gpl_nft_voter::id(),
        );

        let data = anchor_lang::InstructionData::data(
            &gpl_nft_voter::instruction::CreateVoterWeightRecord {
                governing_token_owner: self.bench.payer.pubkey(),
            },
        );

        let accounts = gpl_nft_voter::accounts::CreateVoterWeightRecord {
            registrar: registrar_cookie.registrar,
            realm: registrar_cookie.realm,
            realm_governing_token_mint: registrar_cookie.realm_governing_token_mint,
            voter_weight_record,
            payer: governing_token_owner,
            system_program: solana_sdk::system_program::id(),
        };

        let instructions = vec![Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        }];

        self.bench.process_transaction(&instructions, None).await;

        VoterWeightRecordCookie {
            voter_weight_record,
            governing_token_owner,
        }
    }

    #[allow(dead_code)]
    pub async fn with_max_voter_weight_record(&mut self, registrar_cookie: &RegistrarCookie) {
        let (max_voter_weight_record, _) = Pubkey::find_program_address(
            &[
                b"max_voter-weight-record".as_ref(),
                registrar_cookie.realm.as_ref(),
                registrar_cookie.realm_governing_token_mint.as_ref(),
            ],
            &gpl_nft_voter::id(),
        );

        let data = anchor_lang::InstructionData::data(
            &gpl_nft_voter::instruction::CreateMaxVoterWeightRecord {},
        );

        let accounts = gpl_nft_voter::accounts::CreateMaxVoterWeightRecord {
            registrar: registrar_cookie.registrar,
            realm: registrar_cookie.realm,
            realm_governing_token_mint: registrar_cookie.realm_governing_token_mint,
            max_voter_weight_record,
            payer: self.bench.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        };

        let instructions = vec![Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        }];

        self.bench.process_transaction(&instructions, None).await
    }

    #[allow(dead_code)]
    pub async fn update_voter_weight_record(
        &mut self,
        registrar_cookie: &RegistrarCookie,
        voter_weight_record_cookie: &VoterWeightRecordCookie,
    ) {
        let data = anchor_lang::InstructionData::data(
            &gpl_nft_voter::instruction::UpdateVoterWeightRecord {
                governing_token_owner: voter_weight_record_cookie.governing_token_owner,
                realm: registrar_cookie.realm,
                governing_token_mint: registrar_cookie.realm_governing_token_mint,
            },
        );

        let accounts = gpl_nft_voter::accounts::UpdateVoterWeightRecord {
            registrar: registrar_cookie.registrar,
            voter_weight_record: voter_weight_record_cookie.voter_weight_record,
        };

        let instructions = vec![Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        }];
        self.bench.process_transaction(&instructions, None).await
    }

    #[allow(dead_code)]
    pub async fn relinquish_vote(
        &mut self,
        registrar_cookie: &RegistrarCookie,
        voter_weight_record_cookie: &VoterWeightRecordCookie,
    ) {
        let data = anchor_lang::InstructionData::data(
            &gpl_nft_voter::instruction::UpdateVoterWeightRecord {
                governing_token_owner: voter_weight_record_cookie.governing_token_owner,
                realm: registrar_cookie.realm,
                governing_token_mint: registrar_cookie.realm_governing_token_mint,
            },
        );

        let accounts = gpl_nft_voter::accounts::UpdateVoterWeightRecord {
            registrar: registrar_cookie.registrar,
            voter_weight_record: voter_weight_record_cookie.voter_weight_record,
        };

        let instructions = vec![Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        }];

        self.bench.process_transaction(&instructions, None).await
    }
    #[allow(dead_code)]
    pub async fn with_configure_collection(&mut self, registrar_cookie: &mut RegistrarCookie) {
        // TODO: check which collection to use in local testing
        let collection = Pubkey::from_str(COLLECTION_PUBKEY).unwrap();

        let data =
            anchor_lang::InstructionData::data(&gpl_nft_voter::instruction::ConfigureCollection {
                multiplier: 1,
            });

        let accounts = gpl_nft_voter::accounts::ConfigureCollection {
            registrar: registrar_cookie.registrar,
            realm_authority: registrar_cookie.realm_authority.pubkey(),
            collection,
            token_program: spl_token::id(),
        };

        let instructions = vec![Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        }];

        self.bench
            .process_transaction(&instructions, Some(&[&registrar_cookie.realm_authority]))
            .await;
    }
}
