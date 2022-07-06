use std::str::FromStr;
use std::sync::Arc;

use anchor_lang::prelude::Pubkey;
use solana_gateway::instruction::{add_gatekeeper, issue_vanilla};
use solana_gateway::state::{
    get_gatekeeper_address_with_seed, get_gateway_token_address_with_seed,
};

use gpl_civic_gateway::state::*;

use spl_governance::instruction::cast_vote;
use spl_governance::state::vote_record::{Vote, VoteChoice};

use gpl_civic_gateway::state::{get_registrar_address, Registrar};

use solana_program_test::{ProgramTest, BanksClientError};
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::program_test::governance_test::GovernanceTest;
use crate::program_test::program_test_bench::ProgramTestBench;

use crate::program_test::governance_test::{ProposalCookie, RealmCookie, TokenOwnerRecordCookie};
use crate::program_test::program_test_bench::WalletCookie;
use crate::program_test::tools::NopOverride;

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
        let (gatekeeper_account, _) = get_gatekeeper_address_with_seed(
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
}

impl GatewayVoterTest {
    #[allow(dead_code)]
    pub fn add_programs(program_test: &mut ProgramTest) {
        program_test.add_program("gpl_civic_gateway", gpl_civic_gateway::id(), None);
        program_test.add_program(
            "solana_gateway_program",
            Pubkey::from_str("gatem74V238djXdzWnJf94Wo1DcnuGkfijbf3AuBhfs").unwrap(),
            None,
        );
    }

    #[allow(dead_code)]
    pub async fn start_new() -> Self {
        let mut program_test = ProgramTest::default();

        GatewayVoterTest::add_programs(&mut program_test);
        GovernanceTest::add_program(&mut program_test);

        let program_id = gpl_civic_gateway::id();

        let bench = ProgramTestBench::start_new(program_test).await;
        let bench_rc = Arc::new(bench);

        let governance_bench =
            GovernanceTest::new(bench_rc.clone(), Some(program_id), Some(program_id));

        Self {
            program_id,
            bench: bench_rc,
            governance: governance_bench,
        }
    }

    #[allow(dead_code)]
    pub async fn with_registrar(
        &mut self,
        realm_cookie: &RealmCookie,
        gateway_cookie: &GatewayCookie,
    ) -> Result<RegistrarCookie, BanksClientError> {
        self.with_registrar_using_ix(realm_cookie, gateway_cookie, NopOverride, None)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_registrar_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        realm_cookie: &RealmCookie,
        gateway_cookie: &GatewayCookie,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<RegistrarCookie, BanksClientError> {
        let registrar_key =
            get_registrar_address(&realm_cookie.address, &realm_cookie.account.community_mint);

        let data =
            anchor_lang::InstructionData::data(&gpl_civic_gateway::instruction::CreateRegistrar {});

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
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

        let mut create_registrar_ix = Instruction {
            program_id: gpl_civic_gateway::id(),
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
            gatekeeper_network: gateway_cookie.gatekeeper_network.pubkey(),
            reserved: [0; 128],
        };

        Ok(RegistrarCookie {
            address: registrar_key,
            account,
            realm_authority: realm_cookie.get_realm_authority(),
        })
    }

    pub async fn with_gateway(&mut self) -> Result<GatewayCookie, BanksClientError> {
        self.with_gateway_using_ix(NopOverride, None).await
    }

    pub async fn with_gateway_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<GatewayCookie, BanksClientError> {
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
    ) -> Result<GatewayTokenCookie, BanksClientError> {
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
    ) -> Result<GatewayTokenCookie, BanksClientError> {
        let gatekeeper_account = gateway_cookie.get_gatekeeper_account();
        let gateway_token_cookie = GatewayTokenCookie::new(&wallet_cookie.address, gateway_cookie);

        let mut issue_ix = issue_vanilla(
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
            governance_program_id: self.governance.program_id,
            realm: registrar_cookie.account.realm,
            realm_governing_token_mint: registrar_cookie.account.governing_token_mint,
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
        voter_weight_record_cookie: &mut VoterWeightRecordCookie,
        gateway_token_cookie: &GatewayTokenCookie,
        voter_weight_action: VoterWeightAction,
    ) -> Result<(), BanksClientError> {
        let data = anchor_lang::InstructionData::data(
            &gpl_civic_gateway::instruction::UpdateVoterWeightRecord {
                voter_weight_action,
                target: None,
            },
        );

        let accounts = gpl_civic_gateway::accounts::UpdateVoterWeightRecord {
            registrar: registrar_cookie.address,
            gateway_token: gateway_token_cookie.address,
            voter_weight_record: voter_weight_record_cookie.address,
        };

        let account_metas = anchor_lang::ToAccountMetas::to_account_metas(&accounts, None);

        let instructions = vec![Instruction {
            program_id: gpl_civic_gateway::id(),
            accounts: account_metas,
            data,
        }];

        self.bench.process_transaction(&instructions, None).await
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
        args: Option<CastVoteArgs>,
    ) -> Result<(), BanksClientError> {
        let args = args.unwrap_or_default();

        let data = anchor_lang::InstructionData::data(
            &gpl_civic_gateway::instruction::UpdateVoterWeightRecord {
                voter_weight_action: VoterWeightAction::CastVote,
                target: Some(proposal_cookie.address),
            },
        );

        let accounts = gpl_civic_gateway::accounts::UpdateVoterWeightRecord {
            registrar: registrar_cookie.address,
            voter_weight_record: voter_weight_record_cookie.address,
            gateway_token: gateway_token_cookie.address,
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
