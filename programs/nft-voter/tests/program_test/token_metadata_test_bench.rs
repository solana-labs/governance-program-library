use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use solana_program_test::ProgramTest;

pub struct TokenMetadataTestBench {
    pub program_id: Pubkey,
}

impl TokenMetadataTestBench {
    pub fn program_id() -> Pubkey {
        Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap()
    }

    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("mpl_token_metadata", Self::program_id(), None);
    }

    #[allow(dead_code)]
    pub fn new() -> Self {
        TokenMetadataTestBench {
            program_id: Self::program_id(),
        }
    }
}
