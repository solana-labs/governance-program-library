use gpl_realm_voter::error::RealmVoterError;
use program_test::realm_voter_test::{ConfigureGovernanceProgramArgs, RealmVoterTest};
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, transport::TransportError};
mod program_test;
use program_test::tools::{assert_anchor_err, assert_realm_voter_err};

#[tokio::test]
async fn test_configure_governance_program() -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let governance_program_cookie = realm_voter_test.with_governance_program(None).await;

    // Act
    let governance_program_config_cookie = realm_voter_test
        .with_governance_program_config(&registrar_cookie, &governance_program_cookie, None)
        .await?;

    // // Assert
    let registrar = realm_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar.governance_program_configs.len(), 1);

    assert_eq!(
        registrar.governance_program_configs[0],
        governance_program_config_cookie.program_config
    );

    Ok(())
}

#[tokio::test]
async fn test_configure_governance_program_with_multiple_programs() -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let governance_program_cookie1 = realm_voter_test.with_governance_program(None).await;

    // Create config with gpl_realm_voter::id() to have some other executable program, the actual program is irrelevant here
    let governance_program_cookie2 = realm_voter_test
        .with_governance_program(Some(gpl_realm_voter::id()))
        .await;

    // Act
    realm_voter_test
        .with_governance_program_config(
            &registrar_cookie,
            &governance_program_cookie1,
            Some(ConfigureGovernanceProgramArgs { weight: 1 }),
        )
        .await?;

    realm_voter_test
        .with_governance_program_config(
            &registrar_cookie,
            &governance_program_cookie2,
            Some(ConfigureGovernanceProgramArgs { weight: 2 }),
        )
        .await?;

    // Assert
    let registrar = realm_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar.governance_program_configs.len(), 2);

    Ok(())
}

#[tokio::test]
async fn test_configure_governance_program_for_existing_program() -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let governance_program_cookie = realm_voter_test.with_governance_program(None).await;

    realm_voter_test
        .with_governance_program_config(
            &registrar_cookie,
            &governance_program_cookie,
            Some(ConfigureGovernanceProgramArgs { weight: 2 }),
        )
        .await?;

    // Act

    realm_voter_test
        .with_governance_program_config(
            &registrar_cookie,
            &governance_program_cookie,
            Some(ConfigureGovernanceProgramArgs { weight: 5 }),
        )
        .await?;

    // Assert
    let registrar = realm_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar.governance_program_configs.len(), 1);

    assert_eq!(registrar.governance_program_configs[0].weight, 5);

    Ok(())
}

#[tokio::test]
async fn test_remove_governance_program_configuration() -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let governance_program_cookie = realm_voter_test.with_governance_program(None).await;

    realm_voter_test
        .with_governance_program_config(
            &registrar_cookie,
            &governance_program_cookie,
            Some(ConfigureGovernanceProgramArgs { weight: 2 }),
        )
        .await?;

    // Act

    realm_voter_test
        .with_governance_program_config(
            &registrar_cookie,
            &governance_program_cookie,
            Some(ConfigureGovernanceProgramArgs { weight: 0 }),
        )
        .await?;

    // Assert
    let registrar = realm_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar.governance_program_configs.len(), 0);

    Ok(())
}

#[tokio::test]
async fn test_configure_governance_program_with_0_weight_error() -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let governance_program_cookie = realm_voter_test.with_governance_program(None).await;

    // Act

    let err = realm_voter_test
        .with_governance_program_config(
            &registrar_cookie,
            &governance_program_cookie,
            Some(ConfigureGovernanceProgramArgs { weight: 0 }),
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_realm_voter_err(err, RealmVoterError::InvalidGovernanceProgramWeight);

    Ok(())
}

#[tokio::test]
async fn test_configure_governance_program_with_invalid_realm_error() -> Result<(), TransportError>
{
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let governance_program_cookie = realm_voter_test.with_governance_program(None).await;

    // Try to use a different Realm
    let realm_cookie2 = realm_voter_test.governance.with_realm().await?;

    // Act
    let err = realm_voter_test
        .with_governance_program_config_using_ix(
            &registrar_cookie,
            &governance_program_cookie,
            None,
            |i| i.accounts[1].pubkey = realm_cookie2.address, // realm
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert

    assert_realm_voter_err(err, RealmVoterError::InvalidRealmForRegistrar);

    Ok(())
}

#[tokio::test]
async fn test_configure_governance_program_with_realm_authority_must_sign_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let governance_program_cookie = realm_voter_test.with_governance_program(None).await;

    // Act
    let err = realm_voter_test
        .with_governance_program_config_using_ix(
            &registrar_cookie,
            &governance_program_cookie,
            None,
            |i| i.accounts[2].is_signer = false, // realm_authority
            Some(&[]),
        )
        .await
        .err()
        .unwrap();

    // Assert

    assert_anchor_err(err, anchor_lang::error::ErrorCode::AccountNotSigner);

    Ok(())
}

#[tokio::test]
async fn test_configure_governance_program_with_invalid_realm_authority_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let governance_program_cookie = realm_voter_test.with_governance_program(None).await;

    let realm_authority = Keypair::new();

    // Act
    let err = realm_voter_test
        .with_governance_program_config_using_ix(
            &registrar_cookie,
            &governance_program_cookie,
            None,
            |i| i.accounts[2].pubkey = realm_authority.pubkey(), // realm_authority
            Some(&[&realm_authority]),
        )
        .await
        .err()
        .unwrap();

    // Assert

    assert_realm_voter_err(err, RealmVoterError::InvalidRealmAuthority);

    Ok(())
}
