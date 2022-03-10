use std::sync::Arc;

use anchor_lang::prelude::Pubkey;

use gpl_nft_voter::governance::get_max_voter_weight_record_address;
use gpl_nft_voter::state::{get_registrar_address, CollectionConfig, Registrar};
use solana_program_test::{BanksClientError, ProgramTest};
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use spl_governance_addin_api::max_voter_weight::MaxVoterWeightRecord;

use crate::program_test::governance_test::GovernanceTest;
use crate::program_test::program_test_bench::ProgramTestBench;

use super::governance_test::RealmCookie;
use super::token_metadata_test::{NftCollectionCookie, TokenMetadataTest};
use super::tools::NopOverride;

pub struct NftVoterTest {
    pub bench: Arc<ProgramTestBench>,
    pub governance: GovernanceTest,
    pub token_metadata: TokenMetadataTest,
}

#[derive(Debug, PartialEq)]
pub struct RegistrarCookie {
    pub address: Pubkey,
    pub account: Registrar,
    pub realm_authority: Keypair,
}

pub struct VoterWeightRecordCookie {
    pub voter_weight_record: Pubkey,
    pub governing_token_owner: Pubkey,
}

pub struct MaxVoterWeightRecordCookie {
    pub address: Pubkey,
}

pub struct CollectionConfigCookie {
    pub collection_config: CollectionConfig,
}

