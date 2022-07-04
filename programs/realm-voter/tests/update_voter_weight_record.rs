use crate::program_test::realm_voter_test::{ConfigureGovernanceProgramArgs, RealmVoterTest};
use solana_program_test::*;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_update_voter_weight_record() -> Result<(), TransportError> {
    // Arrange
    let mut realm_voter_test = RealmVoterTest::start_new().await;

    let realm_cookie = realm_voter_test.governance.with_realm().await?;

    let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

    let governance_program_cookie = realm_voter_test.with_governance_program(None).await;

    realm_voter_test
        .with_governance_program_config(
            &registrar_cookie,
            &governance_program_cookie,
            Some(ConfigureGovernanceProgramArgs { weight: 10 }),
        )
        .await?;

    let voter_cookie = realm_voter_test.bench.with_wallet().await;

    let mut voter_weight_record_cookie = realm_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    // let squad_member_cookie = realm_voter_test
    //     .squads
    //     .with_squad_member(&squad_cookie)
    //     .await?;

    realm_voter_test.bench.advance_clock().await;
    let clock = realm_voter_test.bench.get_clock().await;

    // Act
    // realm_voter_test
    //     .update_voter_weight_record(
    //         &registrar_cookie,
    //         &mut voter_weight_record_cookie,
    //         &[&squad_member_cookie],
    //     )
    //     .await?;

    // Assert

    let voter_weight_record = realm_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 10);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(voter_weight_record.weight_action, None);
    assert_eq!(voter_weight_record.weight_action_target, None);

    Ok(())
}

// #[tokio::test]
// async fn test_update_voter_weight_with_multiple_nfts() -> Result<(), TransportError> {
//     // Arrange
//     let mut realm_voter_test = NftVoterTest::start_new().await;

//     let realm_cookie = realm_voter_test.governance.with_realm().await?;

//     let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

//     let nft_collection_cookie = realm_voter_test.token_metadata.with_nft_collection().await?;

//     let max_voter_weight_record_cookie = realm_voter_test
//         .with_max_voter_weight_record(&registrar_cookie)
//         .await?;

//     let _collection_config_cookie = realm_voter_test
//         .with_collection(
//             &registrar_cookie,
//             &nft_collection_cookie,
//             &max_voter_weight_record_cookie,
//             Some(ConfigureCollectionArgs {
//                 weight: 10,
//                 size: 20,
//             }),
//         )
//         .await?;

//     let voter_cookie = realm_voter_test.bench.with_wallet().await;

//     let mut voter_weight_record_cookie = realm_voter_test
//         .with_voter_weight_record(&registrar_cookie, &voter_cookie)
//         .await?;

//     let nft_cookie1 = realm_voter_test
//         .token_metadata
//         .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
//         .await?;

//     let nft_cookie2 = realm_voter_test
//         .token_metadata
//         .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
//         .await?;

//     realm_voter_test.bench.advance_clock().await;
//     let clock = realm_voter_test.bench.get_clock().await;

//     // Act
//     realm_voter_test
//         .update_voter_weight_record(
//             &registrar_cookie,
//             &mut voter_weight_record_cookie,
//             VoterWeightAction::CreateProposal,
//             &[&nft_cookie1, &nft_cookie2],
//         )
//         .await?;

//     // Assert

//     let voter_weight_record = realm_voter_test
//         .get_voter_weight_record(&voter_weight_record_cookie.address)
//         .await;

//     assert_eq!(voter_weight_record.voter_weight, 20);
//     assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
//     assert_eq!(
//         voter_weight_record.weight_action,
//         Some(VoterWeightAction::CreateProposal.into())
//     );
//     assert_eq!(voter_weight_record.weight_action_target, None);

//     Ok(())
// }

// #[tokio::test]
// async fn test_update_voter_weight_with_cast_vote_not_allowed_error() -> Result<(), TransportError> {
//     // Arrange
//     let mut realm_voter_test = NftVoterTest::start_new().await;

//     let realm_cookie = realm_voter_test.governance.with_realm().await?;

//     let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

