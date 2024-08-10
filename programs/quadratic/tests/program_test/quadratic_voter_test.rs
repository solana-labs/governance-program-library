use std::sync::Arc;

use anchor_lang::prelude::Pubkey;
use itertools::Either;
use solana_program::instruction::AccountMeta;

use gpl_quadratic::state::{get_registrar_address, Registrar, *};
use solana_sdk::{instruction::Instruction, signature::Keypair, signer::Signer};
use spl_governance::{
    instruction::cast_vote,
    state::vote_record::{Vote, VoteChoice},
};

use gpl_quadratic::state::quadratic_coefficients::QuadraticCoefficients;
use solana_program_test::{processor, BanksClientError, ProgramTest};

use crate::program_test::{
    governance_test::{GovernanceTest, ProposalCookie, RealmCookie, TokenOwnerRecordCookie},
    predecessor_plugin_test::PredecessorPluginTest,
    program_test_bench::{ProgramTestBench, WalletCookie},
    tools::{extract_voting_weight_address, NopOverride},
};

#[derive(Debug, PartialEq)]
pub struct RegistrarCookie {
    pub address: Pubkey,
    pub account: Registrar,

    pub realm_authority: Keypair,
}

pub struct VoterWeightRecordCookie {
    pub address: Pubkey,
    pub account: VoterWeightRecord,
}

pub struct CastVoteArgs {
    pub cast_spl_gov_vote: bool,
}

impl Default for CastVoteArgs {
    fn default() -> Self {
        Self {
            cast_spl_gov_vote: true,
        }
    }
}

pub struct QuadraticVoterTest {
    pub program_id: Pubkey,
    pub bench: Arc<ProgramTestBench>,
    pub governance: GovernanceTest,
    pub predecessor_plugin: PredecessorPluginTest,
}

impl QuadraticVoterTest {
    #[allow(dead_code)]
    pub fn add_programs(program_test: &mut ProgramTest) {
        program_test.add_program("gpl_quadratic", gpl_quadratic::id(), None);
        // program_test.add_program(
        //     "gpl_quadratic",
        //     gpl_quadratic::id(),
        //     processor!(gpl_quadratic::entry),
        // );
    }

    #[allow(dead_code)]
    pub async fn start_new() -> Self {
        let mut program_test = ProgramTest::default();

        QuadraticVoterTest::add_programs(&mut program_test);
        GovernanceTest::add_program(&mut program_test);
        PredecessorPluginTest::add_program(&mut program_test);

        let program_id = gpl_quadratic::id();

        let bench = ProgramTestBench::start_new(program_test).await;
        let bench_rc = Arc::new(bench);

        let governance_bench =
            GovernanceTest::new(bench_rc.clone(), Some(program_id), Some(program_id));

        let predecessor_plugin = PredecessorPluginTest::new(bench_rc.clone());

        Self {
            program_id,
            bench: bench_rc,
            governance: governance_bench,
            predecessor_plugin,
        }
    }

