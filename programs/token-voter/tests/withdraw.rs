use crate::program_test::program_test_bench::MintType;
use anchor_spl::associated_token;
use gpl_token_voter::error::TokenVoterError;
use program_test::token_voter_test::TokenVoterTest;
use program_test::tools::*;
use solana_program_test::*;
use solana_sdk::instruction::InstructionError;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::transport::TransportError;

mod program_test;

#[tokio::test]
async fn test_withdraw_with_token_extensions_transfer_hooks() -> Result<(), TransportError> {
    // Arrange
    let transfer_hook_program_id = Pubkey::new_unique();
    let mut token_voter_test =
        TokenVoterTest::start_new_token_extensions(Some(&transfer_hook_program_id)).await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let first_user_cookie = token_voter_test.users.first().unwrap();
    let first_mint_cookie_transfer_hook = token_voter_test
        .bench
        .with_mint(
            &MintType::SplTokenExtensionsWithTransferHook,
            Some(&transfer_hook_program_id),
        )
        .await
        .unwrap();
    let _first_user_cookie_token_account = token_voter_test
        .bench
        .with_tokens(
            &first_mint_cookie_transfer_hook,
            &first_user_cookie.key.pubkey(),
            100,
            &MintType::SplTokenExtensionsWithTransferHook,
            false,
        )
        .await;

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
            &first_mint_cookie_transfer_hook,
            0, // no digit shift
        )
        .await?;

    let token_owner_record_cookie = token_voter_test
        .governance
        .with_token_owner_record_using_user_cookie(&realm_cookie, &first_user_cookie)
        .await?;

    let amount_deposited = 10_u64;
    let vault = associated_token::get_associated_token_address_with_program_id(
        &voter_cookie.address,
        &first_mint_cookie_transfer_hook.address,
        &spl_token_2022::id(),
    );
    let deposit_token = associated_token::get_associated_token_address_with_program_id(
        &first_user_cookie.key.pubkey(),
        &first_mint_cookie_transfer_hook.address,
        &spl_token_2022::id(),
    );
    token_voter_test.bench.advance_clock().await;
    let additional_account_metas = token_voter_test
        .bench
        .initialize_transfer_hook_account_metas(
            &first_mint_cookie_transfer_hook.address,
            &first_mint_cookie_transfer_hook.mint_authority,
            &first_user_cookie.key,
            &transfer_hook_program_id,
            &deposit_token,
            &vault,
            &Pubkey::new_unique(),
            amount_deposited,
        )
        .await;
    token_voter_test.bench.advance_clock().await;
    token_voter_test
        .deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie_transfer_hook,
            &spl_token_2022::id(),
            0,
            amount_deposited,
            Some(additional_account_metas.to_vec()),
        )
        .await?;

    let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;
    let first_vault_balance = token_voter_test
        .vault_balance(
            &voter_cookie,
            &first_mint_cookie_transfer_hook,
            &spl_token_2022::id(),
        )
        .await;

    assert_eq!(
        voter_data.deposits.first().unwrap().amount_deposited_native,
        first_vault_balance
    );
    assert_eq!(first_vault_balance, amount_deposited);

    token_voter_test.bench.advance_clock().await;

    let err = token_voter_test
        .withdraw_deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie_transfer_hook,
            &spl_token_2022::id(),
            0,
            amount_deposited,
            None,
        )
        .await
        .err()
        .unwrap();

    assert_ix_err(err, InstructionError::Custom(2110272652));

    let additional_account_metas = token_voter_test
        .bench
        .update_transfer_hook_account_metas(
            &first_mint_cookie_transfer_hook.address,
            &first_mint_cookie_transfer_hook.mint_authority,
            &first_user_cookie.key,
            &transfer_hook_program_id,
            &vault,
            &deposit_token,
            &Pubkey::new_unique(),
            amount_deposited,
        )
        .await;

    token_voter_test
        .withdraw_deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &first_mint_cookie_transfer_hook,
            &spl_token_2022::id(),
            0,
            amount_deposited,
            Some(additional_account_metas),
        )
        .await?;

    // Assert
    let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;

    assert_eq!(voter_data.registrar, registrar_cookie.address);
    assert_eq!(
        voter_data.deposits.first().unwrap().amount_deposited_native,
        0
    );
    assert_eq!(voter_data.deposits.len(), 1);

    let first_vault_balance = token_voter_test
        .vault_balance(
            &voter_cookie,
            &first_mint_cookie_transfer_hook,
            &spl_token_2022::id(),
        )
        .await;
    assert_eq!(first_vault_balance, 0);
    Ok(())
}

#[tokio::test]
async fn test_withdraw_with_token_extensions() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new_token_extensions(None).await;

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

    let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;
    let first_vault_balance = token_voter_test
        .vault_balance(&voter_cookie, &first_mint_cookie, &spl_token_2022::id())
        .await;

    assert_eq!(
        voter_data.deposits.first().unwrap().amount_deposited_native,
        first_vault_balance
    );
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
            None,
        )
        .await?;

    // Assert
    let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;

    assert_eq!(voter_data.registrar, registrar_cookie.address);
    // println!("{:?}", voter_data);
    assert_eq!(
        voter_data.deposits.first().unwrap().amount_deposited_native,
        0
    );
    assert_eq!(voter_data.deposits.len(), 1);

    let first_vault_balance = token_voter_test
        .vault_balance(&voter_cookie, &first_mint_cookie, &spl_token_2022::id())
        .await;
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
            &spl_token::id(),
            0,
            amount_deposited,
            None,
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
            None,
        )
        .await?;

    // Assert
    let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;

    assert_eq!(voter_data.registrar, registrar_cookie.address);
    println!("{:?}", voter_data);
    assert_eq!(
        voter_data.deposits.first().unwrap().amount_deposited_native,
        0
    );
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
            &spl_token::id(),
            0,
            amount_deposited,
            None,
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
            None,
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
        .with_token_owner_record_using_user_cookie(&realm_cookie, &first_user_cookie)
        .await?;
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
            &first_mint_cookie,
            &spl_token::id(),
            0,
            amount_deposited,
            None,
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
            None,
        )
        .await?;

    // Assert
    let voter_data = token_voter_test.get_voter(&voter_cookie.address).await;

    assert_eq!(voter_data.registrar, registrar_cookie.address);
    println!("{:?}", voter_data);
    assert_eq!(
        voter_data.deposits.first().unwrap().amount_deposited_native,
        0
    );
    assert_eq!(voter_data.deposits.len(), 1);

    Ok(())
}
