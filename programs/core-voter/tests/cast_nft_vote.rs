use crate::program_test::core_voter_test::ConfigureCollectionArgs;
use gpl_core_voter::error::NftVoterError;
use gpl_core_voter::state::*;
use program_test::{
    core_voter_test::*,
    tools::{assert_gov_err, assert_nft_voter_err},
};

use solana_program_test::*;
use solana_sdk::transport::TransportError;
use spl_governance::error::GovernanceError;

mod program_test;

#[tokio::test]
async fn test_cast_asset_vote() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let asset_cookie1 = core_voter_test
        .core.create_asset(&collection_cookie, &voter_cookie)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
            }),
        )
        .await?;

    let voter_token_owner_record_cookie = core_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = core_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    core_voter_test.bench.advance_clock().await;
    let clock = core_voter_test.bench.get_clock().await;

    // Act
    let asset_vote_record_cookies = core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&asset_cookie1],
            None,
        )
        .await?;

    // Assert
    let asset_vote_record = core_voter_test
        .get_asset_vote_record_account(&asset_vote_record_cookies[0].address)
        .await;

    assert_eq!(asset_vote_record_cookies[0].account, asset_vote_record);

    let voter_weight_record = core_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 10);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        Some(proposal_cookie.address)
    );

    Ok(())
}

#[tokio::test]
async fn test_cast_asset_vote_with_multiple_nfts() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let asset_cookie1 = core_voter_test
        .core.create_asset(&collection_cookie, &voter_cookie)
        .await?;

    let asset_cookie2 = core_voter_test
        .core.create_asset(&collection_cookie, &voter_cookie)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
            }),
        )
        .await?;

    let voter_token_owner_record_cookie = core_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = core_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    core_voter_test.bench.advance_clock().await;
    let clock = core_voter_test.bench.get_clock().await;

    // Act
    let asset_vote_record_cookies = core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&asset_cookie1, &asset_cookie2],
            None,
        )
        .await?;

    // Assert
    let asset_vote_record1 = core_voter_test
        .get_asset_vote_record_account(&asset_vote_record_cookies[0].address)
        .await;

    assert_eq!(asset_vote_record_cookies[0].account, asset_vote_record1);

    let asset_vote_record2 = core_voter_test
        .get_asset_vote_record_account(&asset_vote_record_cookies[1].address)
        .await;

    assert_eq!(asset_vote_record_cookies[1].account, asset_vote_record2);

    let voter_weight_record = core_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 20);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        Some(proposal_cookie.address)
    );

    Ok(())
}

#[tokio::test]
async fn test_cast_asset_vote_with_nft_already_voted_error() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie: program_test::program_test_bench::WalletCookie = core_voter_test.bench.with_wallet().await;

    let asset_cookie1 = core_voter_test
        .core.create_asset(&collection_cookie, &voter_cookie,)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            None,
        )
        .await?;

    let voter_token_owner_record_cookie = core_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = core_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&asset_cookie1],
            None,
        )
        .await?;

    core_voter_test.bench.advance_clock().await;

    // Act
    let err = core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&asset_cookie1],
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::NftAlreadyVoted);

    Ok(())
}

#[tokio::test]
async fn test_cast_asset_vote_with_invalid_voter_error() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let asset_cookie1 = core_voter_test
        .core.create_asset(&collection_cookie, &voter_cookie)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            None,
        )
        .await?;

    let voter_token_owner_record_cookie = core_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = core_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let voter_cookie2 = core_voter_test.bench.with_wallet().await;

    // Act

    let err = core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie2,
            &voter_token_owner_record_cookie,
            &[&asset_cookie1],
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_gov_err(err, GovernanceError::GoverningTokenOwnerOrDelegateMustSign);

    Ok(())
}

#[tokio::test]
async fn test_cast_asset_vote_with_invalid_owner_error() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let voter_cookie2 = core_voter_test.bench.with_wallet().await;

    let asset_cookie = core_voter_test
        .core.create_asset(&collection_cookie, &voter_cookie2)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
            }),
        )
        .await?;


    let voter_token_owner_record_cookie = core_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = core_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    // Act
    let err = core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&asset_cookie],
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::VoterDoesNotOwnNft);

    Ok(())
}

