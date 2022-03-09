use program_test::nft_voter_test::NftVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_configure_collection() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let mut registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    // Act
    nft_voter_test
        .with_configure_collection(&mut registrar_cookie)
        .await?;

    Ok(())
}

// TODO: Check ream for registrar
