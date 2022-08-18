use anchor_spl::token::TokenAccount;
use gpl_nft_voter::state::DelegatorTokenOwnerRecord;
use program_test::nft_voter_test::NftVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_deposit_governance_tokens_first_deposit_creates_record() -> Result<(), TransportError>
{
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let owner_cookie = nft_voter_test.bench.with_wallet_funded(100000000000).await;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &owner_cookie, None)
        .await?;

    let governance_token_holding_account_cookie = nft_voter_test
        .with_governance_token_holding_account(&registrar_cookie, &nft_cookie, None)
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
async fn test_deposit_governance_tokens_record_exists_doesnt_error() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let owner_cookie = nft_voter_test.bench.with_wallet_funded(100000000000).await;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &owner_cookie, None)
        .await?;

    let governance_token_holding_account_cookie = nft_voter_test
        .with_governance_token_holding_account(&registrar_cookie, &nft_cookie, None)
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
    let first_result = nft_voter_test
        .with_nft_voter_token_owner_record(
            &realm_cookie,
            &nft_cookie,
            &governance_token_holding_account_cookie,
            &owner_cookie,
            &governing_token_source_account_cookie,
            None,
        )
        .await;

    let second_result = nft_voter_test
        .with_nft_voter_token_owner_record(
            &realm_cookie,
            &nft_cookie,
            &governance_token_holding_account_cookie,
            &owner_cookie,
            &governing_token_source_account_cookie,
            None,
        )
        .await;

    // Assert
    assert!(!second_result.is_err());
    assert_eq_formatted(
        first_result.unwrap().address,
        second_result.unwrap().address,
        "record address",
    );

    Ok(())
}

#[tokio::test]
async fn test_deposit_governance_tokens_transfers_tokens_to_holding_account(
) -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let owner_cookie = nft_voter_test.bench.with_wallet_funded(100000000000).await;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &owner_cookie, None)
        .await?;

    let governance_token_holding_account_cookie = nft_voter_test
        .with_governance_token_holding_account(&registrar_cookie, &nft_cookie, None)
        .await?;

    let source_tokens: u64 = 1000;
    let deposit_tokens: u64 = ((source_tokens as f64) * 0.5) as u64;

    let governing_token_source_account_cookie = nft_voter_test
        .bench
        .with_tokens(
            &realm_cookie.community_mint_cookie,
            &owner_cookie.address,
            source_tokens,
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
            Some(deposit_tokens),
        )
        .await?;

    // Assert
    assert_eq_formatted(
        deposit_tokens,
        token_owner_record_cookie
            .account
            .governing_token_deposit_amount,
        "deposit amount",
    );
    assert_eq_formatted(
        source_tokens - deposit_tokens,
        nft_voter_test
            .bench
            .get_anchor_account::<TokenAccount>(governing_token_source_account_cookie.address)
            .await
            .amount,
        "source remaining amount",
    );
    assert_eq_formatted(
        deposit_tokens,
        nft_voter_test
            .bench
            .get_anchor_account::<TokenAccount>(governance_token_holding_account_cookie.address)
            .await
            .amount,
        "holding account amount",
    );

    Ok(())
}

#[tokio::test]
async fn test_deposit_governance_tokens_multiple_deposits_holding_account_stores_cumulative(
) -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let owner_cookie = nft_voter_test.bench.with_wallet_funded(100000000000).await;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &owner_cookie, None)
        .await?;

    let governance_token_holding_account_cookie = nft_voter_test
        .with_governance_token_holding_account(&registrar_cookie, &nft_cookie, None)
        .await?;

    let source_tokens: u64 = 1000;
    let first_deposit_tokens: u64 = ((source_tokens as f64) * 0.25) as u64;
    let second_deposit_tokens: u64 = ((source_tokens as f64) * 0.1) as u64;
    let total_deposit: u64 = first_deposit_tokens + second_deposit_tokens;

    let governing_token_source_account_cookie = nft_voter_test
        .bench
        .with_tokens(
            &realm_cookie.community_mint_cookie,
            &owner_cookie.address,
            source_tokens,
        )
        .await?;

    // Act
    let first_deposit_record_cookie = nft_voter_test
        .with_nft_voter_token_owner_record(
            &realm_cookie,
            &nft_cookie,
            &governance_token_holding_account_cookie,
            &owner_cookie,
            &governing_token_source_account_cookie,
            Some(first_deposit_tokens),
        )
        .await?;

    let second_deposit_record_cookie = nft_voter_test
        .with_nft_voter_token_owner_record(
            &realm_cookie,
            &nft_cookie,
            &governance_token_holding_account_cookie,
            &owner_cookie,
            &governing_token_source_account_cookie,
            Some(second_deposit_tokens),
        )
        .await?;

    // Assert
    assert_eq_formatted(
        total_deposit,
        nft_voter_test
            .bench
            .get_anchor_account::<TokenAccount>(governance_token_holding_account_cookie.address)
            .await
            .amount,
        "holding account amount",
    );
    assert_eq_formatted(
        first_deposit_record_cookie.address,
        second_deposit_record_cookie.address,
        "deposit record address",
    );
    assert_eq_formatted(
        total_deposit,
        nft_voter_test
            .bench
            .get_anchor_account::<DelegatorTokenOwnerRecord>(first_deposit_record_cookie.address)
            .await
            .governing_token_deposit_amount,
        "record deposit",
    );

    Ok(())
}

#[tokio::test]
async fn test_deposit_governance_tokens_source_insufficient_balance_errors(
) -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let owner_cookie = nft_voter_test.bench.with_wallet_funded(100000000000).await;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &owner_cookie, None)
        .await?;

    let governance_token_holding_account_cookie = nft_voter_test
        .with_governance_token_holding_account(&registrar_cookie, &nft_cookie, None)
        .await?;

    let source_tokens: u64 = 1000;
    let deposit_tokens: u64 = source_tokens + 1;

    let governing_token_source_account_cookie = nft_voter_test
        .bench
        .with_tokens(
            &realm_cookie.community_mint_cookie,
            &owner_cookie.address,
            source_tokens,
        )
        .await?;

    // Act
    let result = nft_voter_test
        .with_nft_voter_token_owner_record(
            &realm_cookie,
            &nft_cookie,
            &governance_token_holding_account_cookie,
            &owner_cookie,
            &governing_token_source_account_cookie,
            Some(deposit_tokens),
        )
        .await;

    // Assert
    //TODO better error to check for?
    assert!(result.is_err());

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
