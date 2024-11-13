use crate::program_test::bonk_plugin_test::BonkPluginTest;
use program_test::program_test_bench::{airdrop, WalletCookie};
use program_test::spl_token_staking_test::{find_reward_vault_key, find_stake_receipt_key};
use program_test::{spl_token_staking_test::SplTokenStakingCookie, tools::*};
use solana_program_test::*;
use solana_sdk::native_token::sol_to_lamports;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::{signature::Keypair, transport::TransportError};
mod program_test;
use solana_sdk::signature::Signer;
use spl_governance::error::GovernanceError;
use spl_governance::state::governance::get_governance_address;
use spl_governance::state::proposal::get_proposal_address;

#[tokio::test]
async fn test_update_voter_weight_record_create_proposal() -> Result<(), TransportError> {
    // Arrange
    let mut bonk_plugin_test = BonkPluginTest::start_new().await;

    let realm_cookie = bonk_plugin_test.governance.with_realm().await?;
    let mut spl_token_staking_cookie = SplTokenStakingCookie::new(bonk_plugin_test.bench.clone());
    let stake_pool_pubkey = spl_token_staking_cookie
        .with_stake_pool(&realm_cookie.community_mint_cookie.address)
        .await?;
    let registrar_cookie = bonk_plugin_test
        .with_registrar(&realm_cookie, &stake_pool_pubkey)
        .await?;

    let depositor = Keypair::new();
    airdrop(
        &mut bonk_plugin_test.bench.context.borrow_mut(),
        &depositor.pubkey(),
        sol_to_lamports(10.0),
    )
    .await?;
    let community_mint_cookie = &realm_cookie.community_mint_cookie;
    let token_account_cookie = bonk_plugin_test
        .governance
        .bench
        .with_tokens(&community_mint_cookie, &depositor.pubkey(), 100)
        .await?;

    let stake_pool_reciept = find_stake_receipt_key(
        depositor.pubkey(),
        stake_pool_pubkey,
        0,
        spl_token_staking_cookie.program_id,
    );
    spl_token_staking_cookie
        .deposit_into_stake_pool(
            &depositor,
            &stake_pool_pubkey,
            &stake_pool_reciept,
            &token_account_cookie.address,
            &[],
        )
        .await?;
    let voter_cookie = WalletCookie {
        address: depositor.pubkey(),
        signer: clone_keypair(&depositor),
    };
    let voter_weight_record_cookie = bonk_plugin_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let token_owner_record_cookie = bonk_plugin_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let token_account_cookie = bonk_plugin_test
        .bench
        .with_token_account(&realm_cookie.account.community_mint)
        .await?;

    let governance_key = get_governance_address(
        &bonk_plugin_test.governance.program_id,
        &realm_cookie.address,
        &token_account_cookie.address,
    );

    let proposal_governing_token_mint = realm_cookie.account.community_mint;
    let proposal_seed = Pubkey::new_unique();

    let proposal_key = get_proposal_address(
        &bonk_plugin_test.governance.program_id,
        &governance_key,
        &proposal_governing_token_mint,
        &proposal_seed,
    );
    let update_voter_weight_record_create_proposal_ix = bonk_plugin_test
        .update_voter_weight_record(
            &registrar_cookie,
            &voter_weight_record_cookie,
            // for proposals we can use TOR
            &token_owner_record_cookie.address,
            &token_owner_record_cookie,
            proposal_key,
            gpl_bonk_plugin::state::VoterWeightAction::CreateProposal,
            Some(proposal_key),
            &clone_keypair(&depositor),
            governance_key,
            &None,
        )
        .await?;
    bonk_plugin_test
        .governance
        .with_proposal(
            &realm_cookie,
            &voter_weight_record_cookie,
            &depositor,
            update_voter_weight_record_create_proposal_ix,
            &proposal_seed,
            &proposal_key,
            token_account_cookie,
        )
        .await?;

    // Assert
    let voter_weight_record = bonk_plugin_test
        .get_voter_weight_record(&voter_weight_record_cookie.address)
        .await;
    let clock = bonk_plugin_test.bench.get_clock().await;
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));

    assert_eq!(
        voter_weight_record.weight_action,
        Some(gpl_bonk_plugin::state::VoterWeightAction::CreateProposal.into())
    );
    assert_eq!(voter_weight_record.weight_action_target, Some(proposal_key));
    Ok(())
}

