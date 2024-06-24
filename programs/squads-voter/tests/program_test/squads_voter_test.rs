use std::sync::Arc;

use anchor_lang::prelude::{AccountMeta, Pubkey};

use gpl_squads_voter::state::max_voter_weight_record::{
    get_max_voter_weight_record_address, MaxVoterWeightRecord,
};
use gpl_squads_voter::state::*;
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
    pub max_squads: u8,
}

pub struct VoterWeightRecordCookie {
    pub address: Pubkey,
    pub account: VoterWeightRecord,
}

pub struct MaxVoterWeightRecordCookie {
    pub address: Pubkey,
    pub account: MaxVoterWeightRecord,
}

pub struct SquadConfigCookie {
    pub squad_config: SquadConfig,
}

pub struct ConfigureSquadArgs {
    pub weight: u64,
}

impl Default for ConfigureSquadArgs {
    fn default() -> Self {
        Self { weight: 1 }
    }
}

pub struct SquadsVoterTest {
    pub program_id: Pubkey,
    pub bench: Arc<ProgramTestBench>,
    pub governance: GovernanceTest,
    pub squads: SquadsTest,
}

impl SquadsVoterTest {
    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("gpl_squads_voter", gpl_squads_voter::id(), None);
    }

    #[allow(dead_code)]
    pub async fn start_new() -> Self {
        let mut program_test = ProgramTest::default();

        SquadsVoterTest::add_program(&mut program_test);
        GovernanceTest::add_program(&mut program_test);
        SquadsTest::add_program(&mut program_test);

        let program_id = gpl_squads_voter::id();

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

        let max_squads = 10;

        let data =
            anchor_lang::InstructionData::data(&gpl_squads_voter::instruction::CreateRegistrar {
                max_squads,
            });

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &gpl_squads_voter::accounts::CreateRegistrar {
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
            program_id: gpl_squads_voter::id(),
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
            squads_configs: vec![],
            reserved: [0; 128],
        };

        Ok(RegistrarCookie {
            address: registrar_key,
            account,
            realm_authority: realm_cookie.get_realm_authority(),
            max_squads,
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
            &gpl_squads_voter::id(),
        );

        let data = anchor_lang::InstructionData::data(
            &gpl_squads_voter::instruction::CreateVoterWeightRecord {
                governing_token_owner,
            },
        );

        let accounts = gpl_squads_voter::accounts::CreateVoterWeightRecord {
            governance_program_id: self.governance.program_id,
            realm: registrar_cookie.account.realm,
            realm_governing_token_mint: registrar_cookie.account.governing_token_mint,
            voter_weight_record: voter_weight_record_key,
            payer: self.bench.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        };

        let mut create_voter_weight_record_ix = Instruction {
            program_id: gpl_squads_voter::id(),
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
            &gpl_squads_voter::instruction::CreateMaxVoterWeightRecord {},
        );

        let accounts = gpl_squads_voter::accounts::CreateMaxVoterWeightRecord {
            governance_program_id: self.governance.program_id,
            realm: registrar_cookie.account.realm,
            realm_governing_token_mint: registrar_cookie.account.governing_token_mint,
            max_voter_weight_record: max_voter_weight_record_key,
            payer: self.bench.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        };

        let mut create_max_voter_weight_record_ix = Instruction {
            program_id: gpl_squads_voter::id(),
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
            &gpl_squads_voter::instruction::UpdateVoterWeightRecord {},
        );

        let accounts = gpl_squads_voter::accounts::UpdateVoterWeightRecord {
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
            program_id: gpl_squads_voter::id(),
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
        squads_cookies: &[&SquadCookie],
    ) -> Result<(), TransportError> {
        let data = anchor_lang::InstructionData::data(
            &gpl_squads_voter::instruction::UpdateMaxVoterWeightRecord {},
        );

        let accounts = gpl_squads_voter::accounts::UpdateMaxVoterWeightRecord {
            registrar: registrar_cookie.address,
            max_voter_weight_record: max_voter_weight_record_cookie.address,
        };

        let mut account_metas = anchor_lang::ToAccountMetas::to_account_metas(&accounts, None);

        for squad_cookie in squads_cookies {
            account_metas.push(AccountMeta::new_readonly(squad_cookie.address, false));
        }

        let instructions = vec![Instruction {
            program_id: gpl_squads_voter::id(),
            accounts: account_metas,
            data,
        }];

        self.bench.process_transaction(&instructions, None).await
    }

    #[allow(dead_code)]
    pub async fn with_squad_config(
        &mut self,
        registrar_cookie: &RegistrarCookie,
        squad_cookie: &SquadCookie,
        args: Option<ConfigureSquadArgs>,
    ) -> Result<SquadConfigCookie, TransportError> {
        self.with_squad_config_using_ix(registrar_cookie, squad_cookie, args, NopOverride, None)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_squad_config_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        registrar_cookie: &RegistrarCookie,
        squad_cookie: &SquadCookie,
        args: Option<ConfigureSquadArgs>,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<SquadConfigCookie, TransportError> {
        let args = args.unwrap_or_default();

        let data =
            anchor_lang::InstructionData::data(&gpl_squads_voter::instruction::ConfigureSquad {
                weight: args.weight,
            });

        let accounts = gpl_squads_voter::accounts::ConfigureSquad {
            registrar: registrar_cookie.address,
            realm: registrar_cookie.account.realm,
            realm_authority: registrar_cookie.realm_authority.pubkey(),
            squad: squad_cookie.address,
        };

        let mut configure_squad_ix = Instruction {
            program_id: gpl_squads_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        };

        instruction_override(&mut configure_squad_ix);

        let default_signers = &[&registrar_cookie.realm_authority];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[configure_squad_ix], Some(signers))
            .await?;

        let squad_config = SquadConfig {
            squad: squad_cookie.address,
            weight: args.weight,
            reserved: [0; 8],
        };

        Ok(SquadConfigCookie { squad_config })
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
