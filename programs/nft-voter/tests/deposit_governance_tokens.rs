use anchor_spl::token::Mint;
use program_test::nft_voter_test::NftVoterTest;
use program_test::program_test_bench::MintCookie;
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
        .with_governance_token_holding_account(&registrar_cookie, &nft_cookie)
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

    let token_owner_record = nft_voter_test
        .with_nft_voter_token_owner_record(
            &realm_cookie,
            &nft_cookie,
            &governance_token_holding_account_cookie,
            &owner_cookie,
            &governing_token_source_account_cookie,
        )
        .await?;

    // Assert
    // assert_eq_formatted(
    //     0,
    //     governance_token_holding_account_cookie.account.amount,
    //     "amount",
    // );
    // assert_eq_formatted(
    //     COption::None,
    //     governance_token_holding_account_cookie.account.delegate,
    //     "delegate",
    // );
    // assert_eq_formatted(
    //     0,
    //     governance_token_holding_account_cookie
    //         .account
    //         .delegated_amount,
    //     "delegated_amount",
    // );
    // assert_eq_formatted(
    //     COption::None,
    //     governance_token_holding_account_cookie
    //         .account
    //         .close_authority,
    //     "close_authority",
    // );
    // assert_eq_formatted(
    //     realm_cookie.community_mint_cookie.address,
    //     governance_token_holding_account_cookie.account.mint,
    //     "mint",
    // );
    // assert_eq_formatted(
    //     registrar_cookie.account.governance_program_id,
    //     governance_token_holding_account_cookie.account.owner,
    //     "owner",
    // );
    // assert_eq_formatted(
    //     AccountState::Initialized,
    //     governance_token_holding_account_cookie.account.state,
    //     "state",
    // );

    Ok(())
}

#[tokio::test]
async fn test_create_governance_token_holding_account_nft_is_not_part_of_configured_collection_errors(
) -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;
    let max_voter_weight_record = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;
    assert_eq!(
        0,
        registrar_cookie.account.collection_configs.len(),
        "Unexpected collection already configured for registrar"
    );
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
    assert_eq!(
        1,
        registrar_updated.collection_configs.len(),
        "Unable to add collection to registrar"
    );

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    // Act
    //TODO add validation to the instruction and/or method and expect it to throw
    let error = nft_voter_test
        .with_governance_token_holding_account(&registrar_cookie, &nft_cookie)
        .await
        .err();

    // Assert
    //TODO
    // assert!(error.is_some());

    Ok(())
}

#[tokio::test]
async fn test_create_governance_token_holding_account_already_exists_errors(
) -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let nft_cookie = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    // Act
    nft_voter_test
        .with_governance_token_holding_account(&registrar_cookie, &nft_cookie)
        .await?;

    let error = nft_voter_test
        .with_governance_token_holding_account(&registrar_cookie, &nft_cookie)
        .await
        .err();

    // Assert
    assert!(error.is_some());

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