#[tokio::test]
async fn test_update_voter_weight_record_vote() -> Result<(), TransportError> {
    // Arrange
    let mut bonk_plugin_test = BonkPluginTest::start_new().await;

    let realm_cookie = bonk_plugin_test.governance.with_realm().await?;
    let mut spl_token_staking_cookie = SplTokenStakingCookie::new(bonk_plugin_test.bench.clone());
    let stake_pool_pubkey = spl_token_staking_cookie
        .with_stake_pool(&realm_cookie.community_mint_cookie.address)
        .await?;
    let registrar_cookie = bonk_plugin_test
        .with_registrar(&realm_cookie, &stake_pool_pubkey)
        .await?;

    let depositor = Keypair::new();
    let voter = Keypair::new();
    airdrop(
        &mut bonk_plugin_test.bench.context.borrow_mut(),
        &depositor.pubkey(),
        sol_to_lamports(10.0),
    )
    .await?;
    airdrop(
        &mut bonk_plugin_test.bench.context.borrow_mut(),
        &voter.pubkey(),
        sol_to_lamports(10.0),
    )
    .await?;
    let community_mint_cookie = &realm_cookie.community_mint_cookie;
    let token_account_cookie = bonk_plugin_test
        .governance
        .bench
        .with_tokens(&community_mint_cookie, &depositor.pubkey(), 100)
        .await?;
    let token_account_cookie_voter = bonk_plugin_test
        .governance
        .bench
        .with_tokens(&community_mint_cookie, &voter.pubkey(), 100)
        .await?;

    let stake_pool_reciept = find_stake_receipt_key(
        depositor.pubkey(),
        stake_pool_pubkey,
        0,
        spl_token_staking_cookie.program_id,
    );
    let stake_pool_reciept_voter = find_stake_receipt_key(
        voter.pubkey(),
        stake_pool_pubkey,
        0,
        spl_token_staking_cookie.program_id,
    );
    let reward_vault_key = find_reward_vault_key(
        stake_pool_pubkey,
        community_mint_cookie.address,
        spl_token_staking_cookie.program_id,
    );
    spl_token_staking_cookie
        .deposit_into_stake_pool(
            &depositor,
            &stake_pool_pubkey,
            &stake_pool_reciept,
            &token_account_cookie.address,
            &[&reward_vault_key],
        )
        .await?;
    spl_token_staking_cookie
        .deposit_into_stake_pool(
            &voter,
            &stake_pool_pubkey,
            &stake_pool_reciept_voter,
            &token_account_cookie_voter.address,
            &[&reward_vault_key],
        )
        .await?;
    let depositor_cookie = WalletCookie {
        address: depositor.pubkey(),
        signer: clone_keypair(&depositor),
    };
    let voter_cookie = WalletCookie {
        address: voter.pubkey(),
        signer: clone_keypair(&voter),
    };
    let voter_weight_record_cookie_depositor = bonk_plugin_test
        .with_voter_weight_record(&registrar_cookie, &depositor_cookie)
        .await?;
    let voter_weight_record_cookie_voter = bonk_plugin_test
        .with_voter_weight_record(&registrar_cookie, &voter_cookie)
        .await?;

    let token_owner_record_cookie_depositor = bonk_plugin_test
        .governance
        .with_token_owner_record(&realm_cookie, &depositor_cookie)
        .await?;
    let token_owner_record_cookie_voter = bonk_plugin_test
        .governance
        .with_token_owner_record(&realm_cookie, &voter_cookie)
        .await?;

    let token_account_cookie = bonk_plugin_test
        .bench
        .with_token_account(&realm_cookie.account.community_mint)
        .await?;

    let governance_key = get_governance_address(
        &bonk_plugin_test.governance.program_id,
        &realm_cookie.address,
        &token_account_cookie.address,
    );

    let proposal_governing_token_mint = realm_cookie.account.community_mint;
    let proposal_seed = Pubkey::new_unique();

    let proposal_key = get_proposal_address(
        &bonk_plugin_test.governance.program_id,
        &governance_key,
        &proposal_governing_token_mint,
        &proposal_seed,
    );
    let update_voter_weight_record_create_proposal_ix = bonk_plugin_test
        .update_voter_weight_record(
            &registrar_cookie,
            &voter_weight_record_cookie_depositor,
            // for proposals we can use TOR
            &token_owner_record_cookie_depositor.address,
            &token_owner_record_cookie_depositor,
            proposal_key,
            gpl_bonk_plugin::state::VoterWeightAction::CreateProposal,
            Some(proposal_key),
            &clone_keypair(&depositor),
            governance_key,
            &None,
        )
        .await?;
    let proposal_cookie = bonk_plugin_test
        .governance
        .with_proposal(
            &realm_cookie,
            &voter_weight_record_cookie_depositor,
            &depositor,
            update_voter_weight_record_create_proposal_ix,
            &proposal_seed,
            &proposal_key,
            token_account_cookie,
        )
        .await?;

    let update_voter_weight_record_cast_vote_ix = bonk_plugin_test
        .update_voter_weight_record(
            &registrar_cookie,
            &voter_weight_record_cookie_voter,
            // for proposals we can use TOR
            &token_owner_record_cookie_voter.address,
            &token_owner_record_cookie_voter,
            proposal_key,
            gpl_bonk_plugin::state::VoterWeightAction::CastVote,
            Some(proposal_key),
            &clone_keypair(&voter),
            governance_key,
            &Some(vec![stake_pool_reciept_voter]),
        )
        .await?;

    bonk_plugin_test
        .governance
        .cast_vote(
            &realm_cookie,
            &proposal_cookie,
            &clone_keypair(&voter),
            &clone_keypair(&voter),
            &Some(voter_weight_record_cookie_voter.address),
            &None,
            &token_owner_record_cookie_voter,
            Some(update_voter_weight_record_cast_vote_ix),
        )
        .await?;

    // Assert
    // proposer

    let voter_weight_record = bonk_plugin_test
        .get_voter_weight_record(&voter_weight_record_cookie_depositor.address)
        .await;
    let clock = bonk_plugin_test.bench.get_clock().await;
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));

    assert_eq!(
        voter_weight_record.weight_action,
        Some(gpl_bonk_plugin::state::VoterWeightAction::CreateProposal.into())
    );
    assert_eq!(voter_weight_record.weight_action_target, Some(proposal_key));

    // voter
    let voter_weight_record = bonk_plugin_test
        .get_voter_weight_record(&voter_weight_record_cookie_voter.address)
        .await;
    let clock = bonk_plugin_test.bench.get_clock().await;
    assert_eq!(voter_weight_record.voter_weight_expiry, Some(clock.slot));

    assert_eq!(
        voter_weight_record.weight_action,
        Some(gpl_bonk_plugin::state::VoterWeightAction::CastVote.into())
    );
    assert_eq!(voter_weight_record.weight_action_target, Some(proposal_key));

    Ok(())
}

