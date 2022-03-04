use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use solana_program_test::ProgramTest;

pub struct GovernanceTestBench {
    pub program_id: Pubkey,
}

impl GovernanceTestBench {
    pub fn program_id() -> Pubkey {
        Pubkey::from_str("Governance111111111111111111111111111111111").unwrap()
    }

    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("spl_governance", Self::program_id(), None);
    }

    pub fn new() -> Self {
        GovernanceTestBench {
            program_id: Self::program_id(),
        }
    }
}
