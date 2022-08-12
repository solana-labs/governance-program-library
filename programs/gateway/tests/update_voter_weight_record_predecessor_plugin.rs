use crate::program_test::governance_test::RealmCookie;
use itertools::Either;
use program_test::{gateway_voter_test::GatewayVoterTest, tools::*};
use solana_program::instruction::InstructionError;
use solana_program_test::*;
use solana_sdk::transport::TransportError;
use spl_governance::state::realm::RealmV2;

mod program_test;

const EXPECTED_VOTES: u64 = 1000000;

#[tokio::test]
async fn test_update_voter_weight_record_with_predecessor_voter_weight_record(
) -> Result<(), TransportError> {
    // Arrange
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;

    let (realm_cookie, registrar_cookie, _, gateway_token_cookie, voter_cookie) =
        gateway_voter_test.setup(true).await?;

    // the voter weight record from the registered predecessor plugin (will give a constant weight)
    let predecessor_voter_weight_record_cookie = gateway_voter_test
        .predecessor_plugin
        .with_voter_weight_record(&realm_cookie, &voter_cookie, EXPECTED_VOTES)
        .await?;

    // the voter weight record from the gateway plugin (will pass-through or reject the predecessor weight)
    let mut gateway_voter_weight_record_cookie = gateway_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    gateway_voter_test.bench.advance_clock().await;
    let clock = gateway_voter_test.bench.get_clock().await;

    // Act
    gateway_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut Either::Left(&predecessor_voter_weight_record_cookie),
            &mut gateway_voter_weight_record_cookie,
            &gateway_token_cookie,
        )
        .await?;

    // Assert

    let voter_weight_record = gateway_voter_test
        .get_voter_weight_record(&gateway_voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, EXPECTED_VOTES);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));

    // The gateway plugin by default does not register a weight action if used
    // with no predecessor plugin
    assert_eq!(voter_weight_record.weight_action, None);
    assert_eq!(voter_weight_record.weight_action_target, None);

    Ok(())
}

#[tokio::test]
async fn test_update_fails_with_predecessor_from_different_realm() -> Result<(), TransportError> {
    // Arrange
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;

    let (realm_cookie, registrar_cookie, _, gateway_token_cookie, voter_cookie) =
        gateway_voter_test.setup(true).await?;

    let different_realm_cookie = RealmCookie {
        address: Default::default(),
        ..realm_cookie
    };

    // the voter weight record from the registered predecessor plugin (will give a constant weight)
    let predecessor_voter_weight_record_cookie = gateway_voter_test
        .predecessor_plugin
        .with_voter_weight_record(&different_realm_cookie, &voter_cookie, EXPECTED_VOTES)
        .await?;

    // the voter weight record from the gateway plugin (will pass-through or reject the predecessor weight)
    let mut gateway_voter_weight_record_cookie = gateway_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    gateway_voter_test.bench.advance_clock().await;

    // Act
    let err = gateway_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut Either::Left(&predecessor_voter_weight_record_cookie),
            &mut gateway_voter_weight_record_cookie,
            &gateway_token_cookie,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_ix_err(err, InstructionError::Custom(6004));

    Ok(())
}

#[tokio::test]
async fn test_update_fails_with_predecessor_for_different_governance() -> Result<(), TransportError>
{
    // Arrange
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;

    let (realm_cookie, registrar_cookie, _, gateway_token_cookie, voter_cookie) =
        gateway_voter_test.setup(true).await?;

    let different_community_mint_cookie = gateway_voter_test.bench.with_mint().await?;
    let different_realm_account = RealmV2 {
        community_mint: different_community_mint_cookie.address,
        ..realm_cookie.account
    };
    let different_token_realm_cookie = RealmCookie {
        account: different_realm_account,
        community_mint_cookie: different_community_mint_cookie,
        ..realm_cookie
    };

    // the voter weight record from the registered predecessor plugin (will give a constant weight)
    let predecessor_voter_weight_record_cookie = gateway_voter_test
        .predecessor_plugin
        .with_voter_weight_record(&different_token_realm_cookie, &voter_cookie, EXPECTED_VOTES)
        .await?;

    // the voter weight record from the gateway plugin (will pass-through or reject the predecessor weight)
    let mut gateway_voter_weight_record_cookie = gateway_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    gateway_voter_test.bench.advance_clock().await;

    // Act
    let err = gateway_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut Either::Left(&predecessor_voter_weight_record_cookie),
            &mut gateway_voter_weight_record_cookie,
            &gateway_token_cookie,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_ix_err(err, InstructionError::Custom(6005));

    Ok(())
}

#[tokio::test]
async fn test_update_fails_with_predecessor_with_a_different_owner() -> Result<(), TransportError> {
    // Arrange
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;

    let (realm_cookie, registrar_cookie, _, gateway_token_cookie, voter_cookie) =
        gateway_voter_test.setup(true).await?;

    let different_voter_cookie = gateway_voter_test.bench.with_wallet().await;

    // the voter weight record from the registered predecessor plugin (will give a constant weight)
    let predecessor_voter_weight_record_cookie = gateway_voter_test
        .predecessor_plugin
        .with_voter_weight_record(&realm_cookie, &different_voter_cookie, EXPECTED_VOTES)
        .await?;

    // the voter weight record from the gateway plugin (will pass-through or reject the predecessor weight)
    let mut gateway_voter_weight_record_cookie = gateway_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    gateway_voter_test.bench.advance_clock().await;

    // Act
    let err = gateway_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut Either::Left(&predecessor_voter_weight_record_cookie),
            &mut gateway_voter_weight_record_cookie,
            &gateway_token_cookie,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_ix_err(err, InstructionError::Custom(6006));

    Ok(())
}

#[tokio::test]
async fn test_cast_vote_with_update_voter_weight_record_and_predecessor(
) -> Result<(), TransportError> {
    // Arrange
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;

    let (realm_cookie, registrar_cookie, _, gateway_token_cookie, voter_cookie) =
        gateway_voter_test.setup(true).await?;

    // the voter weight record from the registered predecessor plugin (will give a constant weight)
    let predecessor_voter_weight_record_cookie = gateway_voter_test
        .predecessor_plugin
        .with_voter_weight_record(&realm_cookie, &voter_cookie, EXPECTED_VOTES)
        .await?;

    let voter_token_owner_record_cookie = gateway_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie, EXPECTED_VOTES)
        .await?;

    let gateway_voter_weight_record_cookie = gateway_voter_test
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
            &gateway_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &gateway_token_cookie,
            &voter_token_owner_record_cookie,
            &mut Either::Left(&predecessor_voter_weight_record_cookie),
            None,
        )
        .await?;

    // Assert
    let voter_weight_record = gateway_voter_test
        .get_voter_weight_record(&gateway_voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, EXPECTED_VOTES);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));

    // The gateway plugin by default does not register a weight action or target if used
    // with no predecessor plugin
    assert_eq!(voter_weight_record.weight_action, None);
    assert_eq!(voter_weight_record.weight_action_target, None);

    Ok(())
}