#[tokio::test]
async fn test_update_voter_weight_record_vote_invalid_action() -> Result<(), TransportError> {
    // Arrange
    let mut bonk_plugin_test = BonkPluginTest::start_new().await;

    let realm_cookie = bonk_plugin_test.governance.with_realm().await?;
    let mut spl_token_staking_cookie = SplTokenStakingCookie::new(bonk_plugin_test.bench.clone());
    let stake_pool_pubkey = spl_token_staking_cookie
        .with_stake_pool(&realm_cookie.community_mint_cookie.address)
        .await?;
    let registrar_cookie = bonk_plugin_test
        .with_registrar(&realm_cookie, &stake_pool_pubkey)
        .await?;

    let depositor = Keypair::new();
    airdrop(
        &mut bonk_plugin_test.bench.context.borrow_mut(),
        &depositor.pubkey(),
        sol_to_lamports(10.0),
    )
    .await?;
    let community_mint_cookie = &realm_cookie.community_mint_cookie;
    let token_account_cookie = bonk_plugin_test
        .governance
        .bench
        .with_tokens(&community_mint_cookie, &depositor.pubkey(), 100)
        .await?;

    let stake_pool_reciept = find_stake_receipt_key(
        depositor.pubkey(),
        stake_pool_pubkey,
        0,
        spl_token_staking_cookie.program_id,
    );
    let reward_vault_key = find_reward_vault_key(
        stake_pool_pubkey,
        community_mint_cookie.address,
        spl_token_staking_cookie.program_id,
    );
    spl_token_staking_cookie
        .deposit_into_stake_pool(
            &depositor,
            &stake_pool_pubkey,
            &stake_pool_reciept,
            &token_account_cookie.address,
            &[&reward_vault_key],
        )
        .await?;
    let depositor_cookie = WalletCookie {
        address: depositor.pubkey(),
        signer: clone_keypair(&depositor),
    };
    let voter_weight_record_cookie_depositor = bonk_plugin_test
        .with_voter_weight_record(&registrar_cookie, &depositor_cookie)
        .await?;

    let token_owner_record_cookie_depositor = bonk_plugin_test
        .governance
        .with_token_owner_record(&realm_cookie, &depositor_cookie)
        .await?;

    let token_account_cookie = bonk_plugin_test
        .bench
        .with_token_account(&realm_cookie.account.community_mint)
        .await?;

    let governance_key = get_governance_address(
        &bonk_plugin_test.governance.program_id,
        &realm_cookie.address,
        &token_account_cookie.address,
    );

    let proposal_governing_token_mint = realm_cookie.account.community_mint;
    let proposal_seed = Pubkey::new_unique();

    let proposal_key = get_proposal_address(
        &bonk_plugin_test.governance.program_id,
        &governance_key,
        &proposal_governing_token_mint,
        &proposal_seed,
    );
    let update_voter_weight_record_create_proposal_ix = bonk_plugin_test
        .update_voter_weight_record(
            &registrar_cookie,
            &voter_weight_record_cookie_depositor,
            // for proposals we can use TOR
            &token_owner_record_cookie_depositor.address,
            &token_owner_record_cookie_depositor,
            proposal_key,
            gpl_bonk_plugin::state::VoterWeightAction::CreateProposal,
            Some(proposal_key),
            &clone_keypair(&depositor),
            governance_key,
            &None,
        )
        .await?;
    let proposal_cookie = bonk_plugin_test
        .governance
        .with_proposal(
            &realm_cookie,
            &voter_weight_record_cookie_depositor,
            &depositor,
            update_voter_weight_record_create_proposal_ix,
            &proposal_seed,
            &proposal_key,
            token_account_cookie,
        )
        .await?;

    let err = bonk_plugin_test
        .governance
        .cast_vote(
            &realm_cookie,
            &proposal_cookie,
            &clone_keypair(&depositor),
            &clone_keypair(&depositor),
            &Some(voter_weight_record_cookie_depositor.address),
            &None,
            &token_owner_record_cookie_depositor,
            None,
        )
        .await
        .err()
        .unwrap();

    assert_gov_err(err, GovernanceError::VoterWeightRecordInvalidAction);

    Ok(())
}
