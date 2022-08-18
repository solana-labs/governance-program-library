use program_test::nft_voter_test::NftVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_withdraw_governance_tokens_nothing_deposited_errors() -> Result<(), TransportError> {
    // Arrange
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

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    let governance_token_holding_account_cookie = nft_voter_test
    .with_governance_token_holding_account(&registrar_cookie, &nft_cookie)
    .await?;

    // Assert
   

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
