use program_test::token_voter_test::TokenVoterTest;
use program_test::tools::*;
use solana_program_test::*;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_configure_voter_weights_with_token_extensions() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new_token_extensions(None).await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let first_user_cookie = token_voter_test.users.first().unwrap();
    let first_mint_cookie = token_voter_test.mints.first().unwrap();

    let voter_cookie = token_voter_test
        .with_voter(&registrar_cookie, first_user_cookie)
        .await?;

    let max_voter_weight_record_cookie = token_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let _mint_cookie = token_voter_test
        .configure_mint_config(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            first_mint_cookie,
            0, // no digit shift
        )
        .await?;

    // Assert

    let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;

    assert_eq!(voter_data.deposits.len(), 0);
    assert_eq!(voter_data.registrar, registrar_cookie.address);

    let registrar = token_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar.voting_mint_configs.len(), 1);
    assert_eq!(
        registrar.voting_mint_configs.first().unwrap().mint,
        first_mint_cookie.address
    );

    let max_voter_weight_record = token_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    // supply is 100
    assert_eq!(max_voter_weight_record.max_voter_weight, 100);

    assert_eq!(max_voter_weight_record.max_voter_weight_expiry, None);
    assert_eq!(max_voter_weight_record.realm, realm_cookie.address);
    assert_eq!(
        max_voter_weight_record.governing_token_mint,
        realm_cookie.account.community_mint
    );

    Ok(())
}

#[tokio::test]
async fn test_configure_voter_weights() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new().await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let first_user_cookie = token_voter_test.users.first().unwrap();
    let first_mint_cookie = token_voter_test.mints.first().unwrap();

    let voter_cookie = token_voter_test
        .with_voter(&registrar_cookie, first_user_cookie)
        .await?;

    let max_voter_weight_record_cookie = token_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let _mint_cookie = token_voter_test
        .configure_mint_config(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            first_mint_cookie,
            0, // no digit shift
        )
        .await?;

    // Assert

    let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;

    assert_eq!(voter_data.deposits.len(), 0);
    assert_eq!(voter_data.registrar, registrar_cookie.address);

    let registrar = token_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar.voting_mint_configs.len(), 1);
    assert_eq!(
        registrar.voting_mint_configs.first().unwrap().mint,
        first_mint_cookie.address
    );

    let max_voter_weight_record = token_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    // supply is 100
    assert_eq!(max_voter_weight_record.max_voter_weight, 100);

    assert_eq!(max_voter_weight_record.max_voter_weight_expiry, None);
    assert_eq!(max_voter_weight_record.realm, realm_cookie.address);
    assert_eq!(
        max_voter_weight_record.governing_token_mint,
        realm_cookie.account.community_mint
    );

    Ok(())
}

#[tokio::test]
async fn test_configure_voter_weights_with_invalid_voter_error() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new().await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;

    let first_user_cookie = token_voter_test.users.first().unwrap();

    // Act
    let err = token_voter_test
        .with_voter_using_ix(
            &registrar_cookie,
            &first_user_cookie,
            |i| i.accounts[1].pubkey = Pubkey::new_unique(), // voter
        )
        .await
        .err()
        .unwrap();

    // Assert

    assert_anchor_err(err, anchor_lang::error::ErrorCode::ConstraintSeeds);

    Ok(())
}

#[tokio::test]
async fn test_configure_voter_weights_with_realm_authority_must_sign_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new().await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;

    let max_voter_weight_record_cookie = token_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let first_mint_cookie = token_voter_test.mints.first().unwrap();

    // Act

    let err = token_voter_test
        .configure_mint_config_using_ix(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            first_mint_cookie,
            0,                                   // no digit shift,
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
