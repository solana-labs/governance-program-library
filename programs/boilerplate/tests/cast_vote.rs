use gpl_boilerplate::error::BoilerplateError;
use gpl_boilerplate::state::*;
use program_test::{dummy_voter_test::*, tools::assert_boilerplate_err};

use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_cast_vote() -> Result<(), TransportError> {
    println!(
        "***test_cast_vote",
    );
    // Arrange
    let mut dummy_voter_test = DummyVoterTest::start_new().await;

    let realm_cookie = dummy_voter_test.governance.with_realm().await?;

    let registrar_cookie = dummy_voter_test.with_registrar(&realm_cookie).await?;

    let max_voter_weight_record_cookie = dummy_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = dummy_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = dummy_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = dummy_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = dummy_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;


    dummy_voter_test.bench.advance_clock().await;
    let clock = dummy_voter_test.bench.get_clock().await;

    println!(
        "***ABOUT TO CAST VOTE",
    );
    
    // Act
    dummy_voter_test
        .cast_dummy_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            None,
        )
        .await?;

    // Assert
    let voter_weight_record = dummy_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 1);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        Some(proposal_cookie.address)
    );

    Ok(())
}

#[tokio::test]
async fn test_cast_vote_invalid_voter_error() -> Result<(), TransportError> {
    // Arrange
    let mut dummy_voter_test = DummyVoterTest::start_new().await;

    let realm_cookie = dummy_voter_test.governance.with_realm().await?;

    let registrar_cookie = dummy_voter_test.with_registrar(&realm_cookie).await?;

    let max_voter_weight_record_cookie = dummy_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = dummy_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = dummy_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = dummy_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = dummy_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let voter_cookie2 = dummy_voter_test.bench.with_wallet().await;

    // Act

    let err = dummy_voter_test
        .cast_dummy_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie2,
            &voter_token_owner_record_cookie,
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_boilerplate_err(err, BoilerplateError::InvalidTokenOwnerForVoterWeightRecord);

    Ok(())
}

#[tokio::test]
async fn test_cast_vote_using_multiple_instructions() -> Result<(), TransportError> {
    // Arrange
    let mut dummy_voter_test = DummyVoterTest::start_new().await;

    let realm_cookie = dummy_voter_test.governance.with_realm().await?;

    let registrar_cookie = dummy_voter_test.with_registrar(&realm_cookie).await?;

    let max_voter_weight_record_cookie = dummy_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = dummy_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = dummy_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = dummy_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = dummy_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    dummy_voter_test.bench.advance_clock().await;
    let clock = dummy_voter_test.bench.get_clock().await;

    let args = CastVoteArgs {
        cast_spl_gov_vote: false,
    };

    dummy_voter_test
        .cast_dummy_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            Some(args),
        )
        .await?;

    // Act

    dummy_voter_test
        .cast_dummy_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            None,
        )
        .await?;

    // Assert

    let voter_weight_record = dummy_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 2);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        Some(proposal_cookie.address)
    );

    Ok(())
}
