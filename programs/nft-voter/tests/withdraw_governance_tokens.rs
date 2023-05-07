use program_test::nft_voter_test::NftVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_withdraw_governance_tokens_nothing_deposited_errors() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = &mut nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;
    let max_voter_weight_record = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record,
            None,
        )
        .await?;

    let registrar_updated = nft_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    registrar_cookie.account = registrar_updated;

    let owner_cookie = nft_voter_test.bench.with_wallet_funded(100000000000).await;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &owner_cookie, None)
        .await?;

    let governance_token_holding_account_cookie = nft_voter_test
        .with_governance_token_holding_account(&registrar_cookie, &realm_cookie, &nft_cookie, None)
        .await?;

    let governing_token_source_account_cookie = nft_voter_test
        .bench
        .with_tokens(
            &realm_cookie.community_mint_cookie,
            &owner_cookie.address,
            1000,
        )
        .await?;

    // Act
    let token_owner_record_cookie = nft_voter_test
        .with_nft_voter_token_owner_record(
            &realm_cookie,
            &nft_cookie,
            &governance_token_holding_account_cookie,
            &owner_cookie,
            &governing_token_source_account_cookie,
            None,
        )
        .await?;

    // Assert
    assert_eq_formatted(
        0,
        token_owner_record_cookie
            .account
            .governing_token_deposit_amount,
        "amount",
    );
    assert_eq_formatted(
        realm_cookie.community_mint_cookie.address,
        token_owner_record_cookie.account.governing_token_mint,
        "governing_token_mint",
    );
    assert_eq_formatted(
        owner_cookie.address,
        token_owner_record_cookie.account.governing_token_owner,
        "governing_token_owner",
    );
    assert_eq_formatted(
        nft_cookie.mint_cookie.address,
        token_owner_record_cookie.account.nft_mint,
        "nft_mint",
    );
    assert_eq_formatted(
        realm_cookie.address,
        token_owner_record_cookie.account.realm,
        "realm",
    );

    Ok(())
}

#[tokio::test]
async fn test_withdraw_governance_tokens_try_withdraw_zero_errors() -> Result<(), TransportError> {
    todo!()
}

#[tokio::test]
async fn test_withdraw_governance_tokens_try_withdraw_more_than_deposited_errors(
) -> Result<(), TransportError> {
    todo!()
}

#[tokio::test]
async fn test_withdraw_governance_tokens_nft_has_open_votes_errors() -> Result<(), TransportError> {
    todo!()
}

#[tokio::test]
async fn test_withdraw_governance_tokens_nft_has_open_proposals_errors(
) -> Result<(), TransportError> {
    todo!()
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
