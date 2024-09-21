#[tokio::test]
async fn test_withdraw_governance_token() {
    let program_id = Pubkey::new_unique();
    let registrar_config_account = Pubkey::new_unique();
    let voter_weight_record_account = Pubkey::new_unique();
    let voter_token_account = Pubkey::new_unique();
    let governance_token_mint = Pubkey::new_unique();
    let token_program = Pubkey::new_unique();
    let spl_governance_program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let payer_account = Pubkey::new_unique();

    let mut test_context = ProgramTest::new(
        "registrar",
        program_id,
        processor!(process_instruction),
    );

    test_context
        .add_account_with_data(
            registrar_config_account,
            &RegistrarConfig {
                accepted_tokens: vec![governance_token_mint],
                weights: vec![10],
            },
            RegistrarConfig::LEN,
            &program_id,
        )
        .await
        .unwrap();

    test_context
        .add_account_with_data(
            voter_weight_record_account,
            &VoterWeightRecord {
                voter: payer_account,
                weight: 1000,
                last_deposit_or_withdrawal_slot: 0,
            },
            VoterWeightRecord::LEN,
            &program_id,
        )
        .await
        .unwrap();

    let (mut banks_client, payer, recent_blockhash) = test_context
        .start_with_new_bank()
        .await
        .unwrap();

    let withdraw_amount = 100;

    let withdraw_governance_token_instruction = withdraw_governance_token(
        program_id,
        voter_weight_record_account,
        voter_token_account,
        governance_token_mint,
        registrar_config_account,
        token_program,
        spl_governance_program,
        authority,
        withdraw_amount,
    );

    banks_client
        .process_transaction_with_preflight(withdraw_governance_token_instruction, &[&payer])
        .await
        .unwrap();

    let voter_weight_record_account_data = banks_client
        .get_account_data_with_borsh(voter_weight_record_account)
        .await
        .unwrap();

    assert_eq!(
        voter_weight_record_account_data,
        VoterWeightRecord {
            voter: payer_account,
            weight: 900,
            last_deposit_or_withdrawal_slot: banks_client.get_latest_slot().await.unwrap(),
        }
    );
}

#[tokio::test]
async fn test_deposit_withdraw_different_governance_tokens() {
    let program_id = Pubkey::new_unique();
    let registrar_config_account = Pubkey::new_unique();
    let voter_weight_record_account = Pubkey::new_unique();
    let voter_token_account_1 = Pubkey::new_unique();
    let voter_token_account_2 = Pubkey::new_unique();
    let governance_token_mint_1 = Pubkey::new_unique();
    let governance_token_mint_2 = Pubkey::new_unique();
    let token_program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let payer_account = Pubkey::new_unique();

    let mut test_context = ProgramTest::new(
        "registrar",
        program_id,
        processor!(process_instruction),
    );

    test_context
        .add_account_with_data(
            registrar_config_account,
            &RegistrarConfig {
                accepted_tokens: vec![governance_token_mint_1, governance_token_mint_2],
                weights: vec![10, 20],
            },
            RegistrarConfig::LEN,
            &program_id,
        )
        .await
        .unwrap();

    test_context
        .add_account_with_data(
            voter_weight_record_account,
            &VoterWeightRecord {
                voter: payer_account,
                weight: 0,
                last_deposit_or_withdrawal_slot: 0,
            },
            VoterWeightRecord::LEN,
            &program_id,
        )
        .await
        .unwrap();

    let (mut banks_client, payer, recent_blockhash) = test_context
        .start_with_new_bank()
        .await
        .unwrap();

    let deposit_amount_1 = 100;
    let deposit_amount_2 = 200;

    let deposit_governance_token_instruction_1 = deposit_governance_token(
        program_id,
        voter_weight_record_account,
        voter_token_account_1,
        governance_token_mint_1,
        registrar_config_account,
        token_program,
        authority,
        deposit_amount_1,
    );

    banks_client
        .process_transaction_with_preflight(deposit_governance_token_instruction_1, &[&payer])
        .await
        .unwrap();

    let deposit_governance_token_instruction_2 = deposit_governance_token(
        program_id,
        voter_weight_record_account,
        voter_token_account_2,
        governance_token_mint_2,
        registrar_config_account,
        token_program,
        authority,
        deposit_amount_2,
    );

    banks_client
        .process_transaction_with_preflight(deposit_governance_token_instruction_2, &[&payer])
        .await
        .unwrap();

    let voter_weight_record_account_data = banks_client
        .get_account_data_with_borsh(voter_weight_record_account)
        .await
        .unwrap();

    assert_eq!(
        voter_weight_record_account_data,
        VoterWeightRecord {
            voter: payer_account,
            weight: deposit_amount_1 * 10 + deposit_amount_2 * 20,
            last_deposit_or_withdrawal_slot: banks_client.get_latest_slot().await.unwrap(),
        }
    );

    let withdraw_amount_1 = 50;
    let withdraw_amount_2 = 100;

    let withdraw_governance_token_instruction_1 = withdraw_governance_token(
        program_id,
        voter_weight_record_account,
        voter_token_account_1,
        governance_token_mint_1,
        registrar_config_account,
        token_program,
        spl_governance_program,
        authority,
        withdraw_amount_1,
    );

    banks_client
        .process_transaction_with_preflight(withdraw_governance_token_instruction_1, &[&payer])
        .await
        .unwrap();

    let withdraw_governance_token_instruction_2 = withdraw_governance_token(
        program_id,
        voter_weight_record_account,
        voter_token_account_2,
        governance_token_mint_2,
        registrar_config_account,
        token_program,
        spl_governance_program,
        authority,
        withdraw_amount_2,
    );

    banks_client
        .process_transaction_with_preflight(withdraw_governance_token_instruction_2, &[&payer])
        .await
        .unwrap();

    let voter_weight_record_account_data = banks_client
        .get_account_data_with_borsh(voter_weight_record_account)
        .await
        .unwrap();

    assert_eq!(
        voter_weight_record_account_data,
        VoterWeightRecord {
            voter: payer_account,
            weight: (deposit_amount_1 - withdraw_amount_1) * 10
                + (deposit_amount_2 - withdraw_amount_2) * 20,
            last_deposit_or_withdrawal_slot: banks_client.get_latest_slot().await.unwrap(),
        }
    );
}

