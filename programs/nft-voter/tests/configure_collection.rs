use program_test::nft_voter_test::NftVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

use crate::program_test::nft_voter_test::ConfigureCollectionArgs;

mod program_test;

#[tokio::test]
async fn test_configure_collection() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Act
    let collection_config_cookie = nft_voter_test
        .with_configure_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            None,
        )
        .await?;

    // Assert
    let registrar = nft_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar.collection_configs.len(), 1);

    assert_eq!(
        registrar.collection_configs[0],
        collection_config_cookie.collection_config
    );

    let max_voter_weight_record = nft_voter_test
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
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie1 = nft_voter_test.token_metadata.with_nft_collection().await?;
    let nft_collection_cookie2 = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Act
    nft_voter_test
        .with_configure_collection(
            &registrar_cookie,
            &nft_collection_cookie1,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs { weight: 1, size: 5 }),
        )
        .await?;

    nft_voter_test
        .with_configure_collection(
            &registrar_cookie,
            &nft_collection_cookie2,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 2,
                size: 10,
            }),
        )
        .await?;

    // Assert
    let registrar = nft_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar.collection_configs.len(), 2);

    let max_voter_weight_record = nft_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    assert_eq!(max_voter_weight_record.max_voter_weight_expiry, None);
    assert_eq!(max_voter_weight_record.max_voter_weight, 25);

    Ok(())
}

#[tokio::test]
async fn test_configure_max_collections() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Act

    for _ in 0..registrar_cookie.max_collections {
        let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

        nft_voter_test
            .with_configure_collection(
                &registrar_cookie,
                &nft_collection_cookie,
                &max_voter_weight_record_cookie,
                None,
            )
            .await?;
    }

    // Assert
    let registrar = nft_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(
        registrar.collection_configs.len() as u8,
        registrar_cookie.max_collections
    );

    let max_voter_weight_record = nft_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    assert_eq!(max_voter_weight_record.max_voter_weight_expiry, None);
    assert_eq!(max_voter_weight_record.max_voter_weight, 30);

    Ok(())
}

// TODO: Check ream for registrar

// TODO: Check collection updated
// TODO: Remove collection
// TODO: Check MaxVoterWeight matches realm and mint on registrar