//     let nft_collection_cookie = realm_voter_test.token_metadata.with_nft_collection().await?;

//     let max_voter_weight_record_cookie = realm_voter_test
//         .with_max_voter_weight_record(&registrar_cookie)
//         .await?;

//     realm_voter_test
//         .with_collection(
//             &registrar_cookie,
//             &nft_collection_cookie,
//             &max_voter_weight_record_cookie,
//             Some(ConfigureCollectionArgs {
//                 weight: 10,
//                 size: 20,
//             }),
//         )
//         .await?;

//     let voter_cookie = realm_voter_test.bench.with_wallet().await;

//     let mut voter_weight_record_cookie = realm_voter_test
//         .with_voter_weight_record(&registrar_cookie, &voter_cookie)
//         .await?;

//     let nft1_cookie = realm_voter_test
//         .token_metadata
//         .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
//         .await?;

//     // Act
//     let err = realm_voter_test
//         .update_voter_weight_record(
//             &registrar_cookie,
//             &mut voter_weight_record_cookie,
//             VoterWeightAction::CastVote,
//             &[&nft1_cookie],
//         )
//         .await
//         .err()
//         .unwrap();

//     // Assert
//     assert_nft_voter_err(err, NftVoterError::CastVoteIsNotAllowed);

//     Ok(())
// }

// #[tokio::test]
// async fn test_update_voter_weight_with_unverified_collection_error() -> Result<(), TransportError> {
//     // Arrange
//     let mut realm_voter_test = NftVoterTest::start_new().await;

//     let realm_cookie = realm_voter_test.governance.with_realm().await?;

//     let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

//     let nft_collection_cookie = realm_voter_test.token_metadata.with_nft_collection().await?;

//     let max_voter_weight_record_cookie = realm_voter_test
//         .with_max_voter_weight_record(&registrar_cookie)
//         .await?;

//     realm_voter_test
//         .with_collection(
//             &registrar_cookie,
//             &nft_collection_cookie,
//             &max_voter_weight_record_cookie,
//             Some(ConfigureCollectionArgs {
//                 weight: 10,
//                 size: 20,
//             }),
//         )
//         .await?;

//     let voter_cookie = realm_voter_test.bench.with_wallet().await;

//     let mut voter_weight_record_cookie = realm_voter_test
//         .with_voter_weight_record(&registrar_cookie, &voter_cookie)
//         .await?;

//     // Create NFT without verified collection
//     let nft1_cookie = realm_voter_test
//         .token_metadata
//         .with_nft_v2(
//             &nft_collection_cookie,
//             &voter_cookie,
//             Some(CreateNftArgs {
//                 verify_collection: false,
//                 ..Default::default()
//             }),
//         )
//         .await?;

//     // Act
//     let err = realm_voter_test
//         .update_voter_weight_record(
//             &registrar_cookie,
//             &mut voter_weight_record_cookie,
//             VoterWeightAction::CreateGovernance,
//             &[&nft1_cookie],
//         )
//         .await
//         .err()
//         .unwrap();

//     // Assert
//     assert_nft_voter_err(err, NftVoterError::CollectionMustBeVerified);

//     Ok(())
// }

// #[tokio::test]
// async fn test_update_voter_weight_with_invalid_owner_error() -> Result<(), TransportError> {
//     // Arrange
//     let mut realm_voter_test = NftVoterTest::start_new().await;

//     let realm_cookie = realm_voter_test.governance.with_realm().await?;

//     let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

//     let nft_collection_cookie = realm_voter_test.token_metadata.with_nft_collection().await?;

//     let max_voter_weight_record_cookie = realm_voter_test
//         .with_max_voter_weight_record(&registrar_cookie)
//         .await?;

//     realm_voter_test
//         .with_collection(
//             &registrar_cookie,
//             &nft_collection_cookie,
//             &max_voter_weight_record_cookie,
//             Some(ConfigureCollectionArgs {
//                 weight: 10,
//                 size: 20,
//             }),
//         )
//         .await?;

//     let voter_cookie = realm_voter_test.bench.with_wallet().await;

