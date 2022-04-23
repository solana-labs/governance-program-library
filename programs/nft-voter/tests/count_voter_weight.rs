use crate::program_test::nft_voter_test::ConfigureCollectionArgs;
use gpl_nft_voter::error::NftVoterError;

use program_test::{nft_voter_test::NftVoterTest, tools::assert_nft_voter_err};
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_count_voter_weight() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let collection_config_cookie = nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    // Act

    let voter_weight_counter_cookie = nft_voter_test
        .count_voter_weight(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &[&nft_cookie1],
            &collection_config_cookie,
        )
        .await?;

    // Assert
    let voter_weight_counter = nft_voter_test
        .get_voter_weight_counter_account(&voter_weight_counter_cookie.address)
        .await;

    assert_eq!(voter_weight_counter_cookie.account, voter_weight_counter);

    let nft_vote_record_cookies = voter_weight_counter_cookie.nft_vote_record_cookies;

    let nft_vote_record = nft_voter_test
        .get_nf_vote_record_account(&nft_vote_record_cookies[0].address)
        .await;

    assert_eq!(nft_vote_record_cookies[0].account, nft_vote_record);

    Ok(())
}

#[tokio::test]
async fn test_count_voter_weight_with_multiple_votes() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let collection_config_cookie = nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    nft_voter_test
        .count_voter_weight(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &[&nft_cookie1],
            &collection_config_cookie,
        )
        .await?;

    let nft_cookie2 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    // Act

    let voter_weight_counter_cookie = nft_voter_test
        .count_voter_weight(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &[&nft_cookie2],
            &collection_config_cookie,
        )
        .await?;

    // Assert
    let voter_weight_counter = nft_voter_test
        .get_voter_weight_counter_account(&voter_weight_counter_cookie.address)
        .await;

    assert_eq!(voter_weight_counter.voter_weight, 20);

    Ok(())
}

#[tokio::test]
async fn test_count_voter_weight_with_nft_already_voted_error() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let collection_config_cookie = nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    nft_voter_test
        .count_voter_weight(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &[&nft_cookie1],
            &collection_config_cookie,
        )
        .await?;

    nft_voter_test.bench.advance_clock().await;

    // Act

    let err = nft_voter_test
        .count_voter_weight(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &[&nft_cookie1],
            &collection_config_cookie,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::NftAlreadyVoted);

    Ok(())
}