    #[allow(dead_code)]
    pub async fn with_registrar(
        &mut self,
        realm_cookie: &RealmCookie,
        coefficients: &QuadraticCoefficients,
        previous_plugin_program_id: Option<Pubkey>,
    ) -> Result<RegistrarCookie, BanksClientError> {
        self.with_registrar_using_ix(
            realm_cookie,
            previous_plugin_program_id,
            previous_plugin_program_id.is_some(),
            coefficients,
            &gpl_quadratic::id(),
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_registrar_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        realm_cookie: &RealmCookie,
        previous_plugin_program_id: Option<Pubkey>,
        use_previous_voter_weight_plugin: bool,
        coefficients: &QuadraticCoefficients,
        program_id: &Pubkey,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<RegistrarCookie, BanksClientError> {
        let registrar_key =
            get_registrar_address(&realm_cookie.address, &realm_cookie.account.community_mint);

        let data =
            anchor_lang::InstructionData::data(&gpl_quadratic::instruction::CreateRegistrar {
                coefficients: *coefficients,
                use_previous_voter_weight_plugin,
            });

        let mut accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &gpl_quadratic::accounts::CreateRegistrar {
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

        if let Some(predecessor_id) = previous_plugin_program_id {
            accounts.push(AccountMeta::new_readonly(predecessor_id, false));
        }

        let mut create_registrar_ix = Instruction {
            program_id: *program_id,
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
            previous_voter_weight_plugin_program_id: previous_plugin_program_id,
            realm: realm_cookie.address,
            governing_token_mint: realm_cookie.account.community_mint,
            quadratic_coefficients: *coefficients,
            reserved: [0; 128],
        };

        Ok(RegistrarCookie {
            address: registrar_key,
            account,
            realm_authority: realm_cookie.get_realm_authority(),
        })
    }

    #[allow(dead_code)]
    pub async fn setup(
        &mut self,
        with_predecessor: bool,
        coefficients: &QuadraticCoefficients,
    ) -> Result<(RealmCookie, RegistrarCookie, WalletCookie), BanksClientError> {
        let realm_cookie = self.governance.with_realm().await?;

        // register the quadratic plugin registrar with a predecessor (the dummy voter weight plugin) if requested
        let predecessor_program_id = if with_predecessor {
            Some(PredecessorPluginTest::program_id())
        } else {
            None
        };

        let registrar_cookie = self
            .with_registrar(&realm_cookie, coefficients, predecessor_program_id)
            .await?;

        let voter_cookie = self.bench.with_wallet().await;

        Ok((realm_cookie, registrar_cookie, voter_cookie))
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
            &gpl_quadratic::id(),
        );

        let data = anchor_lang::InstructionData::data(
            &gpl_quadratic::instruction::CreateVoterWeightRecord {
                governing_token_owner,
            },
        );

        let accounts = gpl_quadratic::accounts::CreateVoterWeightRecord {
            registrar: registrar_cookie.address,
            voter_weight_record: voter_weight_record_key,
            payer: self.bench.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        };

        let mut create_voter_weight_record_ix = Instruction {
            program_id: gpl_quadratic::id(),
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
        input_voter_weight_cookie: &mut Either<&VoterWeightRecordCookie, &TokenOwnerRecordCookie>,
        output_voter_weight_record_cookie: &mut VoterWeightRecordCookie,
    ) -> Result<(), BanksClientError> {
        let data = anchor_lang::InstructionData::data(
            &gpl_quadratic::instruction::UpdateVoterWeightRecord {},
        );

        let accounts = gpl_quadratic::accounts::UpdateVoterWeightRecord {
            registrar: registrar_cookie.address,
            voter_weight_record: output_voter_weight_record_cookie.address,
            input_voter_weight: extract_voting_weight_address(input_voter_weight_cookie),
        };

        let account_metas = anchor_lang::ToAccountMetas::to_account_metas(&accounts, None);

        let instructions = vec![Instruction {
            program_id: gpl_quadratic::id(),
            accounts: account_metas,
            data,
        }];

        self.bench.process_transaction(&instructions, None).await
    }

    #[allow(dead_code)]
    pub async fn configure_registrar(
        &self,
        realm_cookie: &RealmCookie,
        registrar_cookie: &RegistrarCookie,
        predecessor_program_id: Option<Pubkey>,
    ) -> Result<(), BanksClientError> {
        self.configure_registrar_using_ix(
            realm_cookie,
            registrar_cookie,
            predecessor_program_id,
            predecessor_program_id.is_some(),
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn configure_registrar_using_ix<F: Fn(&mut Instruction)>(
        &self,
        realm_cookie: &RealmCookie,
        registrar_cookie: &RegistrarCookie,
        predecessor_program_id: Option<Pubkey>,
        use_previous_voter_weight_plugin: bool,
        instruction_override: F,
        signers_override: Option<Option<&[&Keypair]>>,
    ) -> Result<(), BanksClientError> {
        let data =
            anchor_lang::InstructionData::data(&gpl_quadratic::instruction::ConfigureRegistrar {
                coefficients: QuadraticCoefficients::default(),
                use_previous_voter_weight_plugin,
            });

        let mut accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &gpl_quadratic::accounts::ConfigureRegistrar {
                registrar: registrar_cookie.address,
                realm: realm_cookie.address,
                realm_authority: realm_cookie.get_realm_authority().pubkey(),
            },
            None,
        );

        if let Some(predecessor_id) = predecessor_program_id {
            accounts.push(AccountMeta::new_readonly(predecessor_id, false));
        }

        let mut configure_registrar_ix = Instruction {
            program_id: gpl_quadratic::id(),
            accounts,
            data,
        };

        instruction_override(&mut configure_registrar_ix);

        let default_signers = [&realm_cookie.realm_authority];
        let signers = signers_override.unwrap_or(Some(default_signers.as_slice()));

        self.bench
            .process_transaction(&[configure_registrar_ix], signers)
            .await
    }

    /// Casts a vote
    #[allow(dead_code)]
    pub async fn cast_vote(
        &mut self,
        registrar_cookie: &RegistrarCookie,
        voter_weight_record_cookie: &VoterWeightRecordCookie,
        proposal_cookie: &ProposalCookie,
        voter_cookie: &WalletCookie,
        voter_token_owner_record_cookie: &TokenOwnerRecordCookie,
        input_voter_weight_cookie: &mut Either<&VoterWeightRecordCookie, &TokenOwnerRecordCookie>,
        args: Option<CastVoteArgs>,
    ) -> Result<(), BanksClientError> {
        let args = args.unwrap_or_default();

        let data = anchor_lang::InstructionData::data(
            &gpl_quadratic::instruction::UpdateVoterWeightRecord {},
        );

        let accounts = gpl_quadratic::accounts::UpdateVoterWeightRecord {
            registrar: registrar_cookie.address,
            voter_weight_record: voter_weight_record_cookie.address,
            input_voter_weight: extract_voting_weight_address(input_voter_weight_cookie),
        };

        let account_metas = anchor_lang::ToAccountMetas::to_account_metas(&accounts, None);

        let update_voter_weight_ix = Instruction {
            program_id: gpl_quadratic::id(),
            accounts: account_metas,
            data,
        };

        let mut instructions = vec![update_voter_weight_ix];

        if args.cast_spl_gov_vote {
            // spl-gov cast vote
            let vote = Vote::Approve(vec![VoteChoice {
                rank: 0,
                weight_percentage: 100,
            }]);

            let cast_vote_ix = cast_vote(
                &self.governance.program_id,
                &registrar_cookie.account.realm,
                &proposal_cookie.account.governance,
                &proposal_cookie.address,
                &proposal_cookie.account.token_owner_record,
                &voter_token_owner_record_cookie.address,
                &voter_cookie.address,
                &proposal_cookie.account.governing_token_mint,
                &self.bench.payer.pubkey(),
                Some(voter_weight_record_cookie.address),
                None,
                vote,
            );

            instructions.push(cast_vote_ix);
        }

        self.bench
            .process_transaction(&instructions, Some(&[&voter_cookie.signer]))
            .await?;

        Ok(())
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
