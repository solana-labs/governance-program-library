use program_test::nft_voter_test::NftVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_max_create_voter_weight_record() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let registrar_cookie = nft_voter_test.with_registrar().await;

    // Act
    nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await;

    Ok(())
}
