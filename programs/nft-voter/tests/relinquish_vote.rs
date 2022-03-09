use program_test::nft_voter_test::NftVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_relinquish_vote() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie)
        .await?;

    nft_voter_test
        .update_voter_weight_record(&registrar_cookie, &voter_weight_record_cookie)
        .await?;

    // Act

    nft_voter_test
        .relinquish_vote(&registrar_cookie, &voter_weight_record_cookie)
        .await?;

    Ok(())
}
