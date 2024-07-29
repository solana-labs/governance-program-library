use std::{str::FromStr, sync::Arc};

use anchor_lang::prelude::Pubkey;
use solana_program_test::ProgramTest;
use solana_sdk::{signer::Signer, system_program, transport::TransportError, signature::Keypair};

use crate::program_test::program_test_bench::{ProgramTestBench, WalletCookie};

pub struct AssetCookie {
    pub asset: Pubkey,
}

pub struct CollectionCookie {
    pub collection: Pubkey,
    pub authority: Keypair,
}

pub struct CoreTest {
    pub bench: Arc<ProgramTestBench>,
    pub program_id: Pubkey,
}

impl CoreTest {
    pub fn program_id() -> Pubkey {
        Pubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap()
    }

    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("mpl_core", Self::program_id(), None);
    }

    #[allow(dead_code)]
    pub fn new(bench: Arc<ProgramTestBench>) -> Self {
        CoreTest {
            bench,
            program_id: Self::program_id(),
        }
    }

    #[allow(dead_code)]
    pub async fn with_collection(&self) -> Result<CollectionCookie, TransportError> {
        let payer = self.bench.context.borrow().payer.pubkey();

        // Create collection
        let coll_keypair = Keypair::new();
        let coll_authority = Keypair::new();

        let coll_name = "NFT_C".to_string();
        let coll_uri = "URI".to_string();

        // instruction args
        let args = mpl_core::instructions::CreateCollectionV2InstructionArgs {
            name: coll_name,
            uri: coll_uri,
            plugins: None,
            external_plugin_adapters: None,
        };

        // instruction accounts
        let create_coll_ix_accounts = mpl_core::instructions::CreateCollectionV2 {
            collection: coll_keypair.pubkey(),
            update_authority: Some(coll_authority.pubkey()),
            payer,
            system_program: system_program::ID,
        };

        // creates the instruction
        let create_coll_ix = create_coll_ix_accounts.instruction(args);

        self.bench
            .process_transaction(
                &[create_coll_ix],
                Some(&[&coll_keypair, &coll_authority]),
            )
            .await?;

        Ok(CollectionCookie {
            collection: coll_keypair.pubkey(),
            authority: coll_authority,
        })
    }

    #[allow(dead_code)]
    pub async fn with_asset(
        &self,
        collection_cookie: &CollectionCookie,
        asset_owner_cookie: &WalletCookie,
        collection: Option<Pubkey>
    ) -> Result<AssetCookie, TransportError> {
        let payer = self.bench.context.borrow().payer.pubkey();

        // Create Asset
        let asset_keypair = Keypair::new();

        let name = "TestAsset".to_string();
        let uri = "URI".to_string();

        // instruction args
        let args = mpl_core::instructions::CreateV2InstructionArgs {
            data_state: mpl_core::types::DataState::AccountState,
            name,
            uri,
            plugins: None,
            external_plugin_adapters: None,
        };

        // instruction accounts
        let create_accounts = mpl_core::instructions::CreateV2 {
            asset: asset_keypair.pubkey(),
            collection,
            authority: Some(collection_cookie.authority.pubkey()),
            payer,
            owner: Some(asset_owner_cookie.address),
            update_authority: None,
            system_program: system_program::ID,
            log_wrapper: None
        };

        // creates the instruction
        let create_ix = create_accounts.instruction(args);

        self.bench
            .process_transaction(
                &[create_ix],
                Some(&[&asset_keypair, &collection_cookie.authority]),
            )
            .await?;

        Ok(AssetCookie {
            asset: asset_keypair.pubkey(),
        })
    }
}
