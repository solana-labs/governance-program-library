use std::sync::{Arc, RwLock};

use anchor_lang::{prelude::Pubkey, system_program::System, Id};

use anchor_spl::{
    associated_token::{self, AssociatedToken},
    token_interface::TokenAccount,
};
use gpl_token_voter::state::*;
use solana_sdk::{instruction::AccountMeta, sysvar::instructions};

use crate::program_test::governance_test::GovernanceTest;
use crate::program_test::program_test_bench::ProgramTestBench;
use anchor_lang::ToAccountMetas;
use solana_program::program_pack::Pack;
use solana_program_test::{processor, BanksClientError, ProgramTest};
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::program_test::governance_test::RealmCookie;

use crate::program_test::tools::NopOverride;

use crate::program_test::governance_test::TokenOwnerRecordCookie;

use super::{
    program_test_bench::{MintCookie, MintType},
    LoggerWrapper, ProgramOutput,
};

#[derive(Debug, PartialEq)]
pub struct RegistrarCookie {
    pub address: Pubkey,
    pub account: Registrar,
    pub mint: Vec<MintCookie>,
    pub realm_authority: Keypair,
    pub max_mints: u8,
}

#[allow(dead_code)]
pub struct VoterWeightRecordCookie {
    pub address: Pubkey,
    pub account: VoterWeightRecord,
}

#[allow(dead_code)]
pub struct MaxVoterWeightRecordCookie {
    pub address: Pubkey,
    pub account: MaxVoterWeightRecord,
}

#[derive(Debug, PartialEq)]
pub struct UserCookie {
    pub key: Keypair,
    pub token_accounts: Vec<Pubkey>,
}

#[allow(dead_code)]
pub struct VoterCookie {
    pub address: Pubkey,
    pub authority: Pubkey,
    pub voter_weight_record: Pubkey,
    pub voter_weight_record_cookie: VoterWeightRecordCookie,
}

pub struct GovernanceProgramCookie {
    pub program_id: Pubkey,
}

#[allow(dead_code)]
pub struct TokenVoterTest {
    pub program_id: Pubkey,
    pub bench: Arc<ProgramTestBench>,
    pub governance: GovernanceTest,
    pub mints: Vec<MintCookie>,
    pub users: Vec<UserCookie>,
}

impl TokenVoterTest {
    #[allow(dead_code)]
    pub async fn start_new() -> Self {
        let mut program_test = ProgramTest::new("gpl_token_voter", gpl_token_voter::id(), None);
        let (mints, users) = ProgramTestBench::add_mints_and_user_cookies_spl_token(
            &mut program_test,
            MintType::SplToken,
        );
        GovernanceTest::add_program(&mut program_test);
        let program_id = gpl_token_voter::id();

        let bench = ProgramTestBench::start_new(program_test).await;
        let bench_rc = Arc::new(bench);

        let governance_bench =
            GovernanceTest::new(bench_rc.clone(), Some(program_id), Some(program_id));

        // Setup the environment
        // We need to intercept logs to capture program log output
        let log_filter = "solana_rbpf=trace,\
        solana_runtime::message_processor=debug,\
        solana_runtime::system_instruction_processor=trace,\
        solana_program_test=info";
        let env_logger =
            env_logger::Builder::from_env(env_logger::Env::new().default_filter_or(log_filter))
                .format_timestamp_nanos()
                .build();
        let program_output = Arc::new(RwLock::new(ProgramOutput::default()));
        let _ = log::set_boxed_logger(Box::new(LoggerWrapper {
            inner: env_logger,
            output: program_output.clone(),
        }));

        Self {
            program_id,
            bench: bench_rc,
            governance: governance_bench,
            mints,
            users,
        }
    }

