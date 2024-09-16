use gpl_core_voter::error::NftVoterError;
use program_test::{
    core_voter_test::CoreVoterTest,
    tools::{assert_anchor_err, assert_nft_voter_err},
};

use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, transport::TransportError};

use crate::program_test::core_voter_test::ConfigureCollectionArgs;

mod program_test;

#[tokio::test]
async fn test_configure_collection() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let _asset_cookie = core_voter_test
        .core
        .create_asset(&collection_cookie, &voter_cookie)
        .await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Act
    let collection_config_cookie = core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs { weight: 1 }),
        )
        .await?;

    // Assert
    let registrar = core_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar.collection_configs.len(), 1);

    assert_eq!(
        registrar.collection_configs[0],
        collection_config_cookie.collection_config
    );

    let max_voter_weight_record = core_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    assert_eq!(max_voter_weight_record.max_voter_weight_expiry, None);
    assert_eq!(
        max_voter_weight_record.max_voter_weight,
        (registrar.collection_configs[0].weight as u32 * registrar.collection_configs[0].size)
            as u64
    );

    Ok(())
}

#[tokio::test]
async fn test_configure_multiple_collections() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie1 = core_voter_test.core.create_collection(None).await?;
    let collection_cookie2 = core_voter_test.core.create_collection(None).await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    for _ in 0..5 {
        let _asset_cookie = core_voter_test
            .core
            .create_asset(&collection_cookie1, &voter_cookie)
            .await?;
    }

    for _ in 0..10 {
        let _asset_cookie = core_voter_test
            .core
            .create_asset(&collection_cookie2, &voter_cookie)
            .await?;
    }

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Act
    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie1,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs { weight: 1 }),
        )
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie2,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs { weight: 2 }),
        )
        .await?;

    // Assert
    let registrar = core_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar.collection_configs.len(), 2);

    let max_voter_weight_record = core_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    assert_eq!(max_voter_weight_record.max_voter_weight_expiry, None);
    assert_eq!(max_voter_weight_record.max_voter_weight, 25);

    Ok(())
}

#[tokio::test]
async fn test_configure_max_collections() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    
    let _voter_cookie = core_voter_test.bench.with_wallet().await;

    for _ in 0..registrar_cookie.max_collections {
        let collection_cookie = core_voter_test.core.create_collection(Some(3)).await?;


        core_voter_test
            .with_collection(
                &registrar_cookie,
                &collection_cookie,
                &max_voter_weight_record_cookie,
                Some(ConfigureCollectionArgs { weight: 1 }),
            )
            .await?;
    }

    // Assert
    let registrar = core_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(
        registrar.collection_configs.len() as u8,
        registrar_cookie.max_collections
    );

    let max_voter_weight_record = core_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    assert_eq!(max_voter_weight_record.max_voter_weight_expiry, None);
    assert_eq!(max_voter_weight_record.max_voter_weight, 30);

    Ok(())
}

#[tokio::test]
async fn test_configure_existing_collection() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(Some(10)).await?;

    let _voter_cookie = core_voter_test.bench.with_wallet().await;


    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            None,
        )
        .await?;

    // Act

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs { weight: 2 }),
        )
        .await?;

    // Assert
    let registrar = core_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar.collection_configs.len(), 1);

    let max_voter_weight_record = core_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    assert_eq!(max_voter_weight_record.max_voter_weight_expiry, None);
    assert_eq!(max_voter_weight_record.max_voter_weight, 20);

    Ok(())
}

// TODO: Remove collection test

#[tokio::test]
async fn test_configure_collection_with_invalid_realm_error() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let _asset_cookie = core_voter_test
        .core
        .create_asset(&collection_cookie, &voter_cookie)
        .await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Try to use a different Realm
    let realm_cookie2 = core_voter_test.governance.with_realm().await?;

    // Act
    let err = core_voter_test
        .with_collection_using_ix(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            None,
            |i| i.accounts[1].pubkey = realm_cookie2.address, // realm
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert

    assert_nft_voter_err(err, NftVoterError::InvalidRealmForRegistrar);

    Ok(())
}

#[tokio::test]
async fn test_configure_collection_with_realm_authority_must_sign_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let _asset_cookie = core_voter_test
        .core
        .create_asset(&collection_cookie, &voter_cookie)
        .await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Act
    let err = core_voter_test
        .with_collection_using_ix(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            None,
            |i| i.accounts[2].is_signer = false, // realm_authority
            Some(&[]),
        )
        .await
        .err()
        .unwrap();

    // Assert

    assert_anchor_err(err, anchor_lang::error::ErrorCode::AccountNotSigner);

    Ok(())
}

#[tokio::test]
async fn test_configure_collection_with_invalid_realm_authority_error() -> Result<(), TransportError>
{
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(Some(1)).await?;

    // let voter_cookie = core_voter_test.bench.with_wallet().await;

    // let _asset_cookie = core_voter_test
    //     .core
    //     .create_asset(&collection_cookie, &voter_cookie)
    //     .await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let realm_authority = Keypair::new();

    // Act
    let err = core_voter_test
        .with_collection_using_ix(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            None,
            |i| i.accounts[2].pubkey = realm_authority.pubkey(), // realm_authority
            Some(&[&realm_authority]),
        )
        .await
        .err()
        .unwrap();

    // Assert

    assert_nft_voter_err(err, NftVoterError::InvalidRealmAuthority);

    Ok(())
}

#[tokio::test]
async fn test_configure_collection_with_invalid_max_voter_weight_realm_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;
    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(Some(1)).await?;

    // let voter_cookie = core_voter_test.bench.with_wallet().await;

    // let _asset_cookie = core_voter_test
    //     .core
    //     .create_asset(&collection_cookie, &voter_cookie)
    //     .await?;

    let realm_cookie2 = core_voter_test.governance.with_realm().await?;
    let registrar_cookie2 = core_voter_test.with_registrar(&realm_cookie2).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie2)
        .await?;

    // Act
    let err = core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert

    assert_nft_voter_err(err, NftVoterError::InvalidMaxVoterWeightRecordRealm);

    Ok(())
}

#[tokio::test]
async fn test_configure_collection_with_invalid_max_voter_weight_mint_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let mut realm_cookie = core_voter_test.governance.with_realm().await?;
    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(Some(1)).await?;

    // let voter_cookie = core_voter_test.bench.with_wallet().await;

    // let _asset_cookie = core_voter_test
    //     .core
    //     .create_asset(&collection_cookie, &voter_cookie)
    //     .await?;

    // Create Registrar for council mint
    realm_cookie.account.community_mint = realm_cookie.account.config.council_mint.unwrap();
    let registrar_cookie2 = core_voter_test.with_registrar(&realm_cookie).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie2)
        .await?;

    // Act
    let err = core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert

    assert_nft_voter_err(err, NftVoterError::InvalidMaxVoterWeightRecordMint);

    Ok(())
}
