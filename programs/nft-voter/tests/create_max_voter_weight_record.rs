use program_test::nft_voter_test::NftVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_max_create_voter_weight_record() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    // Act
    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    // Assert

    let max_voter_weight_record = nft_voter_test
        .get_max_voter_weight_record(&max_voter_weight_record_cookie.address)
        .await;

    assert_eq!(
        max_voter_weight_record_cookie.account,
        max_voter_weight_record
    );

    Ok(())
}
