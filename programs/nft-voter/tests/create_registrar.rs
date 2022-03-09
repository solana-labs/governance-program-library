mod program_test;

use anchor_lang::prelude::Pubkey;
use gpl_nft_voter::error::NftLockerErrorCode;
use program_test::nft_voter_test::NftVoterTest;

use solana_program::instruction::InstructionError;
use solana_program_test::*;
use solana_sdk::signature::Keypair;

use program_test::tools::{
    assert_anchor_err, assert_gov_tools_err, assert_ix_err, assert_nft_locker_err,
};

use spl_governance_tools::error::GovernanceToolsError;

#[tokio::test]
async fn test_create_registrar() -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    // Act
    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    // Assert
    let registrar = nft_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar, registrar_cookie.account);

    Ok(())
}

#[tokio::test]
async fn test_create_registrar_with_invalid_realm_authority_error() -> Result<(), BanksClientError>
{
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let mut realm_cookie = nft_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    // Act
    let err = nft_voter_test
        .with_registrar(&realm_cookie)
        .await
        .err()
        .unwrap();

    assert_nft_locker_err(err, NftLockerErrorCode::InvalidRealmAuthority);

    Ok(())
}

#[tokio::test]
async fn test_create_registrar_with_realm_authority_must_sign_error() -> Result<(), BanksClientError>
{
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let mut realm_cookie = nft_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    // Act
    let err = nft_voter_test
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
async fn test_create_registrar_with_invalid_spl_gov_program_id_error(
) -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let mut realm_cookie = nft_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    // Act
    let err = nft_voter_test
        .with_registrar_using_ix(
            &realm_cookie,
            |i| i.accounts[1].pubkey = Pubkey::new_unique(), //governance_program_id
            None,
        )
        .await
        .err()
        .unwrap();

    assert_gov_tools_err(err, GovernanceToolsError::InvalidAccountOwner);

    Ok(())
}

#[tokio::test]
async fn test_create_registrar_with_invalid_realm_error() -> Result<(), BanksClientError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let mut realm_cookie = nft_voter_test.governance.with_realm().await?;
    realm_cookie.realm_authority = Keypair::new();

    // Act
    let err = nft_voter_test
        .with_registrar_using_ix(
            &realm_cookie,
            |i| i.accounts[2].pubkey = Pubkey::new_unique(), // realm
            None,
        )
        .await
        .err()
        .unwrap();

    assert_ix_err(err, InstructionError::PrivilegeEscalation);

    Ok(())
}
