use program_test::nft_voter_test_bench::NftVoterTestBench;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_create_voter_weight_record() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_bench = NftVoterTestBench::start_new().await;

    // Act
    nft_voter_bench.with_registrar().await;

    Ok(())
}
