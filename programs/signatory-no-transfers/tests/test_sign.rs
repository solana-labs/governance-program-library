use no_transfers_signatory::{
    error::TransactionCheckerError, instructions::signing_authority_address,
};

use program_test::tools::assert_transaction_checker_err;
use solana_program_test::tokio;
use solana_sdk::transport::TransportError;

use crate::program_test::no_transfers_test::NoTransfersSignatoryTest;

mod program_test;

type TestOutcome = Result<(), TransportError>;

#[tokio::test]
async fn test_sign() -> TestOutcome {
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

    signatory_test
        .governance
        .with_governance_required_signatory(
            &token_owner_record_cookie,
            &mut governance_cookie,
            &signing_authority_address(),
        )
        .await
        .unwrap();

    let mut proposal_cookie = signatory_test
        .governance
        .with_proposal(&token_owner_record_cookie, &mut governance_cookie)
        .await
        .unwrap();

    let signatory_record_cookie = signatory_test
        .governance
        .with_signatory_record_from_governance(
            &proposal_cookie,
            &governance_cookie,
            &signing_authority_address(),
        )
        .await
        .unwrap();

    let proposal_transaction_cookie = signatory_test
        .governance
        .with_nop_transaction(&mut proposal_cookie, &token_owner_record_cookie, 0, None)
        .await
        .unwrap();

    let _ = signatory_test
        .check_transaction(
            &realm_cookie,
            &proposal_cookie,
            &proposal_transaction_cookie,
        )
        .await
        .unwrap();

    // Act
    signatory_test
        .sign(&realm_cookie, &proposal_cookie, &signatory_record_cookie)
        .await
        .unwrap();

    // Assert
    let proposal_account = signatory_test
        .governance
        .get_proposal_account(&proposal_cookie.address)
        .await;

    assert_eq!(proposal_account.signatories_count, 1);
    assert_eq!(proposal_account.signatories_signed_off_count, 1);

    Ok(())
}

#[tokio::test]
async fn test_sign_not_fully_verified_err() -> TestOutcome {
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

    signatory_test
        .governance
        .with_governance_required_signatory(
            &token_owner_record_cookie,
            &mut governance_cookie,
            &signing_authority_address(),
        )
        .await
        .unwrap();

    let mut proposal_cookie = signatory_test
        .governance
        .with_proposal(&token_owner_record_cookie, &mut governance_cookie)
        .await
        .unwrap();

    let signatory_record_cookie = signatory_test
        .governance
        .with_signatory_record_from_governance(
            &proposal_cookie,
            &governance_cookie,
            &signing_authority_address(),
        )
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

    let _ = signatory_test
        .check_transaction(
            &realm_cookie,
            &proposal_cookie,
            &proposal_transaction_cookie,
        )
        .await
        .unwrap();

    // Act
    let err = signatory_test
        .sign(&realm_cookie, &proposal_cookie, &signatory_record_cookie)
        .await
        .err()
        .unwrap();

    // Assert
    assert_transaction_checker_err(err, TransactionCheckerError::ProposalNotFullyChecked);

    Ok(())
}

#[tokio::test]
async fn test_sign_bad_transaction_err() -> TestOutcome {
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

    signatory_test
        .governance
        .with_governance_required_signatory(
            &token_owner_record_cookie,
            &mut governance_cookie,
            &signing_authority_address(),
        )
        .await
        .unwrap();

    let mut proposal_cookie = signatory_test
        .governance
        .with_proposal(&token_owner_record_cookie, &mut governance_cookie)
        .await
        .unwrap();

    let signatory_record_cookie = signatory_test
        .governance
        .with_signatory_record_from_governance(
            &proposal_cookie,
            &governance_cookie,
            &signing_authority_address(),
        )
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

    let _ = signatory_test
        .check_transaction(
            &realm_cookie,
            &proposal_cookie,
            &proposal_transaction_cookie,
        )
        .await
        .unwrap();

    // Act
    let err = signatory_test
        .sign(&realm_cookie, &proposal_cookie, &signatory_record_cookie)
        .await
        .err()
        .unwrap();

    // Assert
    assert_transaction_checker_err(err, TransactionCheckerError::ProposalRejected);

    Ok(())
}
