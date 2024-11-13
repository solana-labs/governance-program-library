use crate::program_test::program_test_bench::MintType;
use program_test::token_voter_test::TokenVoterTest;
use program_test::tools::*;
use solana_program_test::*;
use solana_sdk::instruction::InstructionError;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transport::TransportError;
mod program_test;
use crate::program_test::program_test_bench::TokenAccountCookie;
use anchor_lang::AnchorDeserialize;

#[tokio::test]
async fn test_cast_token_vote() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new().await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let first_user_cookie = token_voter_test.users.first().unwrap();

    let mut mint_iter = token_voter_test.mints.iter();
    let first_mint_cookie = mint_iter.next().unwrap();
    let second_mint_cookie = mint_iter.next().unwrap();

    let voter_cookie = token_voter_test
        .with_voter(&registrar_cookie, first_user_cookie)
        .await?;

    let max_voter_weight_record_cookie = token_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;
    let token_account_keypair = Keypair::new();
    let council_mint_cookie = realm_cookie.council_mint_cookie.as_ref().unwrap();

    token_voter_test
        .governance
        .bench
        .create_token_account(
            &token_account_keypair,
            &council_mint_cookie.address,
            &first_user_cookie.key.pubkey(),
            &MintType::SplToken,
            true,
        )
        .await?;
    token_voter_test
        .governance
        .bench
        .mint_tokens(
            &council_mint_cookie.address,
            &council_mint_cookie.mint_authority,
            &token_account_keypair.pubkey(),
            100,
            &MintType::SplToken,
            &first_user_cookie.key.pubkey(),
            false,
        )
        .await?;

    let _token_account_cookie = TokenAccountCookie {
        address: token_account_keypair.pubkey(),
    };

    let _voting_mint_config = token_voter_test
        .configure_mint_config(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            first_mint_cookie,
            0, // no digit shift
        )
        .await?;

    let _second_voting_mint_config = token_voter_test
        .configure_mint_config(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            second_mint_cookie,
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

    token_voter_test
        .deposit_entry(
            &registrar_cookie,
            &voter_cookie,
            &first_user_cookie,
            &token_owner_record_cookie,
            &second_mint_cookie,
            &spl_token::id(),
            1,
            amount_deposited,
            None,
        )
        .await?;

    let proposal_cookie = token_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    token_voter_test
        .governance
        .cast_vote(
            &realm_cookie,
            &proposal_cookie,
            &voter_cookie,
            &first_user_cookie.key,
            &first_user_cookie.key,
            &max_voter_weight_record_cookie.address,
            &token_owner_record_cookie,
        )
        .await?;

    token_voter_test.bench.advance_clock().await;

    let err = token_voter_test
        .governance
        .cast_vote(
            &realm_cookie,
            &proposal_cookie,
            &voter_cookie,
            &first_user_cookie.key,
            &first_user_cookie.key,
            &max_voter_weight_record_cookie.address,
            &token_owner_record_cookie,
        )
        .await
        .err()
        .unwrap();

    // Token Owner already voted on the Proposal
    assert_ix_err_transport(err, InstructionError::Custom(519));

    let voter_weight_record = token_voter_test
        .get_voter_weight_record(&voter_cookie.voter_weight_record)
        .await;

    let proposal_data = token_voter_test
        .bench
        .get_account_data(proposal_cookie.address)
        .await;
    let mut data_slice: &[u8] = &proposal_data;
    let proposal_state: spl_governance::state::proposal::ProposalV2 =
        spl_governance::state::proposal::ProposalV2::deserialize(&mut data_slice).unwrap();
    assert_eq!(
        proposal_state.options[0].vote_weight,
        voter_weight_record.voter_weight
    );
    assert_eq!(proposal_state.deny_vote_weight.unwrap(), 0);
    Ok(())
}

#[tokio::test]
async fn test_cast_token_vote_token_extension() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new_token_extensions(None).await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let first_user_cookie = token_voter_test.users.first().unwrap();

    let mut mint_iter = token_voter_test.mints.iter();
    let first_mint_cookie = mint_iter.next().unwrap();
    let second_mint_cookie = mint_iter.next().unwrap();

    let voter_cookie = token_voter_test
        .with_voter(&registrar_cookie, first_user_cookie)
        .await?;

    let max_voter_weight_record_cookie = token_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;
    let token_account_keypair = Keypair::new();
    let council_mint_cookie = realm_cookie.council_mint_cookie.as_ref().unwrap();

    token_voter_test
        .governance
        .bench
        .create_token_account(
            &token_account_keypair,
            &council_mint_cookie.address,
            &first_user_cookie.key.pubkey(),
            &MintType::SplToken,
            true,
        )
        .await?;
    token_voter_test
        .governance
        .bench
        .mint_tokens(
            &council_mint_cookie.address,
            &council_mint_cookie.mint_authority,
            &token_account_keypair.pubkey(),
            100,
            &MintType::SplToken,
            &first_user_cookie.key.pubkey(),
            true,
        )
        .await?;

    token_voter_test
        .governance
        .bench
        .mint_tokens(
            &first_mint_cookie.address,
            &first_mint_cookie.mint_authority,
            &token_account_keypair.pubkey(),
            100,
            &MintType::SplTokenExtensions,
            &first_user_cookie.key.pubkey(),
            false,
        )
        .await?;

    let _token_account_cookie = TokenAccountCookie {
        address: token_account_keypair.pubkey(),
    };

    let _voting_mint_config = token_voter_test
        .configure_mint_config(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            first_mint_cookie,
            0, // no digit shift
        )
        .await?;

    let _second_voting_mint_config = token_voter_test
        .configure_mint_config(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            second_mint_cookie,
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

    let proposal_cookie = token_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    token_voter_test
        .governance
        .cast_vote(
            &realm_cookie,
            &proposal_cookie,
            &voter_cookie,
            &first_user_cookie.key,
            &first_user_cookie.key,
            &max_voter_weight_record_cookie.address,
            &token_owner_record_cookie,
        )
        .await?;

    token_voter_test.bench.advance_clock().await;

    let err = token_voter_test
        .governance
        .cast_vote(
            &realm_cookie,
            &proposal_cookie,
            &voter_cookie,
            &first_user_cookie.key,
            &first_user_cookie.key,
            &max_voter_weight_record_cookie.address,
            &token_owner_record_cookie,
        )
        .await
        .err()
        .unwrap();

    // Assert
    // Token Owner already voted on the Proposal
    assert_ix_err_transport(err, InstructionError::Custom(519));

    let voter_weight_record = token_voter_test
        .get_voter_weight_record(&voter_cookie.voter_weight_record)
        .await;

    let proposal_data = token_voter_test
        .bench
        .get_account_data(proposal_cookie.address)
        .await;
    let mut data_slice: &[u8] = &proposal_data;
    let proposal_state: spl_governance::state::proposal::ProposalV2 =
        spl_governance::state::proposal::ProposalV2::deserialize(&mut data_slice).unwrap();
    // println!("proposal_state: {:?}", proposal_state.options[0]);
    assert_eq!(
        proposal_state.options[0].vote_weight,
        voter_weight_record.voter_weight
    );
    assert_eq!(proposal_state.deny_vote_weight.unwrap(), 0);

    Ok(())
}

