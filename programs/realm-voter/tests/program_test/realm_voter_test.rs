use std::sync::Arc;

use anchor_lang::prelude::{AccountMeta, Pubkey};

use gpl_realm_voter::state::max_voter_weight_record::{
    get_max_voter_weight_record_address, MaxVoterWeightRecord,
};
use gpl_realm_voter::state::*;
use solana_sdk::transport::TransportError;

use solana_program_test::ProgramTest;
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::program_test::governance_test::GovernanceTest;
use crate::program_test::program_test_bench::ProgramTestBench;

use crate::program_test::governance_test::RealmCookie;
use crate::program_test::program_test_bench::WalletCookie;

use crate::program_test::tools::NopOverride;

use crate::program_test::squads_test::{SquadCookie, SquadMemberCookie, SquadsTest};

#[derive(Debug, PartialEq)]
pub struct RegistrarCookie {
    pub address: Pubkey,
    pub account: Registrar,

    pub realm_authority: Keypair,
    pub max_governance_programs: u8,
}

pub struct VoterWeightRecordCookie {
    pub address: Pubkey,
    pub account: VoterWeightRecord,
}

pub struct MaxVoterWeightRecordCookie {
    pub address: Pubkey,
    pub account: MaxVoterWeightRecord,
}

pub struct GovernanceProgramConfigCookie {
    pub program_config: GovernanceProgramConfig,
}

pub struct GovernanceProgramCookie {
    pub program_id: Pubkey,
}

pub struct ConfigureGovernanceProgramArgs {
    pub weight: u64,
}

impl Default for ConfigureGovernanceProgramArgs {
    fn default() -> Self {
        Self { weight: 1 }
    }
}

pub struct RealmVoterTest {
    pub program_id: Pubkey,
    pub bench: Arc<ProgramTestBench>,
    pub governance: GovernanceTest,
    pub squads: SquadsTest,
}

