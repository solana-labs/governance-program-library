mod program_test;

use anchor_lang::prelude::{ErrorCode, Pubkey};
use program_test::{
    bonk_plugin_test::BonkPluginTest, spl_token_staking_test::SplTokenStakingCookie,
};

use solana_program::instruction::InstructionError;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, transport::TransportError};

use program_test::tools::{assert_anchor_err, assert_bonks_plugin_err, assert_ix_err};

#[tokio::test]
async fn test_create_registrar() -> Result<(), TransportError> {
    // Arrange
    let mut bonk_plugin_test = BonkPluginTest::start_new().await;

    let realm_cookie = bonk_plugin_test.governance.with_realm().await?;

    // Act
    let mut spl_token_staking_cookie = SplTokenStakingCookie::new(bonk_plugin_test.bench.clone());
    let stake_pool_pubkey = spl_token_staking_cookie
        .with_stake_pool(&realm_cookie.community_mint_cookie.address)
        .await?;
    let registrar_cookie = bonk_plugin_test
        .with_registrar(&realm_cookie, &stake_pool_pubkey)
        .await?;

    // Assert
    let registrar = bonk_plugin_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar, registrar_cookie.account);

    Ok(())
}

#[tokio::test]
async fn test_create_registrar_with_invalid_realm_authority_error() -> Result<(), TransportError> {
    // Arrange
    let mut bonk_plugin_test = BonkPluginTest::start_new().await;

    let mut realm_cookie = bonk_plugin_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();
    let mut spl_token_staking_cookie = SplTokenStakingCookie::new(bonk_plugin_test.bench.clone());
    let stake_pool_pubkey = spl_token_staking_cookie
        .with_stake_pool(&realm_cookie.community_mint_cookie.address)
        .await?;

    // Act
    let err = bonk_plugin_test
        .with_registrar(&realm_cookie, &stake_pool_pubkey)
        .await
        .err()
        .unwrap();

    assert_bonks_plugin_err(
        err,
        gpl_bonk_plugin::error::BonkPluginError::InvalidRealmAuthority,
    );

    Ok(())
}

#[tokio::test]
async fn test_create_registrar_with_realm_authority_must_sign_error() -> Result<(), TransportError>
{
    // Arrange
    let mut bonk_plugin_test = BonkPluginTest::start_new().await;

    let realm_cookie = bonk_plugin_test.governance.with_realm().await?;
    let mut spl_token_staking_cookie = SplTokenStakingCookie::new(bonk_plugin_test.bench.clone());
    let stake_pool_pubkey = spl_token_staking_cookie
        .with_stake_pool(&realm_cookie.community_mint_cookie.address)
        .await?;

    // Act
    let err = bonk_plugin_test
        .with_registrar_using_ix(
            &realm_cookie,
            &stake_pool_pubkey,
            |i| i.accounts[6].is_signer = false, // realm_authority
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
    let mut bonk_plugin_test = BonkPluginTest::start_new().await;

    let realm_cookie = bonk_plugin_test.governance.with_realm().await?;
    let mut spl_token_staking_cookie = SplTokenStakingCookie::new(bonk_plugin_test.bench.clone());
    let stake_pool_pubkey = spl_token_staking_cookie
        .with_stake_pool(&realm_cookie.community_mint_cookie.address)
        .await?;

    // Try to use a different program id
    let governance_program_id = bonk_plugin_test.program_id;

    // Act
    let err = bonk_plugin_test
        .with_registrar_using_ix(
            &realm_cookie,
            &stake_pool_pubkey,
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
    let mut bonk_plugin_test = BonkPluginTest::start_new().await;

    let realm_cookie = bonk_plugin_test.governance.with_realm().await?;

    let mut spl_token_staking_cookie = SplTokenStakingCookie::new(bonk_plugin_test.bench.clone());
    let stake_pool_pubkey = spl_token_staking_cookie
        .with_stake_pool(&realm_cookie.community_mint_cookie.address)
        .await?;

    // Act
    let err = bonk_plugin_test
        .with_registrar_using_ix(
            &realm_cookie,
            &stake_pool_pubkey,
            |i| i.accounts[3].pubkey = Pubkey::new_unique(), // realm
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
    let mut bonk_plugin_test = BonkPluginTest::start_new().await;

    let realm_cookie = bonk_plugin_test.governance.with_realm().await?;

    let mint_cookie = bonk_plugin_test.bench.with_mint().await?;

    let mut spl_token_staking_cookie = SplTokenStakingCookie::new(bonk_plugin_test.bench.clone());
    let stake_pool_pubkey = spl_token_staking_cookie
        .with_stake_pool(&realm_cookie.community_mint_cookie.address)
        .await?;

    // Act
    let err = bonk_plugin_test
        .with_registrar_using_ix(
            &realm_cookie,
            &stake_pool_pubkey,
            |i| i.accounts[5].pubkey = mint_cookie.address, // governing_token_mint
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
async fn test_create_registrar_with_registrar_already_exists_error() -> Result<(), TransportError> {
    // Arrange
    let mut bonk_plugin_test = BonkPluginTest::start_new().await;

    let realm_cookie = bonk_plugin_test.governance.with_realm().await?;

    let mut spl_token_staking_cookie = SplTokenStakingCookie::new(bonk_plugin_test.bench.clone());
    let stake_pool_pubkey = spl_token_staking_cookie
        .with_stake_pool(&realm_cookie.community_mint_cookie.address)
        .await?;
    bonk_plugin_test
        .with_registrar(&realm_cookie, &stake_pool_pubkey)
        .await?;

    bonk_plugin_test.bench.advance_clock().await;

    // Act

    let err = bonk_plugin_test
        .with_registrar(&realm_cookie, &stake_pool_pubkey)
        .await
        .err()
        .unwrap();

    // Assert

    // Registrar already exists and it throws Custom(0) error
    assert_ix_err(err, InstructionError::Custom(0));

    Ok(())
}
