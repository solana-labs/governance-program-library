use std::str::FromStr;
use std::sync::Arc;

use anchor_lang::prelude::Pubkey;
use itertools::Either;
use solana_gateway::{
    instruction::{add_gatekeeper, issue},
    state::{get_gatekeeper_account_address, get_gateway_token_address_with_seed},
};
use solana_program::instruction::AccountMeta;

use gpl_civic_gateway::state::{get_registrar_address, Registrar, *};
use solana_sdk::{
    instruction::Instruction, signature::Keypair, signer::Signer, transport::TransportError,
};
use spl_governance::{
    instruction::cast_vote,
    state::vote_record::{Vote, VoteChoice},
};

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

pub struct GatewayCookie {
    pub gatekeeper_network: Keypair,
    pub gatekeeper: Keypair,
}

impl GatewayCookie {
    pub fn get_gatekeeper_account(&self) -> Pubkey {
        let (gatekeeper_account, _) = get_gatekeeper_account_address(
            &self.gatekeeper.pubkey(),
            &self.gatekeeper_network.pubkey(),
        );
        gatekeeper_account
    }
}

pub struct GatewayTokenCookie {
    pub address: Pubkey,
}

impl GatewayTokenCookie {
    pub fn new(owner: &Pubkey, gateway_cookie: &GatewayCookie) -> Self {
        let (address, _) = get_gateway_token_address_with_seed(
            owner,
            &None,
            &gateway_cookie.gatekeeper_network.pubkey(),
        );
        Self { address }
    }
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

pub struct GatewayVoterTest {
    pub program_id: Pubkey,
    pub bench: Arc<ProgramTestBench>,
    pub governance: GovernanceTest,
    pub predecessor_plugin: PredecessorPluginTest,
}

impl GatewayVoterTest {
    #[allow(dead_code)]
    pub fn add_programs(program_test: &mut ProgramTest) {
        program_test.add_program(
            "gpl_civic_gateway",
            gpl_civic_gateway::id(),
            // processor!(gpl_civic_gateway::entry),
            None,
        );
        program_test.add_program(
            "solana_gateway",
            Pubkey::from_str("gatem74V238djXdzWnJf94Wo1DcnuGkfijbf3AuBhfs").unwrap(),
            // processor!(solana_gateway::processor::process_instruction),
            None,
        );
    }

