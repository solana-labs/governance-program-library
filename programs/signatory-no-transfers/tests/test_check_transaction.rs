use anchor_lang::prelude::Pubkey;
use no_transfers_signatory::{
    accounts::CheckTransaction, error::TransactionCheckerError, state::TransactionsChecked,
};
use program_test::tools::assert_transaction_checker_err;
use solana_program_test::tokio;
use solana_sdk::{signer::Signer, transport::TransportError};

use crate::program_test::no_transfers_test::NoTransfersSignatoryTest;

mod program_test;

type TestOutcome = Result<(), TransportError>;

#[tokio::test]
async fn test_check_null_transaction() -> TestOutcome {
    // Arrange
    let mut signatory_test = NoTransfersSignatoryTest::start_new().await;

    let realm_cookie = signatory_test.governance.with_realm().await;
    let governed_account_cookie = signatory_test.governance.with_governed_account().await;

    let token_owner_record_cookie = signatory_test
        .governance
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    let mut governance_cookie = signatory_test
        .governance
        .with_governance(
            &realm_cookie,
            &governed_account_cookie,
            &token_owner_record_cookie,
        )
        .await
        .unwrap();

    let mut proposal_cookie = signatory_test
        .governance
        .with_proposal(&token_owner_record_cookie, &mut governance_cookie)
        .await
        .unwrap();

    let proposal_transaction_cookie = signatory_test
        .governance
        .with_nop_transaction(&mut proposal_cookie, &token_owner_record_cookie, 0, None)
        .await
        .unwrap();

    // Act
    let checked_transaction_cookie = signatory_test
        .check_transaction(
            &realm_cookie,
            &proposal_cookie,
            &proposal_transaction_cookie,
        )
        .await
        .unwrap();

    // Assert
    let check_transaction_account = signatory_test
        .bench
        .get_anchor_account::<TransactionsChecked>(checked_transaction_cookie.address)
        .await;

    assert!(!check_transaction_account.reject);
    assert_eq!(check_transaction_account.transactions_checked[0], 1);
    assert_eq!(
        check_transaction_account.payer,
        signatory_test.bench.payer.pubkey()
    );

    Ok(())
}

#[tokio::test]
async fn test_check_illegal_token_transaction() -> TestOutcome {
    // Arrange
    let mut signatory_test = NoTransfersSignatoryTest::start_new().await;

    let realm_cookie = signatory_test.governance.with_realm().await;
    let goverened_token_cookie = signatory_test.governance.with_governed_token().await;

    let token_owner_record_cookie = signatory_test
        .governance
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    let mut governance_cookie = signatory_test
        .governance
        .with_token_governance(
            &realm_cookie,
            &goverened_token_cookie,
            &token_owner_record_cookie,
        )
        .await
        .unwrap();

    let mut proposal_cookie = signatory_test
        .governance
        .with_proposal(&token_owner_record_cookie, &mut governance_cookie)
        .await
        .unwrap();

    let proposal_transaction_cookie = signatory_test
        .governance
        .with_transfer_tokens_transaction(
            &goverened_token_cookie,
            &mut proposal_cookie,
            &token_owner_record_cookie,
            Some(0),
        )
        .await
        .unwrap();

    // Act
    let checked_transaction_cookie = signatory_test
        .check_transaction(
            &realm_cookie,
            &proposal_cookie,
            &proposal_transaction_cookie,
        )
        .await
        .unwrap();

    // Assert
    let check_transaction_account = signatory_test
        .bench
        .get_anchor_account::<TransactionsChecked>(checked_transaction_cookie.address)
        .await;

    assert!(check_transaction_account.reject);
    assert_eq!(check_transaction_account.transactions_checked[0], 1);
    assert_eq!(
        check_transaction_account.payer,
        signatory_test.bench.payer.pubkey()
    );

    Ok(())
}

