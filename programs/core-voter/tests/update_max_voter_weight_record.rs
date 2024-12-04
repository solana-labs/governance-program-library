use crate::program_test::core_voter_test::ConfigureCollectionArgs;
use program_test::core_voter_test::CoreVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_update_collection_config_invalidates_max_voter_weight_record_expirey(
) -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let _voter_cookie = core_voter_test.bench.with_wallet().await;

    let mut max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let collection_1_size = 7;
    let collection_1_weight = 5;

    let collection_cookie_1 = core_voter_test
        .core
        .create_collection(Some(collection_1_size))
        .await?;

    // Register collection_1 to registrar
    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie_1,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: collection_1_weight,
            }),
        )
        .await?;

    // Generate an updated new max voter weight record
    core_voter_test
        .update_max_voter_weight_record(&registrar_cookie, &mut max_voter_weight_record_cookie)
        .await?;

    let collection_2_size = 10;
    let collection_2_weight = 2;

    // Generate a new collection and update the registrar with the additional collection
    // while invalidating max voter weight.
    let collection_cookie_2 = core_voter_test
        .core
        .create_collection(Some(collection_2_size))
        .await?;

    // Register collection_2 to registrar
    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie_2,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: collection_2_weight,
            }),
        )
        .await?;

    // Fetch registrar account and assert that collection was added to the registrars collection_configs
    let registrar = core_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    let max_voter_weight_record = core_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    core_voter_test.bench.advance_clock().await;
    let _clock = core_voter_test.bench.get_clock().await;

    // Assert
    let max_voter_weight_total =
        (collection_1_weight * collection_1_size) + (collection_2_weight * collection_2_size);

    assert!(registrar.collection_configs.len() == 2);
    assert!(max_voter_weight_record.max_voter_weight_expiry.is_none());
    assert!(max_voter_weight_record.max_voter_weight == max_voter_weight_total as u64);

    Ok(())
}

#[tokio::test]
async fn test_update_max_voter_weight_record_provides_valid_expirey() -> Result<(), TransportError>
{
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let mut max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let _voter_cookie = core_voter_test.bench.with_wallet().await;

    // Set collection sizes and weights for collection_1
    let collection_1_size = 11;
    let collection_1_weight = 4;

    let collection_cookie_1 = core_voter_test
        .core
        .create_collection(Some(collection_1_size))
        .await?;

    // Register collection_1 to registrar
    let _collection_config_cookie = core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie_1,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: collection_1_weight,
            }),
        )
        .await?;

    // Generate an updated new max voter weight record
    core_voter_test
        .update_max_voter_weight_record(&registrar_cookie, &mut max_voter_weight_record_cookie)
        .await?;

    // Advance clock so that second `update_max_voter_weight_record`` can be made without a duplicate
    // transaction submission which causes transaction to "pass" but not actually update the account.

    core_voter_test.bench.advance_clock().await;
    let _clock = core_voter_test.bench.get_clock().await;

    // Generate a new collection and update the registrar with the additional collection
    // which also invalidates max_voter_weight_expirey.

    let collection_2_size = 9;
    let collection_2_weight = 3;
    let collection_cookie_2 = core_voter_test
        .core
        .create_collection(Some(collection_2_size))
        .await?;

    // Register collection_2 to registrar
    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie_2,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: collection_2_weight,
            }),
        )
        .await?;

    // Revalidate max voter weight record by calling the update
    let _update_max_voter_weight_record_2 = core_voter_test
        .update_max_voter_weight_record(&registrar_cookie, &mut max_voter_weight_record_cookie)
        .await?;

    // Fetch registrar account and assert that collection was added to the registrars collection_configs
    let registrar = core_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    // Fetch max voter weight record and assert that max voter weight expiry is set
    let max_voter_weight_record = core_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    // Assert
    let max_voter_weight_total =
        (collection_1_weight * collection_1_size) + (collection_2_weight * collection_2_size);

    assert!(registrar.collection_configs.len() == 2);
    assert!(max_voter_weight_record.max_voter_weight == max_voter_weight_total as u64);
    assert!(max_voter_weight_record.max_voter_weight_expiry.is_some());

    Ok(())
}
