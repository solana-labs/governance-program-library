use anchor_lang::prelude::ErrorCode;
use program_test::{
    core_voter_test::CoreVoterTest,
    tools::{assert_anchor_err, assert_ix_err},
};
use solana_program::instruction::InstructionError;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_create_max_voter_weight_record() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    // Act
    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Assert
    
    let max_voter_weight_record = core_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    assert_eq!(
        max_voter_weight_record_cookie.account,
        max_voter_weight_record
    );

    Ok(())
}

#[tokio::test]
async fn test_create_max_voter_weight_record_with_invalid_realm_error() -> Result<(), TransportError>
{
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let realm_cookie2 = core_voter_test.governance.with_realm().await?;

    // Act
    let err = core_voter_test
        .with_max_voter_weight_record_using_ix(&registrar_cookie, |i| {
            i.accounts[2].pubkey = realm_cookie2.address // Realm
        })
        .await
        .err()
        .unwrap();

    // Assert

    // PDA doesn't match and hence the error is ConstraintSeeds
    assert_anchor_err(err, ErrorCode::ConstraintSeeds);

    Ok(())
}

#[tokio::test]
async fn test_create_max_voter_weight_record_with_invalid_mint_error() -> Result<(), TransportError>
{
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let realm_cookie2 = core_voter_test.governance.with_realm().await?;

    // Act
    let err = core_voter_test
        .with_max_voter_weight_record_using_ix(&registrar_cookie, |i| {
            i.accounts[2].pubkey = realm_cookie2.address // Mint
        })
        .await
        .err()
        .unwrap();

    // Assert

    // PDA doesn't match and hence the error is ConstraintSeeds
    assert_anchor_err(err, ErrorCode::ConstraintSeeds);

    Ok(())
}

#[tokio::test]
async fn test_create_max_voter_weight_record_with_already_exists_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    core_voter_test.bench.advance_clock().await;

    // Act
    let err = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await
        .err()
        .unwrap();

    // Assert

    // InstructionError::Custom(0) is returned for TransactionError::AccountInUse
    assert_ix_err(err, InstructionError::Custom(0));

    Ok(())
}