    #[allow(dead_code)]
    pub async fn start_new_token_extensions(transfer_hook_program_id: Option<&Pubkey>) -> Self {
        let mut program_test = ProgramTest::new("gpl_token_voter", gpl_token_voter::id(), None);
        let (mints, users) = ProgramTestBench::add_mints_and_user_cookies_spl_token(
            &mut program_test,
            MintType::SplTokenExtensions,
        );

        if transfer_hook_program_id.is_some() {
            program_test.set_compute_max_units(500_000);
            program_test.add_program(
                "spl_transfer_hook_example",
                *transfer_hook_program_id.unwrap(),
                processor!(spl_transfer_hook_example::processor::process),
            );
        };

        GovernanceTest::add_program(&mut program_test);
        let program_id = gpl_token_voter::id();

        let bench = ProgramTestBench::start_new(program_test).await;
        let bench_rc = Arc::new(bench);

        let governance_bench =
            GovernanceTest::new(bench_rc.clone(), Some(program_id), Some(program_id));

        // Setup the environment
        // We need to intercept logs to capture program log output
        let log_filter = "solana_rbpf=trace,\
        solana_runtime::message_processor=debug,\
        solana_runtime::system_instruction_processor=trace,\
        solana_program_test=info";
        let env_logger =
            env_logger::Builder::from_env(env_logger::Env::new().default_filter_or(log_filter))
                .format_timestamp_nanos()
                .build();
        let program_output = Arc::new(RwLock::new(ProgramOutput::default()));
        let _ = log::set_boxed_logger(Box::new(LoggerWrapper {
            inner: env_logger,
            output: program_output.clone(),
        }));

        Self {
            program_id,
            bench: bench_rc,
            governance: governance_bench,
            mints,
            users,
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
        let registrar_key =
            get_registrar_address(&realm_cookie.address, &realm_cookie.account.community_mint);

        let max_mints = 10;
        let data =
            anchor_lang::InstructionData::data(&gpl_token_voter::instruction::CreateRegistrar {
                max_mints,
            });

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &gpl_token_voter::accounts::CreateRegistrar {
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
            program_id: gpl_token_voter::id(),
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
            voting_mint_configs: vec![],
            max_mints,
            reserved: [0; 127],
        };
        Ok(RegistrarCookie {
            address: registrar_key,
            mint: vec![],
            account,
            realm_authority: realm_cookie.get_realm_authority(),
            max_mints,
        })
    }

    #[allow(dead_code)]
    pub async fn with_resize_registrar(
        &mut self,
        realm_cookie: &RealmCookie,
        max_mints: u8,
    ) -> Result<RegistrarCookie, BanksClientError> {
        self.with_resize_registrar_using_ix(realm_cookie, max_mints, NopOverride, None)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_resize_registrar_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        realm_cookie: &RealmCookie,
        max_mints: u8,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<RegistrarCookie, BanksClientError> {
        let registrar_key =
            get_registrar_address(&realm_cookie.address, &realm_cookie.account.community_mint);

        let data =
            anchor_lang::InstructionData::data(&gpl_token_voter::instruction::ResizeRegistrar {
                max_mints,
            });

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &gpl_token_voter::accounts::ResizeRegistrar {
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

        let mut resize_registrar_ix = Instruction {
            program_id: gpl_token_voter::id(),
            accounts,
            data,
        };

        instruction_override(&mut resize_registrar_ix);

        let default_signers = &[&realm_cookie.realm_authority];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[resize_registrar_ix], Some(signers))
            .await?;

        let account = Registrar {
            governance_program_id: self.governance.program_id,
            realm: realm_cookie.address,
            governing_token_mint: realm_cookie.account.community_mint,
            voting_mint_configs: vec![],
            max_mints,
            reserved: [0; 127],
        };
        Ok(RegistrarCookie {
            address: registrar_key,
            mint: vec![],
            account,
            realm_authority: realm_cookie.get_realm_authority(),
            max_mints,
        })
    }

    #[allow(dead_code)]
    pub async fn with_voter(
        &self,
        registrar_cookie: &RegistrarCookie,
        user_cookie: &UserCookie,
    ) -> Result<VoterCookie, BanksClientError> {
        self.with_voter_using_ix(registrar_cookie, user_cookie, NopOverride)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_voter_using_ix<F: Fn(&mut Instruction)>(
        &self,
        registrar_cookie: &RegistrarCookie,
        user_cookie: &UserCookie,
        instruction_override: F,
    ) -> Result<VoterCookie, BanksClientError> {
        let governing_token_owner = user_cookie.key.pubkey();
        let (voter_key, _) = Pubkey::find_program_address(
            &[
                &registrar_cookie.address.to_bytes(),
                b"voter".as_ref(),
                &governing_token_owner.to_bytes(),
            ],
            &gpl_token_voter::id(),
        );
        let (voter_weight_record_key, _) = Pubkey::find_program_address(
            &[
                &registrar_cookie.address.to_bytes(),
                b"voter-weight-record".as_ref(),
                &governing_token_owner.to_bytes(),
            ],
            &gpl_token_voter::id(),
        );

        let data = anchor_lang::InstructionData::data(
            &gpl_token_voter::instruction::CreateVoterWeightRecord {},
        );

        let accounts = gpl_token_voter::accounts::CreateVoterWeightRecord {
            registrar: registrar_cookie.address,
            voter: voter_key,
            voter_weight_record: voter_weight_record_key,
            voter_authority: governing_token_owner,
            system_program: solana_sdk::system_program::id(),
            instructions: solana_program::sysvar::instructions::id(),
        };

        let mut create_voter_weight_record_ix = Instruction {
            program_id: gpl_token_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        };

        instruction_override(&mut create_voter_weight_record_ix);

        self.bench
            .process_transaction(&[create_voter_weight_record_ix], Some(&[&user_cookie.key]))
            .await?;

        let account = VoterWeightRecord::new(
            registrar_cookie.account.realm,
            registrar_cookie.account.governing_token_mint,
            governing_token_owner,
            0,
            Some(0),
            None,
            None,
        );
        let voter_weight_record_cookie = VoterWeightRecordCookie {
            address: voter_weight_record_key,
            account,
        };

        Ok(VoterCookie {
            address: voter_key,
            authority: governing_token_owner,
            voter_weight_record: voter_weight_record_key,
            voter_weight_record_cookie,
        })
    }

    #[allow(dead_code)]
    pub async fn with_max_voter_weight_record(
        &self,
        registrar_cookie: &RegistrarCookie,
    ) -> Result<MaxVoterWeightRecordCookie, BanksClientError> {
        self.with_max_voter_weight_record_using_ix(registrar_cookie, NopOverride)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_max_voter_weight_record_using_ix<F: Fn(&mut Instruction)>(
        &self,
        registrar_cookie: &RegistrarCookie,
        instruction_override: F,
    ) -> Result<MaxVoterWeightRecordCookie, BanksClientError> {
        let max_voter_weight_record_key = MaxVoterWeightRecord::get_max_voter_weight_record_address(
            &registrar_cookie.account.realm,
            &registrar_cookie.account.governing_token_mint,
        );

        let data = anchor_lang::InstructionData::data(
            &gpl_token_voter::instruction::CreateMaxVoterWeightRecord {},
        );

        let accounts = gpl_token_voter::accounts::CreateMaxVoterWeightRecord {
            registrar: registrar_cookie.address,
            max_voter_weight_record: max_voter_weight_record_key,
            payer: self.bench.payer.pubkey(),
            realm: registrar_cookie.account.realm,
            governance_program_id: self.governance.program_id,
            realm_governing_token_mint: registrar_cookie.account.governing_token_mint,
            system_program: solana_sdk::system_program::id(),
        };

        let mut create_max_voter_weight_record_ix = Instruction {
            program_id: gpl_token_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        };

        instruction_override(&mut create_max_voter_weight_record_ix);

        self.bench
            .process_transaction(&[create_max_voter_weight_record_ix], None)
            .await?;

        let account = MaxVoterWeightRecord::new(
            registrar_cookie.account.realm,
            registrar_cookie.account.governing_token_mint,
            0,
            Some(0),
        );

        Ok(MaxVoterWeightRecordCookie {
            account,
            address: max_voter_weight_record_key,
        })
    }

    #[allow(dead_code)]
    pub async fn configure_mint_config(
        &self,
        registrar_cookie: &RegistrarCookie,
        governance_program_cookie: &GovernanceProgramCookie,
        max_voter_weight_cookie: &MaxVoterWeightRecordCookie,
        mint_cookie: &MintCookie,
        digit_shift: i8,
    ) -> Result<VotingMintConfig, BanksClientError> {
        self.configure_mint_config_using_ix(
            registrar_cookie,
            governance_program_cookie,
            max_voter_weight_cookie,
            mint_cookie,
            digit_shift,
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn configure_mint_config_using_ix<F: Fn(&mut Instruction)>(
        &self,
        registrar_cookie: &RegistrarCookie,
        governance_program_cookie: &GovernanceProgramCookie,
        max_voter_weight_cookie: &MaxVoterWeightRecordCookie,
        mint_cookie: &MintCookie,
        digit_shift: i8,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<VotingMintConfig, BanksClientError> {
        let data = anchor_lang::InstructionData::data(
            &gpl_token_voter::instruction::ConfigureMintConfig { digit_shift },
        );

        let accounts = gpl_token_voter::accounts::ConfigureVotingMintConfig {
            registrar: registrar_cookie.address,
            realm: registrar_cookie.account.realm,
            mint: mint_cookie.address,
            max_voter_weight_record: max_voter_weight_cookie.address,
            realm_authority: registrar_cookie.realm_authority.pubkey(),
            governance_program_id: governance_program_cookie.program_id.clone(),
        };

        let mut configure_mint_config_ix = Instruction {
            program_id: gpl_token_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        };

        instruction_override(&mut configure_mint_config_ix);

        let default_signers = &[&registrar_cookie.realm_authority];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[configure_mint_config_ix], Some(signers))
            .await?;

        Ok(VotingMintConfig {
            mint: mint_cookie.address,
            digit_shift,
            // hard coded
            mint_supply: 100 * 10u64.pow(6),
            reserved1: [0; 55],
        })
    }

    #[allow(dead_code)]
    pub async fn deposit_entry(
        &self,
        registrar_cookie: &RegistrarCookie,
        voter_cookie: &VoterCookie,
        user_cookie: &UserCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        mint_cookie: &MintCookie,
        token_program: &Pubkey,
        deposit_entry_index: u8,
        amount: u64,
        // additional accounts for transfer_hooks to work
        additional_account_meta: Option<Vec<AccountMeta>>,
    ) -> Result<(), BanksClientError> {
        self.deposit_entry_using_ix(
            registrar_cookie,
            voter_cookie,
            user_cookie,
            mint_cookie,
            token_owner_record_cookie,
            token_program,
            deposit_entry_index,
            amount,
            NopOverride,
            None,
            additional_account_meta,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn deposit_entry_using_ix<F: Fn(&mut Instruction)>(
        &self,
        registrar_cookie: &RegistrarCookie,
        voter_cookie: &VoterCookie,
        user_cookie: &UserCookie,
        mint_cookie: &MintCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        token_program: &Pubkey,
        deposit_entry_index: u8,
        amount: u64,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
        // additional accounts for transfer_hooks to work
        additional_account_meta: Option<Vec<AccountMeta>>,
    ) -> Result<(), BanksClientError> {
        let data = anchor_lang::InstructionData::data(&gpl_token_voter::instruction::Deposit {
            deposit_entry_index,
            amount,
        });
        let vault = associated_token::get_associated_token_address_with_program_id(
            &voter_cookie.address,
            &mint_cookie.address,
            token_program,
        );
        let deposit_token = associated_token::get_associated_token_address_with_program_id(
            &user_cookie.key.pubkey(),
            &mint_cookie.address,
            token_program,
        );
        let mut accounts = gpl_token_voter::accounts::Deposit {
            registrar: registrar_cookie.address,
            voter: voter_cookie.address,
            voter_weight_record: voter_cookie.voter_weight_record,
            vault,
            deposit_token,
            deposit_authority: user_cookie.key.pubkey(),
            mint: mint_cookie.address,
            token_owner_record: token_owner_record_cookie.address,
            token_program: *token_program,
            instructions: instructions::id(),
            system_program: System::id(),
            associated_token_program: AssociatedToken::id(),
        }
        .to_account_metas(None);

        if let Some(additional_account_meta) = additional_account_meta {
            accounts = accounts
                .into_iter()
                .chain(additional_account_meta.into_iter())
                .collect();
        };
        let mut deposit_ix = Instruction {
            program_id: gpl_token_voter::id(),
            accounts,
            data,
        };

        instruction_override(&mut deposit_ix);

        let default_signers = &[&user_cookie.key];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[deposit_ix], Some(signers))
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn withdraw_deposit_entry(
        &self,
        registrar_cookie: &RegistrarCookie,
        voter_cookie: &VoterCookie,
        user_cookie: &UserCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        mint_cookie: &MintCookie,
        token_program: &Pubkey,
        deposit_entry_index: u8,
        amount: u64,
        // additional accounts for transfer_hooks to work
        additional_account_meta: Option<Vec<AccountMeta>>,
    ) -> Result<(), BanksClientError> {
        self.withdraw_deposit_entry_using_ix(
            registrar_cookie,
            voter_cookie,
            user_cookie,
            mint_cookie,
            token_owner_record_cookie,
            token_program,
            deposit_entry_index,
            amount,
            NopOverride,
            None,
            additional_account_meta,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn withdraw_deposit_entry_using_ix<F: Fn(&mut Instruction)>(
        &self,
        registrar_cookie: &RegistrarCookie,
        voter_cookie: &VoterCookie,
        user_cookie: &UserCookie,
        mint_cookie: &MintCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        token_program: &Pubkey,
        deposit_entry_index: u8,
        amount: u64,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
        // additional accounts for transfer_hooks to work
        additional_account_meta: Option<Vec<AccountMeta>>,
    ) -> Result<(), BanksClientError> {
        let data = anchor_lang::InstructionData::data(&gpl_token_voter::instruction::Withdraw {
            deposit_entry_index,
            amount,
        });
        let vault = associated_token::get_associated_token_address_with_program_id(
            &voter_cookie.address,
            &mint_cookie.address,
            token_program,
        );
        let destination_ata = associated_token::get_associated_token_address_with_program_id(
            &user_cookie.key.pubkey(),
            &mint_cookie.address,
            token_program,
        );
        let mut accounts = gpl_token_voter::accounts::Withdraw {
            registrar: registrar_cookie.address,
            voter: voter_cookie.address,
            voter_weight_record: voter_cookie.voter_weight_record,
            vault,
            destination: destination_ata,
            voter_authority: user_cookie.key.pubkey(),
            mint: mint_cookie.address,
            token_owner_record: token_owner_record_cookie.address,
            token_program: *token_program,
            system_program: System::id(),
            associated_token_program: AssociatedToken::id(),
        }
        .to_account_metas(None);

        if let Some(additional_account_meta) = additional_account_meta {
            accounts = accounts
                .into_iter()
                .chain(additional_account_meta.into_iter())
                .collect();
        };

        let mut withdraw_ix = Instruction {
            program_id: gpl_token_voter::id(),
            accounts: accounts,
            data,
        };

        instruction_override(&mut withdraw_ix);

        let default_signers = &[&user_cookie.key];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[withdraw_ix], Some(signers))
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn close_voter_account(
        &self,
        registrar_cookie: &RegistrarCookie,
        voter_cookie: &VoterCookie,
        user_cookie: &UserCookie,
        mint_cookies: &Vec<MintCookie>,
        token_program: &Pubkey,
    ) -> Result<(), BanksClientError> {
        self.close_voter_account_using_ix(
            registrar_cookie,
            voter_cookie,
            user_cookie,
            mint_cookies,
            token_program,
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn close_voter_account_using_ix<F: Fn(&mut Instruction)>(
        &self,
        registrar_cookie: &RegistrarCookie,
        voter_cookie: &VoterCookie,
        user_cookie: &UserCookie,
        mint_cookies: &Vec<MintCookie>,
        token_program: &Pubkey,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<(), BanksClientError> {
        let data = anchor_lang::InstructionData::data(&gpl_token_voter::instruction::CloseVoter {});

        let accounts = gpl_token_voter::accounts::CloseVoter {
            registrar: registrar_cookie.address,
            voter: voter_cookie.address,
            voter_weight_record: voter_cookie.voter_weight_record,
            sol_destination: user_cookie.key.pubkey(),
            voter_authority: user_cookie.key.pubkey(),
            token_program: *token_program,
        };

        let remaining_accounts: Vec<_> = mint_cookies
            .iter()
            .map(|mint_cookie| {
                let user_mint_ata = associated_token::get_associated_token_address_with_program_id(
                    &voter_cookie.address,
                    &mint_cookie.address,
                    token_program,
                );
                AccountMeta::new(user_mint_ata, false)
            })
            .collect();

        let all_accounts: Vec<_> = accounts
            .to_account_metas(None)
            .into_iter()
            .chain(remaining_accounts.into_iter())
            .collect();

        let mut configure_mint_config_ix = Instruction {
            program_id: gpl_token_voter::id(),
            accounts: all_accounts,
            data,
        };

        instruction_override(&mut configure_mint_config_ix);

        let default_signers = &[&user_cookie.key];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[configure_mint_config_ix], Some(signers))
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_registrar_account(&self, registrar: &Pubkey) -> Registrar {
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

    #[allow(dead_code)]
    pub async fn get_voter(&self, voter: &Pubkey) -> Voter {
        self.bench.get_anchor_account(*voter).await
    }

    #[allow(dead_code)]
    pub async fn vault_balance(
        &self,
        voter: &VoterCookie,
        mint: &MintCookie,
        token_program_id: &Pubkey,
    ) -> u64 {
        let vault = self.associated_token_address(voter.address, mint, token_program_id);
        self.bench
            .get_anchor_account::<TokenAccount>(vault)
            .await
            .amount
    }

    #[allow(dead_code)]
    pub async fn deposit_amount(&self, voter: &VoterCookie, deposit_id: u8) -> u64 {
        self.bench
            .get_anchor_account::<Voter>(voter.address)
            .await
            .deposits[deposit_id as usize]
            .amount_deposited_native
    }

    #[allow(dead_code)]
    pub async fn token_balance(&self, token_account: &Pubkey) -> u64 {
        let token_account_data = self.bench.get_account(token_account).await.unwrap();
        let account_info: spl_token::state::Account =
            spl_token::state::Account::unpack_from_slice(token_account_data.data.as_slice())
                .unwrap();
        account_info.amount
    }
    pub fn associated_token_address(
        &self,
        address: Pubkey,
        mint: &MintCookie,
        token_program_id: &Pubkey,
    ) -> Pubkey {
        spl_associated_token_account::get_associated_token_address_with_program_id(
            &address,
            &&mint.address,
            token_program_id,
        )
    }
}
