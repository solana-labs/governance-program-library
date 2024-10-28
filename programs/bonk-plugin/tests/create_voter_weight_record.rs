use crate::program_test::bonk_plugin_test::BonkPluginTest;
use program_test::{spl_token_staking_test::SplTokenStakingCookie, tools::assert_ix_err};
use solana_program::instruction::InstructionError;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_create_voter_weight_record() -> Result<(), TransportError> {
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

    let voter_cookie = bonk_plugin_test.bench.with_wallet().await;

    // Act
    let voter_weight_record_cookie = bonk_plugin_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    // Assert

    let voter_weight_record = bonk_plugin_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record_cookie.account, voter_weight_record);

    Ok(())
}

#[tokio::test]
async fn test_create_voter_weight_record_with_already_exists_error() -> Result<(), TransportError> {
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

    let voter_cookie = bonk_plugin_test.bench.with_wallet().await;

    bonk_plugin_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    bonk_plugin_test.bench.advance_clock().await;

    // Act
    let err = bonk_plugin_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await
        .err()
        .unwrap();

    // Assert

    // InstructionError::Custom(0) is returned for TransactionError::AccountInUse
    assert_ix_err(err, InstructionError::Custom(0));

    Ok(())
}
