use anchor_lang::Key;
use itertools::Either;
use gpl_gateway::{
    error::GatewayError,
    state::*
};
use program_test::{
    gateway_voter_test::GatewayVoterTest,
    tools::*
};
use solana_program_test::*;
use solana_sdk::transport::TransportError;
use spl_governance_addin_api::voter_weight::VoterWeightAction;

mod program_test;

const EXPECTED_VOTES: u64 = 1000000;

#[tokio::test]
async fn test_update_voter_weight_record_with_predecessor_voter_weight_record() -> Result<(), TransportError> {
    // Arrange
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;
    
    let (realm_cookie, registrar_cookie, gateway_token_cookie, voter_cookie) = gateway_voter_test.setup(true).await?;

    // the voter weight record from the registered predecessor plugin (will give a constant weight)
    let mut predecessor_voter_weight_record_cookie = gateway_voter_test.predecessor_plugin.with_voter_weight_record(
        &realm_cookie,
        &voter_cookie,
        EXPECTED_VOTES
    ).await?;
    
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
            VoterWeightAction::CreateProposal,
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
    assert_eq!(
        voter_weight_record.weight_action,
        None
    );
    assert_eq!(voter_weight_record.weight_action_target, None);

    Ok(())
}

#[tokio::test]
async fn test_cast_vote_with_update_voter_weight_record_and_predecessor() -> Result<(), TransportError> {
    // Arrange
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;

    let (realm_cookie, registrar_cookie, gateway_token_cookie, voter_cookie) = gateway_voter_test.setup(true).await?;

    // the voter weight record from the registered predecessor plugin (will give a constant weight)
    let mut predecessor_voter_weight_record_cookie = gateway_voter_test.predecessor_plugin.with_voter_weight_record(
        &realm_cookie,
        &voter_cookie,
        EXPECTED_VOTES
    ).await?;

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
    assert_eq!(
        voter_weight_record.weight_action,
        None
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        None
    );

    Ok(())
}