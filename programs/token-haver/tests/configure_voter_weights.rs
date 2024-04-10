use gpl_realm_voter::error::RealmVoterError;
use gpl_realm_voter::state::CollectionItemChangeType;
use program_test::realm_voter_test::RealmVoterTest;
use program_test::tools::*;
use solana_program_test::*;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_configure_voter_weights() -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let governance_program_cookie = realm_voter_test.with_governance_program(None).await;

    realm_voter_test
        .configure_governance_program(
            &registrar_cookie,
            &governance_program_cookie,
            CollectionItemChangeType::Upsert,
        )
        .await?;

    let mut max_voter_weight_record_cookie = realm_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Act
    realm_voter_test
        .configure_voter_weights(
            &registrar_cookie,
            &mut max_voter_weight_record_cookie,
            10,
            110,
        )
        .await?;

    // Assert

    let registrar = realm_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar.max_voter_weight, 110);
    assert_eq!(registrar.realm_member_voter_weight, 10);

    let max_voter_weight_record = realm_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    assert_eq!(max_voter_weight_record.max_voter_weight, 110);

    assert_eq!(max_voter_weight_record.max_voter_weight_expiry, None);
    assert_eq!(max_voter_weight_record.realm, realm_cookie.address);
    assert_eq!(
        max_voter_weight_record.governing_token_mint,
        realm_cookie.account.community_mint
    );

    Ok(())
}

#[tokio::test]
async fn test_configure_voter_weights_with_invalid_realm_error() -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let mut max_voter_weight_record_cookie = realm_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Try to use a different Realm
    let realm_cookie2 = realm_voter_test.governance.with_realm().await?;

    // Act
    let err = realm_voter_test
        .configure_voter_weights_using_ix(
            &registrar_cookie,
            &mut max_voter_weight_record_cookie,
            10,
            110,
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
async fn test_configure_voter_weights_with_realm_authority_must_sign_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let mut max_voter_weight_record_cookie = realm_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Act
    let err = realm_voter_test
        .configure_voter_weights_using_ix(
            &registrar_cookie,
            &mut max_voter_weight_record_cookie,
            10,
            110,
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
async fn test_configure_voter_weights_with_invalid_realm_authority_error(
) -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let mut max_voter_weight_record_cookie = realm_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let realm_authority = Keypair::new();

    // Act
    let err = realm_voter_test
        .configure_voter_weights_using_ix(
            &registrar_cookie,
            &mut max_voter_weight_record_cookie,
            10,
            110,
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
