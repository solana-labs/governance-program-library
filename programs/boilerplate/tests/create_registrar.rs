mod program_test;

use anchor_lang::prelude::Pubkey;
use program_test::dummy_voter_test::DummyVoterTest;

use solana_program::instruction::InstructionError;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, transport::TransportError};
use gpl_boilerplate::error::BoilerplateError;

use program_test::tools::{assert_anchor_err, assert_ix_err, assert_boilerplate_err};

#[tokio::test]
async fn test_create_registrar() -> Result<(), TransportError> {
    // Arrange
    let mut dummy_voter_test = DummyVoterTest::start_new().await;

    let realm_cookie = dummy_voter_test.governance.with_realm().await?;

    // Act
    let registrar_cookie = dummy_voter_test.with_registrar(&realm_cookie).await?;

    // Assert
    let registrar = dummy_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar, registrar_cookie.account);

    Ok(())
}

#[tokio::test]
async fn test_create_registrar_with_invalid_realm_authority_error() -> Result<(), TransportError> {
    // Arrange
    let mut dummy_voter_test = DummyVoterTest::start_new().await;

    let mut realm_cookie = dummy_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    // Act
    let err = dummy_voter_test
        .with_registrar(&realm_cookie)
        .await
        .err()
        .unwrap();

    assert_boilerplate_err(err, BoilerplateError::InvalidRealmAuthority);

    Ok(())
}

#[tokio::test]
async fn test_create_registrar_with_realm_authority_must_sign_error() -> Result<(), TransportError>
{
    // Arrange
    let mut dummy_voter_test = DummyVoterTest::start_new().await;

    let mut realm_cookie = dummy_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    // Act
    let err = dummy_voter_test
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
    let mut dummy_voter_test = DummyVoterTest::start_new().await;

    let mut realm_cookie = dummy_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    // Try to use a different program id
    let governance_program_id = dummy_voter_test.program_id;

    // Act
    let err = dummy_voter_test
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
    let mut dummy_voter_test = DummyVoterTest::start_new().await;

    let mut realm_cookie = dummy_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    // Act
    let err = dummy_voter_test
        .with_registrar_using_ix(
            &realm_cookie,
            |i| i.accounts[2].pubkey = Pubkey::new_unique(), // realm
            None,
        )
        .await
        .err()
        .unwrap();

    // PDA doesn't match and hence the error is PrivilegeEscalation
    assert_ix_err(err, InstructionError::PrivilegeEscalation);

    Ok(())
}

#[tokio::test]
async fn test_create_registrar_with_invalid_governing_token_mint_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut dummy_voter_test = DummyVoterTest::start_new().await;

    let mut realm_cookie = dummy_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    let mint_cookie = dummy_voter_test.bench.with_mint().await?;

    // Act
    let err = dummy_voter_test
        .with_registrar_using_ix(
            &realm_cookie,
            |i| i.accounts[3].pubkey = mint_cookie.address, // governing_token_mint
            None,
        )
        .await
        .err()
        .unwrap();

    // PDA doesn't match and hence the error is PrivilegeEscalation
    assert_ix_err(err, InstructionError::PrivilegeEscalation);

    Ok(())
}
