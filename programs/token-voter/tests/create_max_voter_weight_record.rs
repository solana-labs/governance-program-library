use crate::program_test::token_voter_test::TokenVoterTest;
use program_test::tools::assert_ix_err;
use solana_program::instruction::InstructionError;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_create_max_voter_weight_record_with_token_extensions() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new_token_extensions(None).await;
    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;

    // Act
    let max_voter_weight_record_cookie = token_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Assert

    let max_voter_weight_record = token_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    assert_eq!(
        max_voter_weight_record_cookie.account.max_voter_weight,
        max_voter_weight_record.max_voter_weight
    );
    assert_eq!(
        max_voter_weight_record_cookie.account.realm,
        max_voter_weight_record.realm
    );

    Ok(())
}

#[tokio::test]
async fn test_create_max_voter_weight_record() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new().await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;

    // Act
    let max_voter_weight_record_cookie = token_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Assert

    let max_voter_weight_record = token_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    assert_eq!(
        max_voter_weight_record_cookie.account.max_voter_weight,
        max_voter_weight_record.max_voter_weight
    );
    assert_eq!(
        max_voter_weight_record_cookie.account.realm,
        max_voter_weight_record.realm
    );

    Ok(())
}

#[tokio::test]
async fn test_create_max_voter_weight_record_with_already_exists_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new().await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;

    token_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    token_voter_test.bench.advance_clock().await;

    // Act
    let err = token_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await
        .err()
        .unwrap();

    // Assert

    // InstructionError::Custom(0) is returned for TransactionError::AccountInUse
    assert_ix_err(err, InstructionError::Custom(0));

    Ok(())
}
