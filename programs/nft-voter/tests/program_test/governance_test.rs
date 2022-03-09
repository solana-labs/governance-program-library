use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use solana_program_test::ProgramTest;

pub struct GovernanceTest {
    pub program_id: Pubkey,
}

impl GovernanceTest {
    pub fn program_id() -> Pubkey {
        Pubkey::from_str("Governance111111111111111111111111111111111").unwrap()
    }

    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("spl_governance", Self::program_id(), None);
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        GovernanceTest {
            program_id: Self::program_id(),
        }
    }
}