impl NftVoterTest {
    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("gpl_nft_voter", gpl_nft_voter::id(), None);
    }

    #[allow(dead_code)]
    pub async fn start_new() -> Self {
        let mut program_test = ProgramTest::default();

        NftVoterTest::add_program(&mut program_test);
        GovernanceTest::add_program(&mut program_test);
        TokenMetadataTest::add_program(&mut program_test);

        let bench = ProgramTestBench::start_new(program_test).await;
        let bench_rc = Arc::new(bench);

        let governance_bench = GovernanceTest::new(bench_rc.clone());
        let token_metadata_bench = TokenMetadataTest::new(bench_rc.clone());

        Self {
            bench: bench_rc,
            governance: governance_bench,
            token_metadata: token_metadata_bench,
        }
    }

    #[allow(dead_code)]
    pub async fn with_registrar(
        &mut self,
        realm_cookie: &RealmCookie,
    ) -> Result<RegistrarCookie, BanksClientError> {
        self.with_registrar_using_ix(realm_cookie, NopOverride, None)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_registrar_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        realm_cookie: &RealmCookie,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<RegistrarCookie, BanksClientError> {
        let registrar =
            get_registrar_address(&realm_cookie.address, &realm_cookie.account.community_mint);

        let data =
            anchor_lang::InstructionData::data(&gpl_nft_voter::instruction::CreateRegistrar {
                max_collections: 10,
            });

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &gpl_nft_voter::accounts::CreateRegistrar {
                registrar,
                realm: realm_cookie.address,
                governance_program_id: self.governance.program_id,
                governing_token_mint: realm_cookie.account.community_mint,
                realm_authority: realm_cookie.get_realm_authority().pubkey(),
                payer: self.bench.payer.pubkey(),
                system_program: solana_sdk::system_program::id(),
            },
            None,
        );

        let mut create_registrar_ix = Instruction {
            program_id: gpl_nft_voter::id(),
            accounts,
            data,
        };

        instruction_override(&mut create_registrar_ix);

        let default_signers = &[&realm_cookie.realm_authority];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[create_registrar_ix], Some(signers))
            .await?;

        let account = Registrar {
            governance_program_id: self.governance.program_id,
            realm: realm_cookie.address,
            governing_token_mint: realm_cookie.account.community_mint,
            collection_configs: vec![],
            reserved: [0; 64],
        };

        Ok(RegistrarCookie {
            address: registrar,
            account,
            realm_authority: realm_cookie.get_realm_authority(),
        })
    }

    #[allow(dead_code)]
    pub async fn with_voter_weight_record(
        &mut self,
        registrar_cookie: &RegistrarCookie,
    ) -> Result<VoterWeightRecordCookie, BanksClientError> {
        let governing_token_owner = self.bench.context.borrow().payer.pubkey();

        let (voter_weight_record, _) = Pubkey::find_program_address(
            &[
                b"voter-weight-record".as_ref(),
                registrar_cookie.account.realm.as_ref(),
                registrar_cookie.account.governing_token_mint.as_ref(),
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
            registrar: registrar_cookie.address,
            realm: registrar_cookie.account.realm,
            realm_governing_token_mint: registrar_cookie.account.governing_token_mint,
            voter_weight_record,
            payer: governing_token_owner,
            system_program: solana_sdk::system_program::id(),
        };

        let instructions = vec![Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        }];

        self.bench.process_transaction(&instructions, None).await?;

        Ok(VoterWeightRecordCookie {
            voter_weight_record,
            governing_token_owner,
        })
    }

    #[allow(dead_code)]
    pub async fn with_max_voter_weight_record(
        &mut self,
        registrar_cookie: &RegistrarCookie,
    ) -> Result<MaxVoterWeightRecordCookie, BanksClientError> {
        let max_voter_weight_record_address = get_max_voter_weight_record_address(
            &registrar_cookie.account.realm,
            &registrar_cookie.account.governing_token_mint,
        );

        let data = anchor_lang::InstructionData::data(
            &gpl_nft_voter::instruction::CreateMaxVoterWeightRecord {},
        );

        let accounts = gpl_nft_voter::accounts::CreateMaxVoterWeightRecord {
            registrar: registrar_cookie.address,
            realm: registrar_cookie.account.realm,
            realm_governing_token_mint: registrar_cookie.account.governing_token_mint,
            max_voter_weight_record: max_voter_weight_record_address,
            payer: self.bench.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        };

        let instructions = vec![Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        }];

        self.bench.process_transaction(&instructions, None).await?;

        Ok(MaxVoterWeightRecordCookie {
            address: max_voter_weight_record_address,
        })
    }

    #[allow(dead_code)]
    pub async fn update_voter_weight_record(
        &mut self,
        registrar_cookie: &RegistrarCookie,
        voter_weight_record_cookie: &VoterWeightRecordCookie,
    ) -> Result<(), BanksClientError> {
        let data = anchor_lang::InstructionData::data(
            &gpl_nft_voter::instruction::UpdateVoterWeightRecord {
                governing_token_owner: voter_weight_record_cookie.governing_token_owner,
                realm: registrar_cookie.account.realm,
                governing_token_mint: registrar_cookie.account.governing_token_mint,
            },
        );

        let accounts = gpl_nft_voter::accounts::UpdateVoterWeightRecord {
            registrar: registrar_cookie.address,
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
    ) -> Result<(), BanksClientError> {
        let data = anchor_lang::InstructionData::data(
            &gpl_nft_voter::instruction::UpdateVoterWeightRecord {
                governing_token_owner: voter_weight_record_cookie.governing_token_owner,
                realm: registrar_cookie.account.realm,
                governing_token_mint: registrar_cookie.account.governing_token_mint,
            },
        );

        let accounts = gpl_nft_voter::accounts::UpdateVoterWeightRecord {
            registrar: registrar_cookie.address,
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
    pub async fn with_configure_collection(
        &mut self,
        registrar_cookie: &RegistrarCookie,
        collection_cookie: &NftCollectionCookie,
        max_voter_weight_record_cookie: &MaxVoterWeightRecordCookie,
    ) -> Result<CollectionConfigCookie, BanksClientError> {
        let coll_size = 3;
        let coll_weight = 1;

        let data =
            anchor_lang::InstructionData::data(&gpl_nft_voter::instruction::ConfigureCollection {
                weight: coll_weight,
                size: coll_size,
            });

        let accounts = gpl_nft_voter::accounts::ConfigureCollection {
            registrar: registrar_cookie.address,
            realm: registrar_cookie.account.realm,
            realm_authority: registrar_cookie.realm_authority.pubkey(),
            collection: collection_cookie.address,
            max_voter_weight_record: max_voter_weight_record_cookie.address,
        };

        let instructions = vec![Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        }];

        self.bench
            .process_transaction(&instructions, Some(&[&registrar_cookie.realm_authority]))
            .await?;

        let collection_config = CollectionConfig {
            collection: collection_cookie.address,
            size: coll_size,
            weight: coll_weight,
            reserved: [0; 8],
        };

        Ok(CollectionConfigCookie { collection_config })
    }

    #[allow(dead_code)]
    pub async fn get_registrar_account(&mut self, registrar: &Pubkey) -> Registrar {
        self.bench.get_anchor_account::<Registrar>(*registrar).await
    }

    #[allow(dead_code)]
    pub async fn get_max_voter_weight_record(
        &self,
        max_voter_weight_record: &Pubkey,
    ) -> MaxVoterWeightRecord {
        self.bench.get_borsh_account(max_voter_weight_record).await
    }
}
