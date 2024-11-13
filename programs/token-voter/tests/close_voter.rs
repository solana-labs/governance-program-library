use std::borrow::BorrowMut;

use gpl_token_voter::error::TokenVoterError;
use program_test::token_voter_test::TokenVoterTest;
use program_test::tools::*;
use solana_program_test::*;
use solana_sdk::transport::TransportError;
mod program_test;

#[tokio::test]
async fn test_close_with_token_extensions() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new_token_extensions(None).await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let mut users_iter = token_voter_test.users.iter();
    let first_user_cookie = users_iter.next().unwrap();

    let mut mint_iter = token_voter_test.mints.iter();
    let first_mint_cookie = mint_iter.next().unwrap();
    let second_mint_cookie = mint_iter.next().unwrap();

    let voter_cookie = token_voter_test
        .with_voter(&registrar_cookie, first_user_cookie)
        .await?;

    let max_voter_weight_record_cookie = token_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;

    let _voting_mint_config = token_voter_test
        .configure_mint_config(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            &first_mint_cookie,
            0, // no digit shift
        )
        .await?;

    let _second_voting_mint_config = token_voter_test
        .configure_mint_config(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            &second_mint_cookie,
            0, // no digit shift
        )
        .await?;

    let token_owner_record_cookie = token_voter_test
        .governance
        .with_token_owner_record_using_user_cookie(&realm_cookie, &first_user_cookie)
        .await?;

    let amount_deposited = 10_u64;
    token_voter_test
        .deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie,
            &spl_token_2022::id(),
            0,
            amount_deposited,
            None,
        )
        .await?;

    token_voter_test.bench.advance_clock().await;

    token_voter_test
        .deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &second_mint_cookie,
            &spl_token_2022::id(),
            1,
            amount_deposited,
            None,
        )
        .await?;

    token_voter_test.bench.advance_clock().await;

    let err = token_voter_test
        .close_voter_account(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_voter_test.mints,
            &spl_token_2022::id(),
        )
        .await
        .err()
        .unwrap();

    assert_token_voter_err(err, TokenVoterError::VotingTokenNonZero);

    token_voter_test
        .withdraw_deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie,
            &spl_token_2022::id(),
            0,
            amount_deposited,
            None,
        )
        .await?;

    token_voter_test.bench.advance_clock().await;

    let err = token_voter_test
        .close_voter_account(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_voter_test.mints,
            &spl_token_2022::id(),
        )
        .await
        .err()
        .unwrap();
    assert_token_voter_err(err, TokenVoterError::VotingTokenNonZero);

    token_voter_test
        .withdraw_deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &second_mint_cookie,
            &spl_token_2022::id(),
            1,
            amount_deposited,
            None,
        )
        .await?;
    token_voter_test.bench.advance_clock().await;

    token_voter_test
        .close_voter_account(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_voter_test.mints,
            &spl_token_2022::id(),
        )
        .await?;

    // Assert
    // voter_data should be None.
    let voter_data = token_voter_test
        .bench
        .borrow_mut()
        .context
        .borrow_mut()
        .banks_client
        .get_account(voter_cookie.address)
        .await
        .unwrap();

    assert_eq!(voter_data, None);

    Ok(())
}
