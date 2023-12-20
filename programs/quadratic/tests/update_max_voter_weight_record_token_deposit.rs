use crate::program_test::quadratic_voter_test::QuadraticVoterTest;
use crate::Either::Right;
use gpl_quadratic::error::QuadraticError;
use gpl_quadratic::state::QuadraticCoefficients;
use itertools::Either;
use program_test::tools::*;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

const INITIAL_VOTES: u64 = 1000000;
const EXPECTED_VOTES: u64 = 1000; // Square root of 1,000,000

#[tokio::test]
async fn test_update_max_voter_weight_record_with_mint_as_predecessor() -> Result<(), TransportError>
{
    // Arrange
    let mut quadratic_max_voter_test = QuadraticVoterTest::start_new().await;

    let (realm_cookie, registrar_cookie, voter_cookie) = quadratic_max_voter_test
        .setup(false, &QuadraticCoefficients::default())
        .await?;

    let mut max_voter_weight_record_cookie = quadratic_max_voter_test
        .with_max_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    quadratic_max_voter_test.bench.advance_clock().await;
    let clock = quadratic_max_voter_test.bench.get_clock().await;

    // using the mint as the max voter weight
    let max_voter_token_owner_record_cookie = realm_cookie.community_mint_cookie;

    // mint some tokens so that there is a max weight
    quadratic_max_voter_test
        .bench
        .with_tokens(
            &max_voter_token_owner_record_cookie,
            &voter_cookie.address,
            INITIAL_VOTES,
        )
        .await?;

    // Act
    quadratic_max_voter_test
        .update_max_voter_weight_record(
            &registrar_cookie,
            &mut Either::Right(&max_voter_token_owner_record_cookie),
            &mut max_voter_weight_record_cookie,
        )
        .await?;

    // Assert
    let max_voter_weight_record = quadratic_max_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    assert_eq!(max_voter_weight_record.max_voter_weight, EXPECTED_VOTES);
    assert_eq!(
        max_voter_weight_record.max_voter_weight_expiry,
        Some(clock.slot)
    );

    Ok(())
}
