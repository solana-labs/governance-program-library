use std::{str::FromStr, sync::Arc};

use anchor_lang::prelude::Pubkey;
use mpl_token_metadata::state::Collection;
use solana_program_test::ProgramTest;
use solana_sdk::signer::Signer;

use super::program_test_bench::ProgramTestBench;

pub struct NftCookie {
    pub address: Pubkey,
    pub metadata_address: Pubkey,
}

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
        let payer = self.bench.context.borrow().payer.pubkey();

        let mint_cookie = self.bench.with_mint().await;
        let nft_account_cookie = self.bench.with_tokens(&mint_cookie, &nft_owner, 1).await;

        let metadata_seeds = &[
            b"metadata".as_ref(),
            self.program_id.as_ref(),
            &mint_cookie.address.as_ref(),
        ];
        let (metadata_address, _) = Pubkey::find_program_address(metadata_seeds, &self.program_id);

        let name = "TestNFT".to_string();
        let symbol = "NFT".to_string();
        let uri = "URI".to_string();

        let collection = Collection {
            verified: false,
            key: Pubkey::new_unique(),
        };

        let create_metadata_ix = mpl_token_metadata::instruction::create_metadata_accounts_v2(
            self.program_id,
            metadata_address,
            mint_cookie.address,
            mint_cookie.mint_authority.pubkey(),
            nft_owner.clone(),
            nft_owner.clone(),
            name,
            symbol,
            uri,
            None,
            0,
            false,
            false,
            Some(collection),
            None,
        );

        self.bench
            .process_transaction(&[create_metadata_ix], Some(&[&mint_cookie.mint_authority]))
            .await;

        let master_edition_seeds = &[
            b"metadata".as_ref(),
            self.program_id.as_ref(),
            mint_cookie.address.as_ref(),
            b"edition".as_ref(),
        ];
        let (master_edition_address, _) =
            Pubkey::find_program_address(master_edition_seeds, &self.program_id);

        let create_master_edition_ix = mpl_token_metadata::instruction::create_master_edition_v3(
            self.program_id,
            master_edition_address,
            mint_cookie.address,
            nft_owner,
            mint_cookie.mint_authority.pubkey(),
            metadata_address,
            payer,
            None,
        );

        self.bench
            .process_transaction(
                &[create_master_edition_ix],
                Some(&[&mint_cookie.mint_authority]),
            )
            .await;

        // let verify_collection = mpl_token_metadata::instruction::verify_collection(
        //     self.program_id,
        //     metadata_address,
        //     collection_authority.pubkey(),
        //     context.payer.pubkey().clone(),
        //     collection_mint,
        //     collection,
        //     collection_master_edition_account,
        //     collection_authority_record,
        // );

        NftCookie {
            address: nft_account_cookie.address,
            metadata_address,
        }
    }
}
