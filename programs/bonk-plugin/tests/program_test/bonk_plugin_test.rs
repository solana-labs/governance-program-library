use std::sync::Arc;

use anchor_lang::prelude::{AccountMeta, Pubkey};
use gpl_bonk_plugin::state::*;

use solana_program_test::{BanksClientError, ProgramTest};
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::program_test::governance_test::GovernanceTest;
use crate::program_test::program_test_bench::ProgramTestBench;

use crate::program_test::governance_test::RealmCookie;
use crate::program_test::program_test_bench::WalletCookie;

use crate::program_test::tools::NopOverride;

use super::governance_test::TokenOwnerRecordCookie;
use super::spl_token_staking_test::SplTokenStakingCookie;

#[derive(Debug, PartialEq)]
pub struct RegistrarCookie {
    pub address: Pubkey,
    pub account: Registrar,

    pub realm_authority: Keypair,
    pub max_governance_programs: u8,
}

#[allow(dead_code)]
pub struct VoterWeightRecordCookie {
    pub address: Pubkey,
    pub account: VoterWeightRecord,
}

#[allow(dead_code)]
pub struct GovernanceProgramCookie {
    pub program_id: Pubkey,
}

#[allow(dead_code)]
pub struct BonkPluginTest {
    pub program_id: Pubkey,
    pub bench: Arc<ProgramTestBench>,
    pub governance: GovernanceTest,
}

