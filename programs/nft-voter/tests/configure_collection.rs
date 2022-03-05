use program_test::nft_voter_test_bench::NftVoterTestBench;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_configure_collection() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_bench = NftVoterTestBench::start_new().await;

    let mut registrar_cookie = nft_voter_bench.with_registrar().await;

    // Act
    nft_voter_bench.with_configure_collection(&mut registrar_cookie).await;

    Ok(())
}
