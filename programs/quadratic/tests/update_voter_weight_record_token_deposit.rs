use gpl_quadratic::error::QuadraticError;
use gpl_quadratic::state::QuadraticCoefficients;
use itertools::Either;
use program_test::quadratic_voter_test::QuadraticVoterTest;
use program_test::tools::*;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

const INITIAL_VOTES: u64 = 1000000;
const EXPECTED_VOTES: u64 = 1000; // Square root of 1,000,000

#[tokio::test]
async fn test_update_voter_weight_record_with_token_owner_record_as_input(
) -> Result<(), TransportError> {
    // Arrange
    let mut quadratic_voter_test = QuadraticVoterTest::start_new().await;

    let (realm_cookie, registrar_cookie, voter_cookie) = quadratic_voter_test
        .setup(false, &QuadraticCoefficients::default())
        .await?;

    let mut voter_weight_record_cookie = quadratic_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    quadratic_voter_test.bench.advance_clock().await;
    let clock = quadratic_voter_test.bench.get_clock().await;

    let voter_token_owner_record_cookie = quadratic_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie, INITIAL_VOTES)
        .await?;

    // Act
    quadratic_voter_test
        .update_voter_weight_record(
            &registrar_cookie,
            &mut Either::Right(&voter_token_owner_record_cookie),
            &mut voter_weight_record_cookie,
        )
        .await?;

    // Assert
    let voter_weight_record = quadratic_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, EXPECTED_VOTES);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));

    // The quadratic plugin by default does not register a weight action if used
    // with no predecessor plugin
    assert_eq!(
        voter_weight_record.weight_action,
        None // Some(VoterWeightAction::CreateProposal.into())
    );
    assert_eq!(voter_weight_record.weight_action_target, None);

    Ok(())
}

#[tokio::test]
async fn test_cast_vote_with_update_voter_weight_record_default_coefficients(
) -> Result<(), TransportError> {
    // Arrange
    let mut quadratic_voter_test = QuadraticVoterTest::start_new().await;
    let (realm_cookie, registrar_cookie, voter_cookie) = quadratic_voter_test
        .setup(false, &QuadraticCoefficients::default())
        .await?;

    let voter_token_owner_record_cookie = quadratic_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie, INITIAL_VOTES)
        .await?;

    let voter_weight_record_cookie = quadratic_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = quadratic_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    quadratic_voter_test.bench.advance_clock().await;
    let clock = quadratic_voter_test.bench.get_clock().await;

    // Act
    quadratic_voter_test
        .cast_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &mut Either::Right(&voter_token_owner_record_cookie),
            None,
        )
        .await?;

    // Assert
    let voter_weight_record = quadratic_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, EXPECTED_VOTES);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));

    // The quadratic plugin by default does not register a weight action or target if used
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

#[tokio::test]
async fn test_cast_vote_with_update_voter_weight_record_passthrough_coefficients(
) -> Result<(), TransportError> {
    // Arrange
    let passthrough_coefficients = QuadraticCoefficients {
        a: 0.0, // 0x^2
        b: 1.0, // 1x
        c: 0.0, // 0
    };
    let mut quadratic_voter_test = QuadraticVoterTest::start_new().await;
    let (realm_cookie, registrar_cookie, voter_cookie) = quadratic_voter_test
        .setup(false, &passthrough_coefficients)
        .await?;

    let voter_token_owner_record_cookie = quadratic_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie, INITIAL_VOTES)
        .await?;

    let voter_weight_record_cookie = quadratic_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = quadratic_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    quadratic_voter_test.bench.advance_clock().await;
    let clock = quadratic_voter_test.bench.get_clock().await;

    // Act
    quadratic_voter_test
        .cast_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &mut Either::Right(&voter_token_owner_record_cookie),
            None,
        )
        .await?;

    // Assert
    let voter_weight_record = quadratic_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    // the vote weight was passed through
    assert_eq!(voter_weight_record.voter_weight, INITIAL_VOTES);

    Ok(())
}

#[tokio::test]
async fn test_cast_vote_with_update_voter_weight_record_one_vote_coefficients(
) -> Result<(), TransportError> {
    // Arrange
    // These coefficients count each vote as having weight 1, regardless of how many tokens the voter has
    let single_vote_coefficients = QuadraticCoefficients {
        a: 0.0, // 0x^2
        b: 0.0, // 0x
        c: 1.0, // 1
    };
    let mut quadratic_voter_test = QuadraticVoterTest::start_new().await;
    let (realm_cookie, registrar_cookie, voter_cookie) = quadratic_voter_test
        .setup(false, &single_vote_coefficients)
        .await?;

    let voter_token_owner_record_cookie = quadratic_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie, INITIAL_VOTES)
        .await?;

    let voter_weight_record_cookie = quadratic_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = quadratic_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    quadratic_voter_test.bench.advance_clock().await;
    let clock = quadratic_voter_test.bench.get_clock().await;

    // Act
    quadratic_voter_test
        .cast_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &mut Either::Right(&voter_token_owner_record_cookie),
            None,
        )
        .await?;

    // Assert
    let voter_weight_record = quadratic_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    // the vote weight was set to 1
    assert_eq!(voter_weight_record.voter_weight, 1);

    Ok(())
}