impl BonkPluginTest {
    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("gpl_bonk_plugin", gpl_bonk_plugin::id(), None);
    }

    #[allow(dead_code)]
    pub async fn start_new() -> Self {
        let mut program_test = ProgramTest::default();

        BonkPluginTest::add_program(&mut program_test);
        GovernanceTest::add_program(&mut program_test);
        SplTokenStakingCookie::add_program(&mut program_test);

        let program_id = gpl_bonk_plugin::id();

        let bench = ProgramTestBench::start_new(program_test).await;
        let bench_rc = Arc::new(bench);

        let governance_bench = GovernanceTest::new(bench_rc.clone(), Some(program_id));

        Self {
            program_id,
            bench: bench_rc,
            governance: governance_bench,
        }
    }

    #[allow(dead_code)]
    pub async fn with_governance_program(
        &mut self,
        program_id: Option<Pubkey>,
    ) -> GovernanceProgramCookie {
        let program_id = program_id.unwrap_or(GovernanceTest::program_id());

        // Use the spl-governance instance used for testing
        GovernanceProgramCookie { program_id }
    }

    #[allow(dead_code)]
    pub async fn with_registrar(
        &mut self,
        realm_cookie: &RealmCookie,
        stake_pool_key: &Pubkey,
    ) -> Result<RegistrarCookie, BanksClientError> {
        self.with_registrar_using_ix(realm_cookie, stake_pool_key, NopOverride, None)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_registrar_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        realm_cookie: &RealmCookie,
        stake_pool_key: &Pubkey,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<RegistrarCookie, BanksClientError> {
        let registrar_key =
            get_registrar_address(&realm_cookie.address, &realm_cookie.account.community_mint);

        let max_governance_programs = 10;

        let data =
            anchor_lang::InstructionData::data(&gpl_bonk_plugin::instruction::CreateRegistrar {});

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &gpl_bonk_plugin::accounts::CreateRegistrar {
                registrar: registrar_key,
                realm: realm_cookie.address,
                previous_voter_weight_plugin_program_id: None,
                stake_pool: *stake_pool_key,
                governance_program_id: self.governance.program_id,
                governing_token_mint: realm_cookie.account.community_mint,
                realm_authority: realm_cookie.get_realm_authority().pubkey(),
                payer: self.bench.payer.pubkey(),
                system_program: solana_sdk::system_program::id(),
            },
            None,
        );

        let mut create_registrar_ix = Instruction {
            program_id: gpl_bonk_plugin::id(),
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
            realm_authority: realm_cookie.get_realm_authority().pubkey(),
            stake_pool: *stake_pool_key,
            previous_voter_weight_plugin_program_id: None,
            governance_program_id: self.governance.program_id,
            realm: realm_cookie.address,
            governing_token_mint: realm_cookie.account.community_mint,
            reserved: [0; 8],
        };

        Ok(RegistrarCookie {
            address: registrar_key,
            account,
            realm_authority: realm_cookie.get_realm_authority(),
            max_governance_programs,
        })
    }

    #[allow(dead_code)]
    pub async fn with_voter_weight_record(
        &self,
        registrar_cookie: &RegistrarCookie,
        voter_cookie: &WalletCookie,
    ) -> Result<VoterWeightRecordCookie, BanksClientError> {
        self.with_voter_weight_record_using_ix(registrar_cookie, voter_cookie, NopOverride)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_voter_weight_record_using_ix<F: Fn(&mut Instruction)>(
        &self,
        registrar_cookie: &RegistrarCookie,
        voter_cookie: &WalletCookie,
        instruction_override: F,
    ) -> Result<VoterWeightRecordCookie, BanksClientError> {
        let governing_token_owner = voter_cookie.address;

        let (voter_weight_record_key, _) = Pubkey::find_program_address(
            &[
                b"voter-weight-record".as_ref(),
                registrar_cookie.account.realm.as_ref(),
                registrar_cookie.account.governing_token_mint.as_ref(),
                governing_token_owner.as_ref(),
            ],
            &gpl_bonk_plugin::id(),
        );

        let (stake_deposit_record_key, _) = Pubkey::find_program_address(
            &[
                b"stake-deposit-record".as_ref(),
                voter_weight_record_key.as_ref(),
            ],
            &gpl_bonk_plugin::id(),
        );

        let data = anchor_lang::InstructionData::data(
            &gpl_bonk_plugin::instruction::CreateVoterWeightRecord {
                governing_token_owner,
            },
        );

        let accounts = gpl_bonk_plugin::accounts::CreateVoterWeightRecord {
            registrar: registrar_cookie.address,
            stake_deposit_record: stake_deposit_record_key,
            voter_weight_record: voter_weight_record_key,
            payer: self.bench.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        };

        let mut create_voter_weight_record_ix = Instruction {
            program_id: gpl_bonk_plugin::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        };

        instruction_override(&mut create_voter_weight_record_ix);

        self.bench
            .process_transaction(&[create_voter_weight_record_ix], None)
            .await?;

        let account = VoterWeightRecord {
            realm: registrar_cookie.account.realm,
            governing_token_mint: registrar_cookie.account.governing_token_mint,
            governing_token_owner,
            voter_weight: 0,
            voter_weight_expiry: Some(0),
            weight_action: None,
            weight_action_target: None,
            reserved: [0; 8],
        };

        Ok(VoterWeightRecordCookie {
            address: voter_weight_record_key,
            account,
        })
    }

    #[allow(dead_code)]
    pub async fn update_voter_weight_record(
        &self,
        registrar_cookie: &RegistrarCookie,
        voter_weight_record_cookie: &VoterWeightRecordCookie,
        input_voter_weight: &Pubkey,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        action_target: Pubkey,
        action: VoterWeightAction,
        proposal: Option<Pubkey>,
        voter_authority: &Keypair,
        governance_key: Pubkey,
        stake_deposit_receipts: &Option<Vec<Pubkey>>,
    ) -> Result<Instruction, BanksClientError> {
        let data = anchor_lang::InstructionData::data(
            &gpl_bonk_plugin::instruction::UpdateVoterWeightRecord {
                stake_receipts_count: if stake_deposit_receipts.is_some() {
                    stake_deposit_receipts.as_ref().unwrap().len() as u8
                } else {
                    0
                },
                action_target,
                action,
            },
        );
        let (stake_deposit_record_key, _) = Pubkey::find_program_address(
            &[
                b"stake-deposit-record".as_ref(),
                voter_weight_record_cookie.address.as_ref(),
            ],
            &gpl_bonk_plugin::id(),
        );

        let accounts = gpl_bonk_plugin::accounts::UpdateVoterWeightRecord {
            registrar: registrar_cookie.address,
            input_voter_weight: *input_voter_weight,
            voter_weight_record: voter_weight_record_cookie.address,
            stake_deposit_record: stake_deposit_record_key,
            voter_token_owner_record: token_owner_record_cookie.address,
            governance: governance_key,
            proposal,
            voter_authority: voter_authority.pubkey(),
            payer: self.bench.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        };

        let account_metas = anchor_lang::ToAccountMetas::to_account_metas(&accounts, None);
        let remaining_accounts: Vec<_> = stake_deposit_receipts
            .clone()
            .unwrap_or_default() // Use unwrap_or_default() to get empty Vec if None
            .iter()
            .map(|stake_deposit_receipt| AccountMeta::new(*stake_deposit_receipt, false))
            .collect();

        let all_accounts: Vec<_> = account_metas
            .into_iter()
            .chain(remaining_accounts.into_iter())
            .collect();

        let instruction = Instruction {
            program_id: gpl_bonk_plugin::id(),
            accounts: all_accounts,
            data,
        };

        Ok(instruction)
    }

    #[allow(dead_code)]
    pub async fn get_registrar_account(&mut self, registrar: &Pubkey) -> Registrar {
        self.bench.get_anchor_account::<Registrar>(*registrar).await
    }

    #[allow(dead_code)]
    pub async fn get_voter_weight_record(&self, voter_weight_record: &Pubkey) -> VoterWeightRecord {
        self.bench.get_anchor_account(*voter_weight_record).await
    }
}
