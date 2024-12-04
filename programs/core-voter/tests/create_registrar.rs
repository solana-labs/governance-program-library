mod program_test;

use anchor_lang::prelude::{ErrorCode, Pubkey};
use gpl_core_voter::error::NftVoterError;
use program_test::core_voter_test::CoreVoterTest;

use solana_program_test::*;
use solana_sdk::{signature::Keypair, transport::TransportError};

use program_test::tools::{assert_anchor_err, assert_nft_voter_err};

#[tokio::test]
async fn test_create_registrar() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    // Act
    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    // Assert
    let registrar = core_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar, registrar_cookie.account);

    Ok(())
}

#[tokio::test]
async fn test_create_registrar_with_invalid_realm_authority_error() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let mut realm_cookie = core_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    // Act
    let err = core_voter_test
        .with_registrar(&realm_cookie)
        .await
        .err()
        .unwrap();

    assert_nft_voter_err(err, NftVoterError::InvalidRealmAuthority);

    Ok(())
}

#[tokio::test]
async fn test_create_registrar_with_realm_authority_must_sign_error() -> Result<(), TransportError>
{
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let mut realm_cookie = core_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    // Act
    let err = core_voter_test
        .with_registrar_using_ix(
            &realm_cookie,
            |i| i.accounts[4].is_signer = false, // realm_authority
            Some(&[]),
        )
        .await
        .err()
        .unwrap();

    assert_anchor_err(err, anchor_lang::error::ErrorCode::AccountNotSigner);

    Ok(())
}

#[tokio::test]
async fn test_create_registrar_with_invalid_spl_gov_program_id_error() -> Result<(), TransportError>
{
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let mut realm_cookie = core_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    // Try to use a different program id
    let governance_program_id = core_voter_test.program_id;

    // Act
    let err = core_voter_test
        .with_registrar_using_ix(
            &realm_cookie,
            |i| i.accounts[1].pubkey = governance_program_id, //governance_program_id
            None,
        )
        .await
        .err()
        .unwrap();

    assert_anchor_err(err, anchor_lang::error::ErrorCode::ConstraintOwner);

    Ok(())
}

#[tokio::test]
async fn test_create_registrar_with_invalid_realm_error() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let mut realm_cookie = core_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    // Act
    let err = core_voter_test
        .with_registrar_using_ix(
            &realm_cookie,
            |i| i.accounts[2].pubkey = Pubkey::new_unique(), // realm
            None,
        )
        .await
        .err()
        .unwrap();

    // PDA doesn't match and hence the error is ConstraintSeeds
    assert_anchor_err(err, ErrorCode::ConstraintSeeds);

    Ok(())
}

#[tokio::test]
async fn test_create_registrar_with_invalid_governing_token_mint_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let mut realm_cookie = core_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    let mint_cookie = core_voter_test.bench.with_mint().await?;

    // Act
    let err = core_voter_test
        .with_registrar_using_ix(
            &realm_cookie,
            |i| i.accounts[3].pubkey = mint_cookie.address, // governing_token_mint
            None,
        )
        .await
        .err()
        .unwrap();

    // PDA doesn't match and hence the error is ConstraintSeeds
    assert_anchor_err(err, ErrorCode::ConstraintSeeds);

    Ok(())
}
