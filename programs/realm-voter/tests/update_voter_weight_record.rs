use crate::program_test::realm_voter_test::RealmVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_update_voter_weight_record() -> Result<(), TransportError> {
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
        .with_governance_program_config(&registrar_cookie, &governance_program_cookie)
        .await?;

    let voter_cookie = realm_voter_test.bench.with_wallet().await;

    let mut voter_weight_record_cookie = realm_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
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