//     let mut voter_weight_record_cookie = realm_voter_test
//         .with_voter_weight_record(&registrar_cookie, &voter_cookie)
//         .await?;

//     let voter_cookie2 = realm_voter_test.bench.with_wallet().await;

//     let nft1_cookie = realm_voter_test
//         .token_metadata
//         .with_nft_v2(&nft_collection_cookie, &voter_cookie2, None)
//         .await?;

//     // Act
//     let err = realm_voter_test
//         .update_voter_weight_record(
//             &registrar_cookie,
//             &mut voter_weight_record_cookie,
//             VoterWeightAction::CreateGovernance,
//             &[&nft1_cookie],
//         )
//         .await
//         .err()
//         .unwrap();

//     // Assert
//     assert_nft_voter_err(err, NftVoterError::VoterDoesNotOwnNft);

//     Ok(())
// }

// #[tokio::test]
// async fn test_update_voter_weight_with_invalid_collection_error() -> Result<(), TransportError> {
//     // Arrange
//     let mut realm_voter_test = NftVoterTest::start_new().await;

//     let realm_cookie = realm_voter_test.governance.with_realm().await?;

//     let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

//     let nft_collection_cookie = realm_voter_test.token_metadata.with_nft_collection().await?;

//     let max_voter_weight_record_cookie = realm_voter_test
//         .with_max_voter_weight_record(&registrar_cookie)
//         .await?;

//     realm_voter_test
//         .with_collection(
//             &registrar_cookie,
//             &nft_collection_cookie,
//             &max_voter_weight_record_cookie,
//             Some(ConfigureCollectionArgs {
//                 weight: 10,
//                 size: 20,
//             }),
//         )
//         .await?;

//     let voter_cookie = realm_voter_test.bench.with_wallet().await;

//     let mut voter_weight_record_cookie = realm_voter_test
//         .with_voter_weight_record(&registrar_cookie, &voter_cookie)
//         .await?;

//     let nft_collection_cookie2 = realm_voter_test.token_metadata.with_nft_collection().await?;

//     let nft1_cookie = realm_voter_test
//         .token_metadata
//         .with_nft_v2(&nft_collection_cookie2, &voter_cookie, None)
//         .await?;

//     // Act
//     let err = realm_voter_test
//         .update_voter_weight_record(
//             &registrar_cookie,
//             &mut voter_weight_record_cookie,
//             VoterWeightAction::CreateGovernance,
//             &[&nft1_cookie],
//         )
//         .await
//         .err()
//         .unwrap();

//     // Assert
//     assert_nft_voter_err(err, NftVoterError::CollectionNotFound);

//     Ok(())
// }

// #[tokio::test]
// async fn test_update_voter_weight_with_invalid_metadata_error() -> Result<(), TransportError> {
//     // Arrange
//     let mut realm_voter_test = NftVoterTest::start_new().await;

//     let realm_cookie = realm_voter_test.governance.with_realm().await?;

//     let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

//     let nft_collection_cookie = realm_voter_test.token_metadata.with_nft_collection().await?;

//     let max_voter_weight_record_cookie = realm_voter_test
//         .with_max_voter_weight_record(&registrar_cookie)
//         .await?;

//     realm_voter_test
//         .with_collection(
//             &registrar_cookie,
//             &nft_collection_cookie,
//             &max_voter_weight_record_cookie,
//             Some(ConfigureCollectionArgs {
//                 weight: 10,
//                 size: 20,
//             }),
//         )
//         .await?;

//     let voter_cookie = realm_voter_test.bench.with_wallet().await;

//     let mut voter_weight_record_cookie = realm_voter_test
//         .with_voter_weight_record(&registrar_cookie, &voter_cookie)
//         .await?;

//     let mut nft1_cookie = realm_voter_test
//         .token_metadata
//         .with_nft_v2(
//             &nft_collection_cookie,
//             &voter_cookie,
//             Some(CreateNftArgs {
//                 verify_collection: false,
//                 ..Default::default()
//             }),
//         )
//         .await?;

//     let nft2_cookie = realm_voter_test
//         .token_metadata
//         .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
//         .await?;