#[tokio::test]
async fn test_cast_asset_vote_with_invalid_collection_error() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie: program_test::governance_test::RealmCookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let collection_cookie2 = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie: program_test::program_test_bench::WalletCookie = core_voter_test.bench.with_wallet().await;

    let _random_asset_cookie = core_voter_test
        .core.create_asset(&collection_cookie, &voter_cookie)
        .await?;

    let asset_cookie = core_voter_test
        .core.create_asset(&collection_cookie2, &voter_cookie)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
            }),
        )
        .await?;

    let proposal_cookie = core_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;


    let voter_token_owner_record_cookie = core_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    // Act
    let err = core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&asset_cookie],
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::CollectionNotFound);

    Ok(())
}

#[tokio::test]
async fn test_cast_asset_vote_with_same_nft_error() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let asset_cookie = core_voter_test
        .core.create_asset(&collection_cookie, &voter_cookie)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            None,
        )
        .await?;

    let proposal_cookie = core_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let voter_token_owner_record_cookie = core_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    // Act
    let err = core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&asset_cookie, &asset_cookie],
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert

    assert_nft_voter_err(err, NftVoterError::DuplicatedNftDetected);

    Ok(())
}

#[tokio::test]
async fn test_cast_asset_vote_with_max_5_nfts() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let mut asset_cookies = vec![];

    for _ in 0..5 {
        core_voter_test.bench.advance_clock().await;
        let asset_cookie = core_voter_test
            .core
            .create_asset(&collection_cookie, &voter_cookie)
            .await?;

        asset_cookies.push(asset_cookie)
    }

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
            }),
        )
        .await?;

    let voter_token_owner_record_cookie = core_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = core_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    core_voter_test.bench.advance_clock().await;
    let clock = core_voter_test.bench.get_clock().await;

    // Act
    let asset_vote_record_cookies = core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &asset_cookies.iter().collect::<Vec<_>>(),
            None,
        )
        .await?;

    // Assert
    let asset_vote_record1 = core_voter_test
        .get_asset_vote_record_account(&asset_vote_record_cookies[0].address)
        .await;

    assert_eq!(asset_vote_record_cookies[0].account, asset_vote_record1);

    let asset_vote_record2 = core_voter_test
        .get_asset_vote_record_account(&asset_vote_record_cookies[1].address)
        .await;

    assert_eq!(asset_vote_record_cookies[1].account, asset_vote_record2);

    let voter_weight_record = core_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 50);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        Some(proposal_cookie.address)
    );

    Ok(())
}

#[tokio::test]
async fn test_cast_asset_vote_using_multiple_instructions() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let asset_cookie1 = core_voter_test
        .core.create_asset(&collection_cookie, &voter_cookie)
        .await?;

    let asset_cookie2 = core_voter_test
        .core.create_asset(&collection_cookie, &voter_cookie)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
            }),
        )
        .await?;

    let voter_token_owner_record_cookie = core_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = core_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    core_voter_test.bench.advance_clock().await;
    let clock = core_voter_test.bench.get_clock().await;

    let args = CastAssetVoteArgs {
        cast_spl_gov_vote: false,
    };

    core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&asset_cookie1],
            Some(args),
        )
        .await?;

    // Act
    core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&asset_cookie2],
            None,
        )
        .await?;

    // Assert

    let voter_weight_record = core_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 20);
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));
    assert_eq!(
        voter_weight_record.weight_action,
        Some(VoterWeightAction::CastVote.into())
    );
    assert_eq!(
        voter_weight_record.weight_action_target,
        Some(proposal_cookie.address)
    );

    Ok(())
}

#[tokio::test]
async fn test_cast_asset_vote_using_multiple_instructions_with_nft_already_voted_error() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let asset_cookie1 = core_voter_test
        .core.create_asset(&collection_cookie, &voter_cookie)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
            }),
        )
        .await?;

    let voter_token_owner_record_cookie = core_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = core_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let args = CastAssetVoteArgs {
        cast_spl_gov_vote: false,
    };

    core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&asset_cookie1],
            Some(args),
        )
        .await?;

    // Act
    let err = core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&asset_cookie1],
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::NftAlreadyVoted);

    Ok(())
}

