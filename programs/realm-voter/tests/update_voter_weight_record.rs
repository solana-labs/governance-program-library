use crate::program_test::realm_voter_test::RealmVoterTest;
use gpl_realm_voter::{error::RealmVoterError, state::CollectionItemChangeType};
use program_test::tools::*;
use solana_program_test::*;
use solana_sdk::transport::TransportError;
mod program_test;

#[tokio::test]
async fn test_update_voter_weight_record() -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    // Create TokenOwnerRecord for other Realm
    let realm_cookie2 = realm_voter_test.governance.with_realm().await?;
    let token_owner_cookie = realm_voter_test.bench.with_wallet().await;
    let token_owner_record_cookie = realm_voter_test
        .governance
        .with_token_owner_record(&realm_cookie2, &token_owner_cookie)
        .await?;

    let governance_program_cookie = realm_voter_test.with_governance_program(None).await;

    realm_voter_test
        .configure_governance_program(
            &registrar_cookie,
            &governance_program_cookie,
            CollectionItemChangeType::Upsert,
        )
        .await?;

    let mut voter_weight_record_cookie = realm_voter_test
        .with_voter_weight_record(&registrar_cookie, &token_owner_cookie)
        .await?;

    let mut max_voter_weight_record_cookie = realm_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    realm_voter_test
        .configure_voter_weights(
            &registrar_cookie,
            &mut max_voter_weight_record_cookie,
            10,
            110,
        )
        .await?;

    let clock = realm_voter_test.bench.get_clock().await;

    // Act
    realm_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut voter_weight_record_cookie,
            &token_owner_record_cookie,
        )
        .await?;

    // Assert

    let voter_weight_record = realm_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 10);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(voter_weight_record.weight_action, None);
    assert_eq!(voter_weight_record.weight_action_target, None);

    Ok(())
}

#[tokio::test]
async fn test_update_voter_weight_record_with_token_owner_record_from_own_realm_not_allowed_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let token_owner_cookie = realm_voter_test.bench.with_wallet().await;
    let token_owner_record_cookie = realm_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &token_owner_cookie)
        .await?;

    let governance_program_cookie = realm_voter_test.with_governance_program(None).await;

    realm_voter_test
        .configure_governance_program(
            &registrar_cookie,
            &governance_program_cookie,
            CollectionItemChangeType::Upsert,
        )
        .await?;

    let mut voter_weight_record_cookie = realm_voter_test
        .with_voter_weight_record(&registrar_cookie, &token_owner_cookie)
        .await?;

    // Act
    let err = realm_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut voter_weight_record_cookie,
            &token_owner_record_cookie,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_realm_voter_err(err, RealmVoterError::TokenOwnerRecordFromOwnRealmNotAllowed);

    Ok(())
}

#[tokio::test]
async fn test_update_voter_weight_record_for_member_from_not_configured_governance_program_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    // Create TokenOwnerRecord for other Realm
    let realm_cookie2 = realm_voter_test.governance.with_realm().await?;
    let token_owner_cookie = realm_voter_test.bench.with_wallet().await;
    let token_owner_record_cookie = realm_voter_test
        .governance
        .with_token_owner_record(&realm_cookie2, &token_owner_cookie)
        .await?;

    let mut voter_weight_record_cookie = realm_voter_test
        .with_voter_weight_record(&registrar_cookie, &token_owner_cookie)
        .await?;

    // Act
    let err = realm_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut voter_weight_record_cookie,
            &token_owner_record_cookie,
        )
        .await
        .err()
        .unwrap();

    // Assert

    assert_realm_voter_err(err, RealmVoterError::GovernanceProgramNotConfigured);

    Ok(())
}

#[tokio::test]
async fn test_update_voter_weight_record_with_token_owner_record_must_match_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let governance_program_cookie = realm_voter_test.with_governance_program(None).await;

    realm_voter_test
        .configure_governance_program(
            &registrar_cookie,
            &governance_program_cookie,
            CollectionItemChangeType::Upsert,
        )
        .await?;

    // Create TokenOwnerRecord for other Realm
    let realm_cookie2 = realm_voter_test.governance.with_realm().await?;
    let token_owner_cookie = realm_voter_test.bench.with_wallet().await;
    let token_owner_record_cookie = realm_voter_test
        .governance
        .with_token_owner_record(&realm_cookie2, &token_owner_cookie)
        .await?;

    let token_owner_cookie2 = realm_voter_test.bench.with_wallet().await;

    let mut voter_weight_record_cookie = realm_voter_test
        .with_voter_weight_record(&registrar_cookie, &token_owner_cookie2)
        .await?;

    // Act
    let err = realm_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut voter_weight_record_cookie,
            &token_owner_record_cookie,
        )
        .await
        .err()
        .unwrap();

    // Assert

    assert_realm_voter_err(err, RealmVoterError::GoverningTokenOwnerMustMatch);

    Ok(())
}
