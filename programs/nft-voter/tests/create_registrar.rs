use program_test::nft_voter_test::NftVoterTest;
use solana_program_test::*;

mod program_test;

#[tokio::test]
async fn test_create_registrar() {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    // Act
    let registrar_cookie = nft_voter_test.with_registrar().await;

    // Assert
    let registrar = nft_voter_test
        .get_registrar_account(&registrar_cookie.address)
        .await;

    assert_eq!(registrar, registrar_cookie.account)
}
