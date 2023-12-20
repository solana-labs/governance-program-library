mod program_test;

use anchor_lang::prelude::{ErrorCode, Pubkey};
use program_test::gateway_voter_test::GatewayVoterTest;

use gpl_civic_gateway::error::GatewayError;

use solana_program_test::*;
use solana_sdk::{signature::Keypair, transport::TransportError};

use crate::program_test::predecessor_plugin_test::PredecessorPluginTest;
use crate::program_test::tools::NopOverride;
use program_test::tools::{assert_anchor_err, assert_gateway_err};

#[tokio::test]
async fn test_create_registrar() -> Result<(), TransportError> {
    // Arrange
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;

    let realm_cookie = gateway_voter_test.governance.with_realm().await?;
    let gateway_cookie = gateway_voter_test.with_gateway().await?;

    // Act
    let registrar_cookie = gateway_voter_test
        .with_registrar(&realm_cookie, &gateway_cookie, None)
        .await?;

    // Assert
    let registrar = gateway_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar, registrar_cookie.account);

    Ok(())
}

#[tokio::test]
async fn test_configure_registrar_new_previous_plugin() -> Result<(), TransportError> {
    // Arrange
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;

    let realm_cookie = gateway_voter_test.governance.with_realm().await?;
    let gateway_cookie = gateway_voter_test.with_gateway().await?;

    // Act
    let predecessor_program_id = PredecessorPluginTest::program_id();
    let registrar_cookie = gateway_voter_test
        .with_registrar(&realm_cookie, &gateway_cookie, Some(predecessor_program_id))
        .await?;

    // Assert
    let registrar = gateway_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar, registrar_cookie.account);

    Ok(())
}

#[tokio::test]
async fn test_configure_registrar_missing_previous_plugin_error() -> Result<(), TransportError> {
    // Arrange
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;

    let realm_cookie = gateway_voter_test.governance.with_realm().await?;
    let gateway_cookie = gateway_voter_test.with_gateway().await?;

    // Act
    let err = gateway_voter_test
        .with_registrar_using_ix(
            &realm_cookie,
            &gateway_cookie,
            None,
            true, // This causes the error
            &gpl_civic_gateway::id(),
            NopOverride,
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_gateway_err(err, GatewayError::MissingPreviousVoterWeightPlugin);

    Ok(())
}

#[tokio::test]
async fn test_create_registrar_with_invalid_realm_authority_error() -> Result<(), TransportError> {
    // Arrange
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;

    let gateway_cookie = gateway_voter_test.with_gateway().await?;
    let mut realm_cookie = gateway_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    // Act
    let err = gateway_voter_test
        .with_registrar(&realm_cookie, &gateway_cookie, None)
        .await
        .err()
        .unwrap();

    assert_gateway_err(err, GatewayError::InvalidRealmAuthority);

    Ok(())
}

#[tokio::test]
async fn test_create_registrar_with_realm_authority_must_sign_error() -> Result<(), TransportError>
{
    // Arrange
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;

    let mut realm_cookie = gateway_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();
    let gateway_cookie = gateway_voter_test.with_gateway().await?;

    // Act
    let err = gateway_voter_test
        .with_registrar_using_ix(
            &realm_cookie,
            &gateway_cookie,
            None,
            false,
            &gpl_civic_gateway::id(),
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
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;

    let mut realm_cookie = gateway_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    let gateway_cookie = gateway_voter_test.with_gateway().await?;

    // Try to use a different program id
    let governance_program_id = gateway_voter_test.program_id;

    // Act
    let err = gateway_voter_test
        .with_registrar_using_ix(
            &realm_cookie,
            &gateway_cookie,
            None,
            false,
            &gpl_civic_gateway::id(),
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
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;

    let mut realm_cookie = gateway_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    let gateway_cookie = gateway_voter_test.with_gateway().await?;

    // Act
    let err = gateway_voter_test
        .with_registrar_using_ix(
            &realm_cookie,
            &gateway_cookie,
            None,
            false,
            &gpl_civic_gateway::id(),
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
    let mut gateway_voter_test = GatewayVoterTest::start_new().await;

    let mut realm_cookie = gateway_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    let gateway_cookie = gateway_voter_test.with_gateway().await?;

    let mint_cookie = gateway_voter_test.bench.with_mint().await?;

    // Act
    let err = gateway_voter_test
        .with_registrar_using_ix(
            &realm_cookie,
            &gateway_cookie,
            None,
            false,
            &gpl_civic_gateway::id(),
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
