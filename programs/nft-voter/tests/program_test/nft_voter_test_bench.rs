use anchor_lang::prelude::Pubkey;
use solana_program_test::ProgramTest;
use solana_sdk::instruction::Instruction;
use solana_sdk::signer::Signer;

use solana_sdk::signature::Keypair;

use crate::program_test::governance_test_bench::GovernanceTestBench;
use crate::program_test::program_test_bench::ProgramTestBench;

pub struct NftVoterTestBench {
    pub bench: ProgramTestBench,
    pub governance_bench: GovernanceTestBench,
}

impl NftVoterTestBench {
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("gpl_nft_voter", gpl_nft_voter::id(), None);
    }

    pub async fn start_new() -> Self {
        let mut program_test = ProgramTest::default();

        NftVoterTestBench::add_program(&mut program_test);
        //  GovernanceTestBench::add_program(&mut program_test);

        let bench = ProgramTestBench::start_new(program_test).await;

        let governance_bench = GovernanceTestBench::new();

        Self {
            bench,
            governance_bench,
        }
    }

    pub async fn with_registrar(&mut self) {
        let realm = Pubkey::new_unique();
        let realm_governing_token_mint = Keypair::new();
        let realm_authority = Keypair::new();

        self.bench
            .create_mint(&realm_governing_token_mint, &realm_authority.pubkey(), None)
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
            .await
    }
}
