use std::{str::FromStr, sync::Arc};

use anchor_lang::prelude::Pubkey;
use mpl_token_metadata::state::Collection;
use solana_program_test::ProgramTest;
use solana_sdk::{signer::Signer, transport::TransportError};

use crate::program_test::program_test_bench::{MintCookie, ProgramTestBench, WalletCookie};

pub struct NftCookie {
    pub address: Pubkey,
    pub metadata: Pubkey,
    pub mint_cookie: MintCookie,
}

pub struct NftCollectionCookie {
    pub mint: Pubkey,
    pub metadata: Pubkey,
    pub master_edition: Pubkey,
}

pub struct CreateNftArgs {
    pub verify_collection: bool,
    pub amount: u64,
}

impl Default for CreateNftArgs {
    fn default() -> Self {
        Self {
            verify_collection: true,
            amount: 1,
        }
    }
}

pub struct TokenMetadataTest {
    pub bench: Arc<ProgramTestBench>,
    pub program_id: Pubkey,
}

impl TokenMetadataTest {
    pub fn program_id() -> Pubkey {
        Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap()
    }

    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("mpl_token_metadata", Self::program_id(), None);
    }

    #[allow(dead_code)]
    pub fn new(bench: Arc<ProgramTestBench>) -> Self {
        TokenMetadataTest {
            bench,
            program_id: Self::program_id(),
        }
    }

    #[allow(dead_code)]
    pub async fn with_nft_collection(&self) -> Result<NftCollectionCookie, TransportError> {
        let update_authority = self.bench.context.borrow().payer.pubkey();
        let payer = self.bench.context.borrow().payer.pubkey();

        // Create collection
        let coll_mint_cookie = self.bench.with_mint().await?;
        self.bench
            .with_tokens(&coll_mint_cookie, &update_authority, 1)
            .await?;

        let coll_metadata_seeds = &[
            b"metadata".as_ref(),
            self.program_id.as_ref(),
            &coll_mint_cookie.address.as_ref(),
        ];
        let (coll_metadata_key, _) =
            Pubkey::find_program_address(coll_metadata_seeds, &self.program_id);

        let coll_name = "NFT_C".to_string();
        let coll_symbol = "NFT_C".to_string();
        let coll_uri = "URI".to_string();

        let create_coll_metadata_ix = mpl_token_metadata::instruction::create_metadata_accounts_v3(
            self.program_id,
            coll_metadata_key,
            coll_mint_cookie.address,
            coll_mint_cookie.mint_authority.pubkey(),
            payer.clone(),
            update_authority.clone(),
            coll_name,
            coll_symbol,
            coll_uri,
            None,
            10,
            false,
            false,
            None,
            None,
            None,
        );

        self.bench
            .process_transaction(
                &[create_coll_metadata_ix],
                Some(&[&coll_mint_cookie.mint_authority]),
            )
            .await?;

        let master_edition_seeds = &[
            b"metadata".as_ref(),
            self.program_id.as_ref(),
            coll_mint_cookie.address.as_ref(),
            b"edition".as_ref(),
        ];
        let (master_edition_key, _) =
            Pubkey::find_program_address(master_edition_seeds, &self.program_id);

        let create_master_edition_ix = mpl_token_metadata::instruction::create_master_edition_v3(
            self.program_id,
            master_edition_key,
            coll_mint_cookie.address,
            update_authority,
            coll_mint_cookie.mint_authority.pubkey(),
            coll_metadata_key,
            payer,
            Some(0),
        );

        self.bench
            .process_transaction(
                &[create_master_edition_ix],
                Some(&[&coll_mint_cookie.mint_authority]),
            )
            .await?;

        Ok(NftCollectionCookie {
            mint: coll_mint_cookie.address,
            metadata: coll_metadata_key,
            master_edition: master_edition_key,
        })
    }

    #[allow(dead_code)]
    pub async fn with_nft_v2(
        &self,
        nft_collection_cookie: &NftCollectionCookie,
        nft_owner_cookie: &WalletCookie,
        args: Option<CreateNftArgs>,
    ) -> Result<NftCookie, TransportError> {
        let CreateNftArgs {
            verify_collection,
            amount,
        } = args.unwrap_or_default();

        // Crate NFT
        let mint_cookie = self.bench.with_mint().await?;
        let nft_account_cookie = self
            .bench
            .with_tokens(&mint_cookie, &nft_owner_cookie.address, amount)
            .await?;

        let metadata_seeds = &[
            b"metadata".as_ref(),
            self.program_id.as_ref(),
            &mint_cookie.address.as_ref(),
        ];
        let (metadata_key, _) = Pubkey::find_program_address(metadata_seeds, &self.program_id);

        let name = "TestNFT".to_string();
        let symbol = "NFT".to_string();
        let uri = "URI".to_string();

        let collection = Collection {
            verified: false,
            key: nft_collection_cookie.mint,
        };

        let create_metadata_ix = mpl_token_metadata::instruction::create_metadata_accounts_v3(
            self.program_id,
            metadata_key,
            mint_cookie.address,
            mint_cookie.mint_authority.pubkey(),
            self.bench.payer.pubkey(),
            self.bench.payer.pubkey(),
            name,
            symbol,
            uri,
            None,
            10,
            false,
            false,
            Some(collection),
            None,
            None,
        );

        self.bench
            .process_transaction(&[create_metadata_ix], Some(&[&mint_cookie.mint_authority]))
            .await?;

        if verify_collection {
            let verify_collection = mpl_token_metadata::instruction::verify_collection(
                self.program_id,
                metadata_key,
                self.bench.payer.pubkey(),
                self.bench.payer.pubkey(),
                nft_collection_cookie.mint,
                nft_collection_cookie.metadata,
                nft_collection_cookie.master_edition,
                None,
            );

            self.bench
                .process_transaction(&[verify_collection], None)
                .await?;
        }

        Ok(NftCookie {
            address: nft_account_cookie.address,
            metadata: metadata_key,
            mint_cookie,
        })
    }
}
