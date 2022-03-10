use program_test::nft_voter_test::NftVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_configure_collection() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Act
    let collection_config_cookie = nft_voter_test
        .with_configure_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
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

    Ok(())
}

// TODO: Check ream for registrar
// TODO: Check collection added
// TODO: Check collection updated
// TODO: Remove collection
// TODO: Check MaxVoterWeight matches realm and mint on registrar