#[tokio::test]
async fn test_check_non_governance_token_transaction() -> TestOutcome {
    // Arrange
    let mut signatory_test = NoTransfersSignatoryTest::start_new().await;

    let realm_cookie = signatory_test.governance.with_realm().await;
    let goverened_token_cookie = signatory_test.governance.with_governed_token().await;

    let token_owner_record_cookie = signatory_test
        .governance
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    let mut governance_cookie = signatory_test
        .governance
        .with_token_governance(
            &realm_cookie,
            &goverened_token_cookie,
            &token_owner_record_cookie,
        )
        .await
        .unwrap();

    let mut proposal_cookie = signatory_test
        .governance
        .with_proposal(&token_owner_record_cookie, &mut governance_cookie)
        .await
        .unwrap();

    let fake_source_account = Pubkey::new_unique();
    let fake_dest_account = Pubkey::new_unique();

    let mut transfer_ixn = spl_token::instruction::transfer(
        &spl_token::id(),
        &fake_source_account,
        &fake_dest_account,
        &proposal_cookie.account.governance,
        &[],
        15,
    )
    .unwrap();

    let proposal_transaction_cookie = signatory_test
        .governance
        .with_proposal_transaction(
            &mut proposal_cookie,
            &token_owner_record_cookie,
            0,
            Some(0),
            &mut transfer_ixn,
            None,
        )
        .await
        .unwrap();

    // Act
    let checked_transaction_cookie = signatory_test
        .check_transaction(
            &realm_cookie,
            &proposal_cookie,
            &proposal_transaction_cookie,
        )
        .await
        .unwrap();

    // Assert
    let check_transaction_account = signatory_test
        .bench
        .get_anchor_account::<TransactionsChecked>(checked_transaction_cookie.address)
        .await;

    assert!(!check_transaction_account.reject);
    assert_eq!(check_transaction_account.transactions_checked[0], 1);
    assert_eq!(
        check_transaction_account.payer,
        signatory_test.bench.payer.pubkey()
    );

    Ok(())
}

#[tokio::test]
async fn test_check_two_transactions() -> TestOutcome {
    // Arrange
    let mut signatory_test = NoTransfersSignatoryTest::start_new().await;

    let realm_cookie = signatory_test.governance.with_realm().await;
    let governed_account_cookie = signatory_test.governance.with_governed_account().await;

    let token_owner_record_cookie = signatory_test
        .governance
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    let mut governance_cookie = signatory_test
        .governance
        .with_governance(
            &realm_cookie,
            &governed_account_cookie,
            &token_owner_record_cookie,
        )
        .await
        .unwrap();

    let mut proposal_cookie = signatory_test
        .governance
        .with_proposal(&token_owner_record_cookie, &mut governance_cookie)
        .await
        .unwrap();

    let proposal_transaction_cookie = signatory_test
        .governance
        .with_nop_transaction(&mut proposal_cookie, &token_owner_record_cookie, 0, None)
        .await
        .unwrap();

    let proposal_transaction_cookie_2 = signatory_test
        .governance
        .with_nop_transaction(&mut proposal_cookie, &token_owner_record_cookie, 0, None)
        .await
        .unwrap();

    let checked_transaction_cookie = signatory_test
        .check_transaction(
            &realm_cookie,
            &proposal_cookie,
            &proposal_transaction_cookie,
        )
        .await
        .unwrap();

    // Act
    let checked_transaction_cookie = signatory_test
        .check_transaction(
            &realm_cookie,
            &proposal_cookie,
            &proposal_transaction_cookie_2,
        )
        .await
        .unwrap();

    // Assert
    let check_transaction_account = signatory_test
        .bench
        .get_anchor_account::<TransactionsChecked>(checked_transaction_cookie.address)
        .await;

    assert!(!check_transaction_account.reject);
    assert_eq!(check_transaction_account.transactions_checked[0], 2);
    assert_eq!(
        check_transaction_account.payer,
        signatory_test.bench.payer.pubkey()
    );

    Ok(())
}

#[tokio::test]
async fn test_check_transaction_out_of_order_err() -> TestOutcome {
    // Arrange
    let mut signatory_test = NoTransfersSignatoryTest::start_new().await;

    let realm_cookie = signatory_test.governance.with_realm().await;
    let governed_account_cookie = signatory_test.governance.with_governed_account().await;

    let token_owner_record_cookie = signatory_test
        .governance
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    let mut governance_cookie = signatory_test
        .governance
        .with_governance(
            &realm_cookie,
            &governed_account_cookie,
            &token_owner_record_cookie,
        )
        .await
        .unwrap();

    let mut proposal_cookie = signatory_test
        .governance
        .with_proposal(&token_owner_record_cookie, &mut governance_cookie)
        .await
        .unwrap();

    let proposal_transaction_cookie = signatory_test
        .governance
        .with_nop_transaction(&mut proposal_cookie, &token_owner_record_cookie, 0, None)
        .await
        .unwrap();

    let proposal_transaction_cookie_2 = signatory_test
        .governance
        .with_nop_transaction(&mut proposal_cookie, &token_owner_record_cookie, 0, None)
        .await
        .unwrap();

    // Act
    let err = signatory_test
        .check_transaction(
            &realm_cookie,
            &proposal_cookie,
            &proposal_transaction_cookie_2,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_transaction_checker_err(err, TransactionCheckerError::WrongTransaction);

    Ok(())
}
