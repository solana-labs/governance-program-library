use program_test::nft_voter_test::NftVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_vote_with_nft() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test.governance
        .with_proposal(&realm_cookie, &voter_weight_record_cookie)
        .await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let nft1 = nft_voter_test.token_metadata.with_nft_v2(&nft_collection_cookie, &voter_cookie, true).await?;
    let _nft2 = nft_voter_test.token_metadata.with_nft_v2(&nft_collection_cookie, &voter_cookie, true).await;

    // Act
    nft_voter_test
        .vote_with_nft(&registrar_cookie, &voter_weight_record_cookie, &proposal_cookie, &nft1)
        .await?;

    Ok(())
}