    #[allow(dead_code)]
    pub async fn start_new() -> Self {
        let mut program_test = ProgramTest::default();

        GatewayVoterTest::add_programs(&mut program_test);
        GovernanceTest::add_program(&mut program_test);
        PredecessorPluginTest::add_program(&mut program_test);

        let program_id = gpl_civic_gateway::id();

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
        gateway_cookie: &GatewayCookie,
        previous_plugin_program_id: Option<Pubkey>,
    ) -> Result<RegistrarCookie, BanksClientError> {
        self.with_registrar_using_ix(
            realm_cookie,
            gateway_cookie,
            previous_plugin_program_id,
            previous_plugin_program_id.is_some(),
            &gpl_civic_gateway::id(),
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_registrar_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        realm_cookie: &RealmCookie,
        gateway_cookie: &GatewayCookie,
        previous_plugin_program_id: Option<Pubkey>,
        use_previous_voter_weight_plugin: bool,
        program_id: &Pubkey,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<RegistrarCookie, BanksClientError> {
        let registrar_key =
            get_registrar_address(&realm_cookie.address, &realm_cookie.account.community_mint);

        let data =
            anchor_lang::InstructionData::data(&gpl_civic_gateway::instruction::CreateRegistrar {
                use_previous_voter_weight_plugin,
            });

        let mut accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &gpl_civic_gateway::accounts::CreateRegistrar {
                registrar: registrar_key,
                realm: realm_cookie.address,
                governance_program_id: self.governance.program_id,
                governing_token_mint: realm_cookie.account.community_mint,
                gatekeeper_network: gateway_cookie.gatekeeper_network.pubkey(),
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
            gatekeeper_network: gateway_cookie.gatekeeper_network.pubkey(),
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
    ) -> Result<
        (
            RealmCookie,
            RegistrarCookie,
            GatewayCookie,
            GatewayTokenCookie,
            WalletCookie,
        ),
        TransportError,
    > {
        let realm_cookie = self.governance.with_realm().await?;
        let gateway_cookie = self.with_gateway().await?;

        // register the gateway plugin registrar with a predecessor (the dummy voter weight plugin) if requested
        let predecessor_program_id = if with_predecessor {
            Some(PredecessorPluginTest::program_id())
        } else {
            None
        };

        let registrar_cookie = self
            .with_registrar(&realm_cookie, &gateway_cookie, predecessor_program_id)
            .await?;
        let voter_cookie = self.bench.with_wallet().await;
        let gateway_token_cookie = self
            .with_gateway_token(&gateway_cookie, &voter_cookie)
            .await?;

        Ok((
            realm_cookie,
            registrar_cookie,
            gateway_cookie,
            gateway_token_cookie,
            voter_cookie,
        ))
    }

    pub async fn with_gateway(&mut self) -> Result<GatewayCookie, TransportError> {
        self.with_gateway_using_ix(NopOverride, None).await
    }

    pub async fn with_gateway_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<GatewayCookie, TransportError> {
        let gatekeeper_network = Keypair::new();
        let gatekeeper = Keypair::new();

        let mut add_gatekeeper_ix = add_gatekeeper(
            &self.bench.payer.pubkey(),
            &gatekeeper.pubkey(),
            &gatekeeper_network.pubkey(),
        );

        instruction_override(&mut add_gatekeeper_ix);

        let default_signers = &[&gatekeeper_network];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[add_gatekeeper_ix], Some(signers))
            .await?;

        Ok(GatewayCookie {
            gatekeeper_network,
            gatekeeper,
        })
    }

    #[allow(dead_code)]
    pub async fn with_gateway_token(
        &mut self,
        gateway_cookie: &GatewayCookie,
        wallet_cookie: &WalletCookie,
    ) -> Result<GatewayTokenCookie, TransportError> {
        self.with_gateway_token_using_ix(gateway_cookie, wallet_cookie, NopOverride, None)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_gateway_token_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        gateway_cookie: &GatewayCookie,
        wallet_cookie: &WalletCookie,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<GatewayTokenCookie, TransportError> {
        let gatekeeper_account = gateway_cookie.get_gatekeeper_account();
        let gateway_token_cookie = GatewayTokenCookie::new(&wallet_cookie.address, gateway_cookie);

        let mut issue_ix = issue(
            &self.bench.payer.pubkey(),
            &wallet_cookie.address,
            &gatekeeper_account,
            &gateway_cookie.gatekeeper.pubkey(),
            &gateway_cookie.gatekeeper_network.pubkey(),
            None,
            None,
        );

        instruction_override(&mut issue_ix);

        let default_signers = &[&gateway_cookie.gatekeeper];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[issue_ix], Some(signers))
            .await?;

        Ok(gateway_token_cookie)
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
            &gpl_civic_gateway::id(),
        );

        let data = anchor_lang::InstructionData::data(
            &gpl_civic_gateway::instruction::CreateVoterWeightRecord {
                governing_token_owner,
            },
        );

        let accounts = gpl_civic_gateway::accounts::CreateVoterWeightRecord {
            registrar: registrar_cookie.address,
            voter_weight_record: voter_weight_record_key,
            payer: self.bench.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        };

        let mut create_voter_weight_record_ix = Instruction {
            program_id: gpl_civic_gateway::id(),
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
        gateway_token_cookie: &GatewayTokenCookie,
    ) -> Result<(), BanksClientError> {
        let data = anchor_lang::InstructionData::data(
            &gpl_civic_gateway::instruction::UpdateVoterWeightRecord {},
        );

        let accounts = gpl_civic_gateway::accounts::UpdateVoterWeightRecord {
            registrar: registrar_cookie.address,
            gateway_token: gateway_token_cookie.address,
            voter_weight_record: output_voter_weight_record_cookie.address,
            input_voter_weight: extract_voting_weight_address(input_voter_weight_cookie),
        };

        let account_metas = anchor_lang::ToAccountMetas::to_account_metas(&accounts, None);

        let instructions = vec![Instruction {
            program_id: gpl_civic_gateway::id(),
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
        gateway_cookie: &GatewayCookie,
        predecessor_program_id: Option<Pubkey>,
    ) -> Result<(), BanksClientError> {
        self.configure_registrar_using_ix(
            realm_cookie,
            registrar_cookie,
            gateway_cookie,
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
        gateway_cookie: &GatewayCookie,
        predecessor_program_id: Option<Pubkey>,
        use_previous_voter_weight_plugin: bool,
        instruction_override: F,
        signers_override: Option<Option<&[&Keypair]>>,
    ) -> Result<(), BanksClientError> {
        let data = anchor_lang::InstructionData::data(
            &gpl_civic_gateway::instruction::ConfigureRegistrar {
                use_previous_voter_weight_plugin,
            },
        );

        let mut accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &gpl_civic_gateway::accounts::ConfigureRegistrar {
                registrar: registrar_cookie.address,
                realm: realm_cookie.address,
                realm_authority: realm_cookie.get_realm_authority().pubkey(),
                gatekeeper_network: gateway_cookie.gatekeeper_network.pubkey(),
            },
            None,
        );

        if let Some(predecessor_id) = predecessor_program_id {
            accounts.push(AccountMeta::new_readonly(predecessor_id, false));
        }

        let mut configure_registrar_ix = Instruction {
            program_id: gpl_civic_gateway::id(),
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
        gateway_token_cookie: &GatewayTokenCookie,
        voter_token_owner_record_cookie: &TokenOwnerRecordCookie,
        input_voter_weight_cookie: &mut Either<&VoterWeightRecordCookie, &TokenOwnerRecordCookie>,
        args: Option<CastVoteArgs>,
    ) -> Result<(), TransportError> {
        let args = args.unwrap_or_default();

        let data = anchor_lang::InstructionData::data(
            &gpl_civic_gateway::instruction::UpdateVoterWeightRecord {},
        );

        let accounts = gpl_civic_gateway::accounts::UpdateVoterWeightRecord {
            registrar: registrar_cookie.address,
            voter_weight_record: voter_weight_record_cookie.address,
            gateway_token: gateway_token_cookie.address,
            input_voter_weight: extract_voting_weight_address(input_voter_weight_cookie),
        };

        let account_metas = anchor_lang::ToAccountMetas::to_account_metas(&accounts, None);

        let update_voter_weight_ix = Instruction {
            program_id: gpl_civic_gateway::id(),
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
