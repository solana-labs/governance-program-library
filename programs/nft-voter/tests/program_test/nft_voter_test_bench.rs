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

pub struct NftVoterTestBench {
    pub bench: ProgramTestBench,
    pub governance_bench: GovernanceTestBench,
}

pub struct RegistrarCookie {
    pub registrar: Pubkey,
    pub realm: Pubkey,
    pub realm_governing_token_mint: Pubkey,
}

impl NftVoterTestBench {
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("gpl_nft_voter", gpl_nft_voter::id(), None);
    }

    pub async fn start_new() -> Self {
        let mut program_test = ProgramTest::default();

        NftVoterTestBench::add_program(&mut program_test);
        GovernanceTestBench::add_program(&mut program_test);

        let bench = ProgramTestBench::start_new(program_test).await;

        let governance_bench = GovernanceTestBench::new();

        Self {
            bench,
            governance_bench,
        }
    }

    #[allow(dead_code)]
    pub async fn with_registrar(&mut self) -> RegistrarCookie {
        let realm_governing_token_mint = Keypair::new();
        let realm_authority = Keypair::new();

        self.bench
            .create_mint(&realm_governing_token_mint, &realm_authority.pubkey(), None)
            .await;

        let name = self.bench.get_unique_name("realm");

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
            payer: self.bench.context.payer.pubkey(),
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
        }
    }

    #[allow(dead_code)]
    pub async fn with_voter_weight_record(&mut self, registrar_cookie: &RegistrarCookie) {
        let governing_token_owner = self.bench.context.payer.pubkey();

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

        self.bench.process_transaction(&instructions, None).await
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
}
