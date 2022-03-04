use std::{str::FromStr, sync::Arc};

use anchor_lang::prelude::Pubkey;
use solana_program_test::ProgramTest;

use crate::program_test::program_test_bench::ProgramTestBench;

pub struct GovernanceTestBench {
    pub bench: Arc<ProgramTestBench>,
    pub program_id: Pubkey,
}

impl GovernanceTestBench {
    pub fn program_id() -> Pubkey {
        Pubkey::from_str("Governance111111111111111111111111111111111").unwrap()
    }

    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("spl_governance", Self::program_id(), None);
    }

    pub fn new(bench: Arc<ProgramTestBench>) -> Self {
        GovernanceTestBench {
            program_id: Self::program_id(),
            bench,
        }
    }
}
