use program_test::nft_voter_test_bench::NftVoterTestBench;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_update_voter_weight_record() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_bench = NftVoterTestBench::start_new().await;

    let registrar_cookie = nft_voter_bench.with_registrar().await;

    let voter_weight_record_cookie = nft_voter_bench
        .with_voter_weight_record(&registrar_cookie)
        .await;

    let nft1 = nft_voter_bench.token_metadata.with_nft_v2().await;
    let nft2 = nft_voter_bench.token_metadata.with_nft_v2().await;

    // Act
    nft_voter_bench
        .update_voter_weight_record(&registrar_cookie, &voter_weight_record_cookie)
        .await;

    Ok(())
}
