use crate::program_test::nft_voter_test::ConfigureCollectionArgs;
use program_test::nft_voter_test::NftVoterTest;
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_count_voter_weight() -> Result<(), TransportError> {
    // Arrange
    let mut nft_voter_test = NftVoterTest::start_new().await;

    let realm_cookie = nft_voter_test.governance.with_realm().await?;

    let registrar_cookie = nft_voter_test.with_registrar(&realm_cookie).await?;

    let nft_collection_cookie = nft_voter_test.token_metadata.with_nft_collection().await?;

    let max_voter_weight_record_cookie = nft_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let collection_config_cookie = nft_voter_test
        .with_collection(
            &registrar_cookie,
            &nft_collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
                size: 20,
            }),
        )
        .await?;

    let voter_cookie = nft_voter_test.bench.with_wallet().await;

    let voter_weight_record_cookie = nft_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = nft_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let nft_cookie1 = nft_voter_test
        .token_metadata
        .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
        .await?;

    // Act

    let voter_weight_counter_cookie = nft_voter_test
        .count_voter_weight(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &[&nft_cookie1],
            &collection_config_cookie,
        )
        .await?;

    // Assert
    let voter_weight_counter = nft_voter_test
        .get_voter_weight_counter_account(&voter_weight_counter_cookie.address)
        .await;

    assert_eq!(voter_weight_counter_cookie.account, voter_weight_counter);

    // let voter_weight_record = nft_voter_test
    //     .get_voter_weight_record(&voter_weight_record_cookie.address)
    //     .await;

    // assert_eq!(voter_weight_record.voter_weight, 10);
    // assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    // assert_eq!(
    //     voter_weight_record.weight_action,
    //     Some(VoterWeightAction::CastVote.into())
    // );
    // assert_eq!(
    //     voter_weight_record.weight_action_target,
    //     Some(proposal_cookie.address)
    // );

    Ok(())
}
