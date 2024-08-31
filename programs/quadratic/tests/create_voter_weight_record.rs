use gpl_quadratic::state::QuadraticCoefficients;
use program_test::quadratic_voter_test::QuadraticVoterTest;
use program_test::tools::assert_ix_err;
use solana_program::instruction::InstructionError;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_create_voter_weight_record() -> Result<(), TransportError> {
    // Arrange
    let mut quadratic_voter_test = QuadraticVoterTest::start_new().await;

    let realm_cookie = quadratic_voter_test.governance.with_realm().await?;

    let registrar_cookie = quadratic_voter_test
        .with_registrar(&realm_cookie, &QuadraticCoefficients::default(), None)
        .await?;

    let voter_cookie = quadratic_voter_test.bench.with_wallet().await;

    // Act
    let voter_weight_record_cookie = quadratic_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    // Assert

    let voter_weight_record = quadratic_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record_cookie.account, voter_weight_record);

    Ok(())
}

#[tokio::test]
async fn test_create_voter_weight_record_with_already_exists_error() -> Result<(), TransportError> {
    // Arrange
    let mut quadratic_voter_test = QuadraticVoterTest::start_new().await;

    let realm_cookie = quadratic_voter_test.governance.with_realm().await?;

    let registrar_cookie = quadratic_voter_test
        .with_registrar(&realm_cookie, &QuadraticCoefficients::default(), None)
        .await?;

    let voter_cookie = quadratic_voter_test.bench.with_wallet().await;

    quadratic_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    quadratic_voter_test.bench.advance_clock().await;

    // Act
    let err = quadratic_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await
        .err()
        .unwrap();

    // Assert

    // InstructionError::Custom(0) is returned for TransactionError::AccountInUse
    assert_ix_err(err, InstructionError::Custom(0));

    Ok(())
}
