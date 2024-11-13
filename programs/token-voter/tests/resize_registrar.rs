mod program_test;
use program_test::token_voter_test::TokenVoterTest;

use crate::program_test::program_test_bench::MintType;
use crate::program_test::tools::assert_token_voter_err;
use gpl_token_voter::error::TokenVoterError;
use solana_program_test::*;
use solana_sdk::signer::Signer;
use solana_sdk::transport::TransportError;

#[tokio::test]
async fn test_resize_registrar() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new().await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    // Act
    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    // Act
    let registrar_cookie_resized = token_voter_test
        .with_resize_registrar(&realm_cookie, 2)
        .await?;

    // Assert
    let registrar = token_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar, registrar_cookie_resized.account);

    Ok(())
}

#[tokio::test]
async fn test_resize_registrar_with_configured_mint() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new().await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    // Act
    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let first_user_cookie = token_voter_test.users.first().unwrap();
    let mut mint_iter = token_voter_test.mints.iter();
    let first_mint_cookie = mint_iter.next().unwrap();
    let second_mint_cookie = mint_iter.next().unwrap();

    let _first_user_cookie_token_account = token_voter_test
        .bench
        .with_tokens(
            &first_mint_cookie,
            &first_user_cookie.key.pubkey(),
            100,
            &MintType::SplTokenExtensionsWithTransferFees,
            false,
        )
        .await;

    let max_voter_weight_record_cookie = token_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let _voting_mint_config = token_voter_test
        .configure_mint_config(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            &first_mint_cookie,
            0, // no digit shift
        )
        .await?;
    let _second_voting_mint_config = token_voter_test
        .configure_mint_config(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            &second_mint_cookie,
            0, // no digit shift
        )
        .await?;

    // Act
    let err = token_voter_test
        .with_resize_registrar(&realm_cookie, 2)
        .await
        .err()
        .unwrap();
    assert_token_voter_err(err, TokenVoterError::InvalidResizeMaxMints);

    let registrar_cookie_resized = token_voter_test
        .with_resize_registrar(&realm_cookie, 3)
        .await?;

    // Assert
    let registrar = token_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(
        registrar.max_mints,
        registrar_cookie_resized.account.max_mints
    );

    Ok(())
}
