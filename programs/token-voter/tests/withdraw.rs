use program_test::token_voter_test::TokenVoterTest;
use program_test::tools::*;
use solana_program_test::*;
use solana_sdk::transport::TransportError;
use token_voter::error::TokenVoterError;

mod program_test;

#[tokio::test]
async fn test_withdraw_with_token_extensions() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new_token_extensions().await;

    let realm_cookie = token_voter_test.governance.with_realm_token_extension().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let first_user_cookie = token_voter_test.users.first().unwrap();
    let first_mint_cookie = token_voter_test.mints.first().unwrap();

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
            first_mint_cookie,
            0, // no digit shift
        )
        .await?;

    let token_owner_record_cookie = token_voter_test
        .governance
        .with_token_owner_record_using_user_cookie(&realm_cookie, &first_user_cookie).await?;
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
        )
        .await?;

    let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;
    let first_vault_balance = token_voter_test.vault_balance(&voter_cookie, &first_mint_cookie, &spl_token_2022::id()).await;

    assert_eq!(voter_data.deposits.first().unwrap().amount_deposited_native, first_vault_balance);
    assert_eq!(first_vault_balance, amount_deposited);

    token_voter_test.bench.advance_clock().await;

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
        )
        .await?;
    
    // Assert
    let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;

    assert_eq!(voter_data.registrar, registrar_cookie.address);
    // println!("{:?}", voter_data);
    assert_eq!(voter_data.deposits.first().unwrap().amount_deposited_native, 0);
    assert_eq!(voter_data.deposits.len(), 1);

    let first_vault_balance = token_voter_test.vault_balance(&voter_cookie, &first_mint_cookie, &spl_token_2022::id()).await;
    assert_eq!(first_vault_balance, 0);

    Ok(())
}


#[tokio::test]
async fn test_withdraw() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new().await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let first_user_cookie = token_voter_test.users.first().unwrap();
    let first_mint_cookie = token_voter_test.mints.first().unwrap();

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
            first_mint_cookie,
            0, // no digit shift
        )
        .await?;

    let token_owner_record_cookie = token_voter_test
        .governance
        .with_token_owner_record_using_user_cookie(&realm_cookie, &first_user_cookie).await?;
    let amount_deposited = 10_u64;
    token_voter_test
        .deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie,
            &spl_token::id(),
            0,
            amount_deposited,
        )
        .await?;


    token_voter_test.bench.advance_clock().await;

    token_voter_test
        .withdraw_deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie,
            &spl_token::id(),
            0,
            amount_deposited,
        )
        .await?;
    
    // Assert
    let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;

    assert_eq!(voter_data.registrar, registrar_cookie.address);
    println!("{:?}", voter_data);
    assert_eq!(voter_data.deposits.first().unwrap().amount_deposited_native, 0);
    assert_eq!(voter_data.deposits.len(), 1);

    Ok(())
}

#[tokio::test]
async fn test_withdraw_fail_to_withdraw_in_same_slot() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new().await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let first_user_cookie = token_voter_test.users.first().unwrap();
    let first_mint_cookie = token_voter_test.mints.first().unwrap();

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
            first_mint_cookie,
            0, // no digit shift
        )
        .await?;

    let token_owner_record_cookie = token_voter_test
        .governance
        .with_token_owner_record_using_user_cookie(&realm_cookie, &first_user_cookie).await?;
    let amount_deposited = 10_u64;
    token_voter_test
        .deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie,
            &spl_token::id(),
            0,
            amount_deposited,
        )
        .await?;

    let err = token_voter_test
        .withdraw_deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie,
            &spl_token::id(),
            0,
            amount_deposited,
        )
                .await
                .err()
                .unwrap();
    
    // Assert
    assert_token_voter_err(err, TokenVoterError::CannotWithdraw);

    Ok(())
}


#[tokio::test]
async fn test_withdraw_multi_deposit_and_withdraw() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new().await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let first_user_cookie = token_voter_test.users.first().unwrap();
    let first_mint_cookie = token_voter_test.mints.first().unwrap();

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
            first_mint_cookie,
            0, // no digit shift
        )
        .await?;

    let token_owner_record_cookie = token_voter_test
        .governance
        .with_token_owner_record_using_user_cookie(&realm_cookie, &first_user_cookie).await?;
    let amount_deposited = 5_u64;
    token_voter_test
        .deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie,
            &spl_token::id(),
            0,
            amount_deposited,
        )
        .await?;

    token_voter_test.bench.advance_clock().await;

    token_voter_test
        .deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie,
            &spl_token::id(),
            0,
            amount_deposited,
        )
        .await?;


    token_voter_test.bench.advance_clock().await;

    token_voter_test
        .withdraw_deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie,
            &spl_token::id(),
            0,
            amount_deposited * 2,
        )
        .await?;
    
    // Assert
    let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;

    assert_eq!(voter_data.registrar, registrar_cookie.address);
    println!("{:?}", voter_data);
    assert_eq!(voter_data.deposits.first().unwrap().amount_deposited_native, 0);
    assert_eq!(voter_data.deposits.len(), 1);

    Ok(())
}