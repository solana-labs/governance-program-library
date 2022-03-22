use crate::program_test::nft_voter_test::ConfigureCollectionArgs;
use program_test::nft_voter_test::NftVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_relinquish_nft_vote() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs { weight: 1, size: 1 }), // Set Size == 1 to complete voting with just one vote
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_token_owner_record_cookie = nft_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, true)
        .await?;

    let nft_vote_record_cookies = nft_voter_test
        .cast_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&nft_cookie1],
        )
        .await?;

    // Act

    nft_voter_test
        .relinquish_vote(
            &registrar_cookie,
            &proposal_cookie,
            &voter_cookie,
            &nft_vote_record_cookies,
        )
        .await?;

    // Assert

    Ok(())
}
