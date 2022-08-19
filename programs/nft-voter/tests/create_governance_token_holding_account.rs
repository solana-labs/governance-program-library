use gpl_nft_voter::error::NftVoterError;
use program_test::nft_voter_test::NftVoterTest;
use program_test::tools::assert_nft_voter_err;
use solana_program::program_option::COption;
use solana_program_test::*;
use solana_sdk::transport::TransportError;
use spl_token::state::AccountState;

mod program_test;

#[tokio::test]
async fn test_create_governance_token_holding_account() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = &mut nft_voter_test
        .with_registrar_with_collection(&realm_cookie)
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(
            &registrar_cookie
                .collection_cookies
                .as_ref()
                .unwrap()
                .first()
                .unwrap(),
            &voter_cookie,
            None,
        )
        .await?;

    // Act
    let governance_token_holding_account_cookie = nft_voter_test
        .with_governance_token_holding_account(&registrar_cookie, &realm_cookie, &nft_cookie, None)
        .await?;

    // Assert
    assert_eq_formatted(
        0,
        governance_token_holding_account_cookie.account.amount,
        "amount",
    );
    assert_eq_formatted(
        COption::None,
        governance_token_holding_account_cookie.account.delegate,
        "delegate",
    );
    assert_eq_formatted(
        0,
        governance_token_holding_account_cookie
            .account
            .delegated_amount,
        "delegated_amount",
    );
    assert_eq_formatted(
        COption::None,
        governance_token_holding_account_cookie
            .account
            .close_authority,
        "close_authority",
    );
    assert_eq_formatted(
        realm_cookie.community_mint_cookie.address,
        governance_token_holding_account_cookie.account.mint,
        "mint",
    );
    assert_eq_formatted(
        registrar_cookie.account.governance_program_id,
        governance_token_holding_account_cookie.account.owner,
        "owner",
    );
    assert_eq_formatted(
        AccountState::Initialized,
        governance_token_holding_account_cookie.account.state,
        "state",
    );

    Ok(())
}

#[tokio::test]
async fn test_create_governance_token_holding_account_nft_is_not_part_of_configured_collection_errors(
) -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = &mut nft_voter_test
        .with_registrar_with_collection(&realm_cookie)
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    // create the NFT with a different collection not configured for th realm
    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(
            &nft_voter_test.token_metadata.with_nft_collection().await?,
            &voter_cookie,
            None,
        )
        .await?;

    // Act
    let error = nft_voter_test
        .with_governance_token_holding_account(&registrar_cookie, &realm_cookie, &nft_cookie, None)
        .await
        .err();

    // Assert
    assert_nft_voter_err(error.unwrap(), NftVoterError::InvalidNftCollection);

    Ok(())
}

fn assert_eq_formatted<T: std::fmt::Debug + std::cmp::PartialEq>(
    expected: T,
    actual: T,
    name: &str,
) -> () {
    assert_eq!(
        expected, actual,
        "{} not equal: expected {:?} but got {:?}",
        name, expected, actual
    );
}
