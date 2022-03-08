use std::{borrow::Borrow, str::FromStr, sync::Arc};

use anchor_lang::prelude::Pubkey;
use solana_program_test::ProgramTest;
use solana_sdk::signer::Signer;

use super::program_test_bench::ProgramTestBench;

pub struct NftCookie {}

pub struct TokenMetadataTestBench {
    pub bench: Arc<ProgramTestBench>,
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
    pub fn new(bench: Arc<ProgramTestBench>) -> Self {
        TokenMetadataTestBench {
            bench,
            program_id: Self::program_id(),
        }
    }

    #[allow(dead_code)]
    pub async fn with_nft_v2(&self) -> NftCookie {
        let nft_owner = self.bench.context.borrow().payer.pubkey();
        let mint_cookie = self.bench.with_mint().await;
        let nft_account_cookie = self.bench.with_tokens(&mint_cookie, &nft_owner, 1).await;

        NftCookie {}
    }
}