#[tokio::test]
async fn test_invalid_deposit_withdraw_amount() {
    let program_id = Pubkey::new_unique();
    let registrar_config_account = Pubkey::new_unique();
    let voter_weight_record_account = Pubkey::new_unique();
    let voter_token_account = Pubkey::new_unique();
    let governance_token_mint = Pubkey::new_unique();
    let token_program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let payer_account = Pubkey::new_unique();

    let mut test_context = ProgramTest::new(
        "registrar",
        program_id,
        processor!(process_instruction),
    );

    test_context
        .add_account_with_data(
            registrar_config_account,
            &RegistrarConfig {
                accepted_tokens: vec![governance_token_mint],
                weights: vec![10],
            },
            RegistrarConfig::LEN,
            &program_id,
        )
        .await
        .unwrap();

    test_context
        .add_account_with_data(
            voter_weight_record_account,
            &VoterWeightRecord {
                voter: payer_account,
                weight: 0,
                last_deposit_or_withdrawal_slot: 0,
            },
            VoterWeightRecord::LEN,
            &program_id,
        )
        .await
        .unwrap();

    let (mut banks_client, payer, recent_blockhash) = test_context
        .start_with_new_bank()
        .await
        .unwrap();

    let invalid_deposit_amount = 0;

    let deposit_governance_token_instruction = deposit_governance_token(
        program_id,
        voter_weight_record_account,
        voter_token_account,
        governance_token_mint,
        registrar_config_account,
        token_program,
        authority,
        invalid_deposit_amount,
    );

    assert!(banks_client
        .process_transaction_with_preflight(deposit_governance_token_instruction, &[&payer])
        .await
        .is_err());

    let invalid_withdraw_amount = 1000;

    let withdraw_governance_token_instruction = withdraw_governance_token(
        program_id,
        voter_weight_record_account,
        voter_token_account,
        governance_token_mint,
        registrar_config_account,
        token_program,
        spl_governance_program,
        authority,
        invalid_withdraw_amount,
    );

    assert!(banks_client
        .process_transaction_with_preflight(withdraw_governance_token_instruction, &[&payer])
        .await
        .is_err());
}

#[tokio::test]
async fn test_insufficient_balance_for_withdraw() {
    let program_id = Pubkey::new_unique();
    let registrar_config_account = Pubkey::new_unique();
    let voter_weight_record_account = Pubkey::new_unique();
    let voter_token_account = Pubkey::new_unique();
    let governance_token_mint = Pubkey::new_unique();
    let token_program = Pubkey::new_unique();
    let spl_governance_program = Pubkey::new_unique();
    let authority = Pubkey::new_unique();
    let payer_account = Pubkey::new_unique();

    let mut test_context = ProgramTest::new(
        "registrar",
        program_id,
        processor!(process_instruction),
    );

    test_context
        .add_account_with_data(
            registrar_config_account,
            &RegistrarConfig {
                accepted_tokens: vec![governance_token_mint],
                weights: vec![10],
            },
            RegistrarConfig::LEN,
            &program_id,
        )
        .await
        .unwrap();

    test_context
        .add_account_with_data(
            voter_weight_record_account,
            &VoterWeightRecord {
                voter: payer_account,
                weight: 100,
                last_deposit_or_withdrawal_slot: 0,
            },
            VoterWeightRecord::LEN,
            &program_id,
        )
        .await
        .unwrap();

    let (mut banks_client, payer, recent_blockhash) = test_context
        .start_with_new_bank()
        .await
        .unwrap();

    let withdraw_amount = 1000;

    let withdraw_governance_token_instruction = withdraw_governance_token(
        program_id,
        voter_weight_record_account,
        voter_token_account,
        governance_token_mint,
        registrar_config_account,
        token_program,
        spl_governance_program,
        authority,
        withdraw_amount,
    );

    assert!(banks_client
        .process_transaction_with_preflight(withdraw_governance_token_instruction, &[&payer])
        .await
        .is_err());
}
