use crate::program_test::core_voter_test::ConfigureCollectionArgs;
use gpl_core_voter::error::NftVoterError;
use gpl_core_voter::state::*;
use program_test::core_voter_test::CoreVoterTest;
use program_test::tools::*;
use solana_program_test::*;
use solana_sdk::msg;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_update_voter_weight_record() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let asset_cookie1 = core_voter_test
        .core
        .create_asset(&collection_cookie, &voter_cookie)
        .await?;

    msg!("Register the collection to the registrar");
    // Register the collection to the registrar
    let _collection_config_cookie = core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs { weight: 10 }),
        )
        .await?;

    let mut voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    core_voter_test.bench.advance_clock().await;
    let clock = core_voter_test.bench.get_clock().await;

    // Act
    core_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut voter_weight_record_cookie,
            VoterWeightAction::CreateProposal,
            &[&asset_cookie1],
        )
        .await?;

    // Assert

    let voter_weight_record = core_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 10);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));

    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CreateProposal.into())
    );
    assert_eq!(voter_weight_record.weight_action_target, None);

    Ok(())
}

#[tokio::test]
async fn test_update_voter_weight_with_multiple_nfts() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let asset_cookie1 = core_voter_test
        .core
        .create_asset(&collection_cookie, &voter_cookie)
        .await?;

    let asset_cookie2 = core_voter_test
        .core
        .create_asset(&collection_cookie, &voter_cookie)
        .await?;

    let _collection_config_cookie = core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs { weight: 10 }),
        )
        .await?;

    let mut voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    core_voter_test.bench.advance_clock().await;
    let clock = core_voter_test.bench.get_clock().await;

    // Act
    core_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut voter_weight_record_cookie,
            VoterWeightAction::CreateProposal,
            &[&asset_cookie1, &asset_cookie2],
        )
        .await?;

    // Assert

    let voter_weight_record = core_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 20);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CreateProposal.into())
    );
    assert_eq!(voter_weight_record.weight_action_target, None);

    Ok(())
}

#[tokio::test]
async fn test_update_voter_weight_with_cast_vote_not_allowed_error() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let asset_cookie1 = core_voter_test
        .core
        .create_asset(&collection_cookie, &voter_cookie)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs { weight: 10 }),
        )
        .await?;

    let mut voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    // Act
    let err = core_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut voter_weight_record_cookie,
            VoterWeightAction::CastVote,
            &[&asset_cookie1],
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::CastVoteIsNotAllowed);

    Ok(())
}

#[tokio::test]
async fn test_update_voter_weight_with_invalid_owner_error() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let voter_cookie2 = core_voter_test.bench.with_wallet().await;

    let asset_cookie1 = core_voter_test
        .core
        .create_asset(&collection_cookie, &voter_cookie2)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs { weight: 10 }),
        )
        .await?;

    let mut voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    // Act
    let err = core_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut voter_weight_record_cookie,
            VoterWeightAction::CreateGovernance,
            &[&asset_cookie1],
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::VoterDoesNotOwnNft);

    Ok(())
}

#[tokio::test]
async fn test_update_voter_weight_with_invalid_collection_error() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let collection_cookie2 = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let _random_asset_cookie = core_voter_test
        .core
        .create_asset(&collection_cookie2, &voter_cookie)
        .await?;

    let asset_cookie1 = core_voter_test
        .core
        .create_asset(&collection_cookie2, &voter_cookie)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs { weight: 10 }),
        )
        .await?;

    let mut voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    // Act
    let err = core_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut voter_weight_record_cookie,
            VoterWeightAction::CreateGovernance,
            &[&asset_cookie1],
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::CollectionNotFound);

    Ok(())
}

#[tokio::test]
async fn test_update_voter_weight_with_same_nft_error() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let asset_cookie = core_voter_test
        .core
        .create_asset(&collection_cookie, &voter_cookie)
        .await?;

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

    let mut voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    // Act
    let err = core_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut voter_weight_record_cookie,
            VoterWeightAction::CreateProposal,
            &[&asset_cookie, &asset_cookie],
        )
        .await
        .err()
        .unwrap();

    // Assert

    assert_nft_voter_err(err, NftVoterError::DuplicatedNftDetected);

    Ok(())
}
