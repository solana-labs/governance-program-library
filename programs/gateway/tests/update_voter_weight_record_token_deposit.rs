use gpl_civic_gateway::error::GatewayError;
use itertools::Either;
use program_test::gateway_voter_test::GatewayVoterTest;
use program_test::tools::*;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

const EXPECTED_VOTES: u64 = 1000000;

#[tokio::test]
async fn test_update_voter_weight_record_with_token_owner_record_as_input(
) -> Result<(), TransportError> {
    // Arrange
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;

    let (realm_cookie, registrar_cookie, _, gateway_token_cookie, voter_cookie) =
        gateway_voter_test.setup(false).await?;

    let mut voter_weight_record_cookie = gateway_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    gateway_voter_test.bench.advance_clock().await;
    let clock = gateway_voter_test.bench.get_clock().await;

    let voter_token_owner_record_cookie = gateway_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie, EXPECTED_VOTES)
        .await?;

    // Act
    gateway_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut Either::Right(&voter_token_owner_record_cookie),
            &mut voter_weight_record_cookie,
            &gateway_token_cookie,
        )
        .await?;

    // Assert

    let voter_weight_record = gateway_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, EXPECTED_VOTES);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));

    // The gateway plugin by default does not register a weight action if used
    // with no predecessor plugin
    assert_eq!(
        voter_weight_record.weight_action,
        None // Some(VoterWeightAction::CreateProposal.into())
    );
    assert_eq!(voter_weight_record.weight_action_target, None);

    Ok(())
}

#[tokio::test]
async fn test_update_voter_weight_record_with_invalid_gateway_token_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;
    let (realm_cookie, registrar_cookie, _, _, voter_cookie) =
        gateway_voter_test.setup(false).await?;

    let different_gateway_cookie = gateway_voter_test.with_gateway().await?;
    let invalid_gateway_token_cookie = gateway_voter_test
        .with_gateway_token(&different_gateway_cookie, &voter_cookie)
        .await?;

    let mut voter_weight_record_cookie = gateway_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    gateway_voter_test.bench.advance_clock().await;

    let voter_token_owner_record_cookie = gateway_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie, EXPECTED_VOTES)
        .await?;

    // Act
    let err = gateway_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut Either::Right(&voter_token_owner_record_cookie),
            &mut voter_weight_record_cookie,
            &invalid_gateway_token_cookie,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_gateway_err(err, GatewayError::InvalidGatewayToken);

    Ok(())
}

#[tokio::test]
async fn test_cast_vote_with_update_voter_weight_record() -> Result<(), TransportError> {
    // Arrange
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;
    let (realm_cookie, registrar_cookie, _, gateway_token_cookie, voter_cookie) =
        gateway_voter_test.setup(false).await?;

    let voter_token_owner_record_cookie = gateway_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie, EXPECTED_VOTES)
        .await?;

    let voter_weight_record_cookie = gateway_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = gateway_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    gateway_voter_test.bench.advance_clock().await;
    let clock = gateway_voter_test.bench.get_clock().await;

    // Act
    gateway_voter_test
        .cast_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &gateway_token_cookie,
            &voter_token_owner_record_cookie,
            &mut Either::Right(&voter_token_owner_record_cookie),
            None,
        )
        .await?;

    // Assert
    let voter_weight_record = gateway_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, EXPECTED_VOTES);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));

    // The gateway plugin by default does not register a weight action or target if used
    // with no predecessor plugin
    assert_eq!(
        voter_weight_record.weight_action,
        None // Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        None // Some(proposal_cookie.address)
    );

    Ok(())
}
