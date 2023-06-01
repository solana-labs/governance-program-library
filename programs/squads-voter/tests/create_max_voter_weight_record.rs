use crate::program_test::squads_voter_test::SquadsVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_create_max_voter_weight_record() -> Result<(), TransportError> {
    // Arrange
    let mut squads_voter_test = SquadsVoterTest::start_new().await;

    let realm_cookie = squads_voter_test.governance.with_realm().await?;

    let registrar_cookie = squads_voter_test.with_registrar(&realm_cookie).await?;

    // Act
    let max_voter_weight_record_cookie = squads_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Assert

    let max_voter_weight_record = squads_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    assert_eq!(
        max_voter_weight_record_cookie.account,
        max_voter_weight_record
    );

    Ok(())
}

// #[tokio::test]
// async fn test_create_max_voter_weight_record_with_invalid_realm_error() -> Result<(), TransportError>
// {
//     // Arrange
//     let mut squads_voter_test = NftVoterTest::start_new().await;

//     let realm_cookie = squads_voter_test.governance.with_realm().await?;

//     let registrar_cookie = squads_voter_test.with_registrar(&realm_cookie).await?;

//     let realm_cookie2 = squads_voter_test.governance.with_realm().await?;

//     // Act
//     let err = squads_voter_test
//         .with_max_voter_weight_record_using_ix(&registrar_cookie, |i| {
//             i.accounts[2].pubkey = realm_cookie2.address // Realm
//         })
//         .await
//         .err()
//         .unwrap();

//     // Assert

//     // PDA doesn't match and hence the error is PrivilegeEscalation
//     assert_ix_err(err, InstructionError::PrivilegeEscalation);

//     Ok(())
// }

// #[tokio::test]
// async fn test_create_max_voter_weight_record_with_invalid_mint_error() -> Result<(), TransportError>
// {
//     // Arrange
//     let mut squads_voter_test = NftVoterTest::start_new().await;

//     let realm_cookie = squads_voter_test.governance.with_realm().await?;

//     let registrar_cookie = squads_voter_test.with_registrar(&realm_cookie).await?;

//     let realm_cookie2 = squads_voter_test.governance.with_realm().await?;

//     // Act
//     let err = squads_voter_test
//         .with_max_voter_weight_record_using_ix(&registrar_cookie, |i| {
//             i.accounts[2].pubkey = realm_cookie2.address // Mint
//         })
//         .await
//         .err()
//         .unwrap();

//     // Assert

//     // PDA doesn't match and hence the error is PrivilegeEscalation
//     assert_ix_err(err, InstructionError::PrivilegeEscalation);

//     Ok(())
// }

// #[tokio::test]
// async fn test_create_max_voter_weight_record_with_already_exists_error(
// ) -> Result<(), TransportError> {
//     // Arrange
//     let mut squads_voter_test = NftVoterTest::start_new().await;

//     let realm_cookie = squads_voter_test.governance.with_realm().await?;

//     let registrar_cookie = squads_voter_test.with_registrar(&realm_cookie).await?;

//     squads_voter_test
//         .with_max_voter_weight_record(&registrar_cookie)
//         .await?;

//     squads_voter_test.bench.advance_clock().await;

//     // Act
//     let err = squads_voter_test
//         .with_max_voter_weight_record(&registrar_cookie)
//         .await
//         .err()
//         .unwrap();

//     // Assert

//     // InstructionError::Custom(0) is returned for TransactionError::AccountInUse
//     assert_ix_err(err, InstructionError::Custom(0));

//     Ok(())
// }