#[tokio::test]
async fn test_cast_token_vote_token_extension_transfer_fees() -> Result<(), TransportError> {
    // Arrange
    let mut token_voter_test = TokenVoterTest::start_new_token_extensions(None).await;

    let realm_cookie = token_voter_test.governance.with_realm().await?;

    let registrar_cookie = token_voter_test.with_registrar(&realm_cookie).await?;
    let governance_program_cookie = token_voter_test.with_governance_program(None).await;

    let first_user_cookie = token_voter_test.users.first().unwrap();

    let mut mint_iter = token_voter_test.mints.iter();
    let first_mint_cookie = mint_iter.next().unwrap();
    let second_mint_cookie = mint_iter.next().unwrap();

    let voter_cookie = token_voter_test
        .with_voter(&registrar_cookie, first_user_cookie)
        .await?;

    let max_voter_weight_record_cookie = token_voter_test
        .with_max_voter_weight_record(&registrar_cookie)
        .await?;
    let token_account_keypair = Keypair::new();
    let council_mint_cookie = realm_cookie.council_mint_cookie.as_ref().unwrap();

    token_voter_test
        .governance
        .bench
        .create_token_account(
            &token_account_keypair,
            &council_mint_cookie.address,
            &first_user_cookie.key.pubkey(),
            &MintType::SplToken,
            true,
        )
        .await?;
    token_voter_test
        .governance
        .bench
        .mint_tokens(
            &council_mint_cookie.address,
            &council_mint_cookie.mint_authority,
            &token_account_keypair.pubkey(),
            100,
            &MintType::SplToken,
            &first_user_cookie.key.pubkey(),
            true,
        )
        .await?;

    token_voter_test
        .governance
        .bench
        .mint_tokens(
            &first_mint_cookie.address,
            &first_mint_cookie.mint_authority,
            &token_account_keypair.pubkey(),
            100,
            &MintType::SplTokenExtensionsWithTransferFees,
            &first_user_cookie.key.pubkey(),
            false,
        )
        .await?;

    let _token_account_cookie = TokenAccountCookie {
        address: token_account_keypair.pubkey(),
    };

    let _voting_mint_config = token_voter_test
        .configure_mint_config(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            first_mint_cookie,
            0, // no digit shift
        )
        .await?;

    let _second_voting_mint_config = token_voter_test
        .configure_mint_config(
            &registrar_cookie,
            &governance_program_cookie,
            &max_voter_weight_record_cookie,
            second_mint_cookie,
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

    let proposal_cookie = token_voter_test
        .governance
        .with_proposal(&realm_cookie)
        .await?;

    token_voter_test
        .governance
        .cast_vote(
            &realm_cookie,
            &proposal_cookie,
            &voter_cookie,
            &first_user_cookie.key,
            &first_user_cookie.key,
            &max_voter_weight_record_cookie.address,
            &token_owner_record_cookie,
        )
        .await?;

    token_voter_test.bench.advance_clock().await;

    let err = token_voter_test
        .governance
        .cast_vote(
            &realm_cookie,
            &proposal_cookie,
            &voter_cookie,
            &first_user_cookie.key,
            &first_user_cookie.key,
            &max_voter_weight_record_cookie.address,
            &token_owner_record_cookie,
        )
        .await
        .err()
        .unwrap();

    // Token Owner already voted on the Proposal
    assert_ix_err_transport(err, InstructionError::Custom(519));

    let voter_weight_record = token_voter_test
        .get_voter_weight_record(&voter_cookie.voter_weight_record)
        .await;

    let proposal_data = token_voter_test
        .bench
        .get_account_data(proposal_cookie.address)
        .await;
    let mut data_slice: &[u8] = &proposal_data;
    let proposal_state: spl_governance::state::proposal::ProposalV2 =
        spl_governance::state::proposal::ProposalV2::deserialize(&mut data_slice).unwrap();

    assert_eq!(
        proposal_state.options[0].vote_weight,
        voter_weight_record.voter_weight
    );
    assert_eq!(proposal_state.deny_vote_weight.unwrap(), 0);

    Ok(())
}