#[tokio::test]
async fn test_cast_asset_vote_using_multiple_instructions_with_attempted_sandwiched_relinquish() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let asset_cookie1 = core_voter_test
        .core.create_asset(&collection_cookie, &voter_cookie)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            Some(ConfigureCollectionArgs {
                weight: 10,
            }),
        )
        .await?;

    let voter_token_owner_record_cookie = core_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = core_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    let args = CastAssetVoteArgs {
        cast_spl_gov_vote: false,
    };

    // Cast vote with NFT
    let asset_vote_record_cookies = core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&asset_cookie1],
            Some(args),
        )
        .await?;

    core_voter_test.bench.advance_clock().await;

    // Try relinquish NftVoteRecords to accumulate vote
    core_voter_test
        .relinquish_nft_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &asset_vote_record_cookies,
        )
        .await?;

    // Act

    core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&asset_cookie1],
            None,
        )
        .await?;

    // Assert

    let voter_weight_record = core_voter_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;

    assert_eq!(voter_weight_record.voter_weight, 10);

    Ok(())
}

#[tokio::test]
async fn test_cast_asset_vote_using_delegate() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let asset_cookie1 = core_voter_test
        .core.create_asset(&collection_cookie, &voter_cookie)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            None,
        )
        .await?;

    let voter_token_owner_record_cookie = core_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let voter_weight_record_cookie = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let proposal_cookie = core_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    core_voter_test.bench.advance_clock().await;

    let delegate_cookie = core_voter_test.bench.with_wallet().await;
    core_voter_test
        .governance
        .set_governance_delegate(
            &realm_cookie,
            &voter_token_owner_record_cookie,
            &voter_cookie,
            &Some(delegate_cookie.address),
        )
        .await;

    // Act
    let asset_vote_record_cookies = core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &delegate_cookie,
            &voter_token_owner_record_cookie,
            &[&asset_cookie1],
            None,
        )
        .await?;

    // Assert
    let asset_vote_record = core_voter_test
        .get_asset_vote_record_account(&asset_vote_record_cookies[0].address)
        .await;

    assert_eq!(asset_vote_record_cookies[0].account, asset_vote_record);

    Ok(())
}

#[tokio::test]
async fn test_cast_asset_vote_with_invalid_voter_weight_token_owner_error() -> Result<(), TransportError> {
    // Arrange
    let mut core_voter_test = CoreVoterTest::start_new().await;

    let realm_cookie = core_voter_test.governance.with_realm().await?;

    let registrar_cookie = core_voter_test.with_registrar(&realm_cookie).await?;

    let collection_cookie = core_voter_test.core.create_collection(None).await?;

    let max_voter_weight_record_cookie = core_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let voter_cookie = core_voter_test.bench.with_wallet().await;

    let asset_cookie1 = core_voter_test
        .core
        .create_asset(&collection_cookie, &voter_cookie)
        .await?;

    core_voter_test
        .with_collection(
            &registrar_cookie,
            &collection_cookie,
            &max_voter_weight_record_cookie,
            None,
        )
        .await?;

    let voter_token_owner_record_cookie = core_voter_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    // Try to update VoterWeightRecord for different governing_token_owner
    let voter_cookie2 = core_voter_test.bench.with_wallet().await;

    let voter_weight_record_cookie2 = core_voter_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie2)
        .await?;

    let proposal_cookie = core_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    // Act

    let err = core_voter_test
        .cast_asset_vote(
            &registrar_cookie,
            &voter_weight_record_cookie2,
            &max_voter_weight_record_cookie,
            &proposal_cookie,
            &voter_cookie,
            &voter_token_owner_record_cookie,
            &[&asset_cookie1],
            None,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_nft_voter_err(err, NftVoterError::InvalidTokenOwnerForVoterWeightRecord);

    Ok(())
}