impl RealmVoterTest {
    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("gpl_realm_voter", gpl_realm_voter::id(), None);
    }

    #[allow(dead_code)]
    pub async fn start_new() -> Self {
        let mut program_test = ProgramTest::default();

        RealmVoterTest::add_program(&mut program_test);
        GovernanceTest::add_program(&mut program_test);
        SquadsTest::add_program(&mut program_test);

        let program_id = gpl_realm_voter::id();

        let bench = ProgramTestBench::start_new(program_test).await;
        let bench_rc = Arc::new(bench);

        let governance_bench =
            GovernanceTest::new(bench_rc.clone(), Some(program_id), Some(program_id));
        let squads_bench = SquadsTest::new(bench_rc.clone());

        Self {
            program_id,
            bench: bench_rc,
            governance: governance_bench,
            squads: squads_bench,
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
    ) -> Result<RegistrarCookie, TransportError> {
        self.with_registrar_using_ix(realm_cookie, NopOverride, None)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_registrar_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        realm_cookie: &RealmCookie,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<RegistrarCookie, TransportError> {
        let registrar_key =
            get_registrar_address(&realm_cookie.address, &realm_cookie.account.community_mint);

        let max_governance_programs = 10;

        let data =
            anchor_lang::InstructionData::data(&gpl_realm_voter::instruction::CreateRegistrar {
                max_governance_programs,
            });

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &gpl_realm_voter::accounts::CreateRegistrar {
                registrar: registrar_key,
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
            program_id: gpl_realm_voter::id(),
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
            governance_program_configs: vec![],
            reserved: [0; 128],
            max_voter_weight: 0,
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
    ) -> Result<VoterWeightRecordCookie, TransportError> {
        self.with_voter_weight_record_using_ix(registrar_cookie, voter_cookie, NopOverride)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_voter_weight_record_using_ix<F: Fn(&mut Instruction)>(
        &self,
        registrar_cookie: &RegistrarCookie,
        voter_cookie: &WalletCookie,
        instruction_override: F,
    ) -> Result<VoterWeightRecordCookie, TransportError> {
        let governing_token_owner = voter_cookie.address;

        let (voter_weight_record_key, _) = Pubkey::find_program_address(
            &[
                b"voter-weight-record".as_ref(),
                registrar_cookie.account.realm.as_ref(),
                registrar_cookie.account.governing_token_mint.as_ref(),
                governing_token_owner.as_ref(),
            ],
            &gpl_realm_voter::id(),
        );

        let data = anchor_lang::InstructionData::data(
            &gpl_realm_voter::instruction::CreateVoterWeightRecord {
                governing_token_owner,
            },
        );

        let accounts = gpl_realm_voter::accounts::CreateVoterWeightRecord {
            governance_program_id: self.governance.program_id,
            realm: registrar_cookie.account.realm,
            realm_governing_token_mint: registrar_cookie.account.governing_token_mint,
            voter_weight_record: voter_weight_record_key,
            payer: self.bench.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        };

        let mut create_voter_weight_record_ix = Instruction {
            program_id: gpl_realm_voter::id(),
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
    pub async fn with_max_voter_weight_record(
        &mut self,
        registrar_cookie: &RegistrarCookie,
    ) -> Result<MaxVoterWeightRecordCookie, TransportError> {
        self.with_max_voter_weight_record_using_ix(registrar_cookie, NopOverride)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_max_voter_weight_record_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        registrar_cookie: &RegistrarCookie,
        instruction_override: F,
    ) -> Result<MaxVoterWeightRecordCookie, TransportError> {
        let max_voter_weight_record_key = get_max_voter_weight_record_address(
            &registrar_cookie.account.realm,
            &registrar_cookie.account.governing_token_mint,
        );

        let data = anchor_lang::InstructionData::data(
            &gpl_realm_voter::instruction::CreateMaxVoterWeightRecord {},
        );

        let accounts = gpl_realm_voter::accounts::CreateMaxVoterWeightRecord {
            governance_program_id: self.governance.program_id,
            realm: registrar_cookie.account.realm,
            realm_governing_token_mint: registrar_cookie.account.governing_token_mint,
            max_voter_weight_record: max_voter_weight_record_key,
            payer: self.bench.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        };

        let mut create_max_voter_weight_record_ix = Instruction {
            program_id: gpl_realm_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        };

        instruction_override(&mut create_max_voter_weight_record_ix);

        self.bench
            .process_transaction(&[create_max_voter_weight_record_ix], None)
            .await?;

        let account = MaxVoterWeightRecord {
            realm: registrar_cookie.account.realm,
            governing_token_mint: registrar_cookie.account.governing_token_mint,
            max_voter_weight: 0,
            max_voter_weight_expiry: Some(0),
            reserved: [0; 8],
        };

        Ok(MaxVoterWeightRecordCookie {
            account,
            address: max_voter_weight_record_key,
        })
    }

    #[allow(dead_code)]
    pub async fn update_voter_weight_record(
        &self,
        registrar_cookie: &RegistrarCookie,
        voter_weight_record_cookie: &mut VoterWeightRecordCookie,
        squads_member_cookies: &[&SquadMemberCookie],
    ) -> Result<(), TransportError> {
        let data = anchor_lang::InstructionData::data(
            &gpl_realm_voter::instruction::UpdateVoterWeightRecord {},
        );

        let accounts = gpl_realm_voter::accounts::UpdateVoterWeightRecord {
            registrar: registrar_cookie.address,
            voter_weight_record: voter_weight_record_cookie.address,
        };

        let mut account_metas = anchor_lang::ToAccountMetas::to_account_metas(&accounts, None);

        for squad_member_cookie in squads_member_cookies {
            account_metas.push(AccountMeta::new_readonly(
                squad_member_cookie.squad_address,
                false,
            ));
        }

        let instructions = vec![Instruction {
            program_id: gpl_realm_voter::id(),
            accounts: account_metas,
            data,
        }];

        self.bench.process_transaction(&instructions, None).await
    }

    #[allow(dead_code)]
    pub async fn update_max_voter_weight_record(
        &self,
        registrar_cookie: &RegistrarCookie,
        max_voter_weight_record_cookie: &mut MaxVoterWeightRecordCookie,
        max_voter_weight: u64,
    ) -> Result<(), TransportError> {
        let data = anchor_lang::InstructionData::data(
            &gpl_realm_voter::instruction::UpdateMaxVoterWeightRecord { max_voter_weight },
        );

        let accounts = gpl_realm_voter::accounts::ConfigureMaxVoterWeight {
            registrar: registrar_cookie.address,
            max_voter_weight_record: max_voter_weight_record_cookie.address,
            realm: registrar_cookie.account.realm,
            realm_authority: registrar_cookie.realm_authority.pubkey(),
        };

        let mut account_metas = anchor_lang::ToAccountMetas::to_account_metas(&accounts, None);

        let instructions = vec![Instruction {
            program_id: gpl_realm_voter::id(),
            accounts: account_metas,
            data,
        }];

        let default_signers = &[&registrar_cookie.realm_authority];

        self.bench
            .process_transaction(&instructions, Some(default_signers))
            .await
    }

    #[allow(dead_code)]
    pub async fn with_governance_program_config(
        &mut self,
        registrar_cookie: &RegistrarCookie,
        governance_program_cookie: &GovernanceProgramCookie,
        args: Option<ConfigureGovernanceProgramArgs>,
    ) -> Result<GovernanceProgramConfigCookie, TransportError> {
        self.with_governance_program_config_using_ix(
            registrar_cookie,
            governance_program_cookie,
            args,
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_governance_program_config_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        registrar_cookie: &RegistrarCookie,
        governance_program_cookie: &GovernanceProgramCookie,
        args: Option<ConfigureGovernanceProgramArgs>,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<GovernanceProgramConfigCookie, TransportError> {
        let args = args.unwrap_or_default();

        let data = anchor_lang::InstructionData::data(
            &gpl_realm_voter::instruction::ConfigureGovernanceProgram {
                weight: args.weight,
            },
        );

        let accounts = gpl_realm_voter::accounts::ConfigureGovernanceProgram {
            registrar: registrar_cookie.address,
            realm: registrar_cookie.account.realm,
            realm_authority: registrar_cookie.realm_authority.pubkey(),
            governance_program_id: governance_program_cookie.program_id.clone(),
        };

        let mut configure_squad_ix = Instruction {
            program_id: gpl_realm_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        };

        instruction_override(&mut configure_squad_ix);

        let default_signers = &[&registrar_cookie.realm_authority];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[configure_squad_ix], Some(signers))
            .await?;

        let squad_config = GovernanceProgramConfig {
            program_id: governance_program_cookie.program_id.clone(),

            reserved: [0; 8],
            weight: args.weight,
        };

        Ok(GovernanceProgramConfigCookie {
            program_config: squad_config,
        })
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
        self.bench
            .get_anchor_account(*max_voter_weight_record)
            .await
    }

    #[allow(dead_code)]
    pub async fn get_voter_weight_record(&self, voter_weight_record: &Pubkey) -> VoterWeightRecord {
        self.bench.get_anchor_account(*voter_weight_record).await
    }
}