//     // Try to use verified NFT Metadata
//     nft1_cookie.metadata = nft2_cookie.metadata;

//     // Act
//     let err = realm_voter_test
//         .update_voter_weight_record(
//             &registrar_cookie,
//             &mut voter_weight_record_cookie,
//             VoterWeightAction::CreateGovernance,
//             &[&nft1_cookie],
//         )
//         .await
//         .err()
//         .unwrap();

//     // Assert
//     assert_nft_voter_err(err, NftVoterError::TokenMetadataDoesNotMatch);

//     Ok(())
// }

// #[tokio::test]
// async fn test_update_voter_weight_with_same_nft_error() -> Result<(), TransportError> {
//     // Arrange
//     let mut realm_voter_test = NftVoterTest::start_new().await;

//     let realm_cookie = realm_voter_test.governance.with_realm().await?;

//     let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

//     let nft_collection_cookie = realm_voter_test.token_metadata.with_nft_collection().await?;

//     let max_voter_weight_record_cookie = realm_voter_test
//         .with_max_voter_weight_record(&registrar_cookie)
//         .await?;

//     realm_voter_test
//         .with_collection(
//             &registrar_cookie,
//             &nft_collection_cookie,
//             &max_voter_weight_record_cookie,
//             None,
//         )
//         .await?;

//     let voter_cookie = realm_voter_test.bench.with_wallet().await;

//     let mut voter_weight_record_cookie = realm_voter_test
//         .with_voter_weight_record(&registrar_cookie, &voter_cookie)
//         .await?;

//     let nft_cookie = realm_voter_test
//         .token_metadata
//         .with_nft_v2(&nft_collection_cookie, &voter_cookie, None)
//         .await?;

//     // Act
//     let err = realm_voter_test
//         .update_voter_weight_record(
//             &registrar_cookie,
//             &mut voter_weight_record_cookie,
//             VoterWeightAction::CreateProposal,
//             &[&nft_cookie, &nft_cookie],
//         )
//         .await
//         .err()
//         .unwrap();

//     // Assert

//     assert_nft_voter_err(err, NftVoterError::DuplicatedNftDetected);

//     Ok(())
// }

// #[tokio::test]
// async fn test_update_voter_weight_record_with_no_nft_error() -> Result<(), TransportError> {
//     // Arrange
//     let mut realm_voter_test = NftVoterTest::start_new().await;

//     let realm_cookie = realm_voter_test.governance.with_realm().await?;

//     let registrar_cookie = realm_voter_test.with_registrar(&realm_cookie).await?;

//     let nft_collection_cookie = realm_voter_test.token_metadata.with_nft_collection().await?;

//     let max_voter_weight_record_cookie = realm_voter_test
//         .with_max_voter_weight_record(&registrar_cookie)
//         .await?;

//     let _collection_config_cookie = realm_voter_test
//         .with_collection(
//             &registrar_cookie,
//             &nft_collection_cookie,
//             &max_voter_weight_record_cookie,
//             Some(ConfigureCollectionArgs {
//                 weight: 10,
//                 size: 20,
//             }),
//         )
//         .await?;

//     let voter_cookie = realm_voter_test.bench.with_wallet().await;

//     let mut voter_weight_record_cookie = realm_voter_test
//         .with_voter_weight_record(&registrar_cookie, &voter_cookie)
//         .await?;

//     let nft1_cookie = realm_voter_test
//         .token_metadata
//         .with_nft_v2(
//             &nft_collection_cookie,
//             &voter_cookie,
//             Some(CreateNftArgs {
//                 amount: 0,
//                 ..Default::default()
//             }),
//         )
//         .await?;

//     // Act
//     let err = realm_voter_test
//         .update_voter_weight_record(
//             &registrar_cookie,
//             &mut voter_weight_record_cookie,
//             VoterWeightAction::CreateProposal,
//             &[&nft1_cookie],
//         )
//         .await
//         .err()
//         .unwrap();

//     // Assert
//     assert_nft_voter_err(err, NftVoterError::InvalidNftAmount);

//     Ok(())
// }
