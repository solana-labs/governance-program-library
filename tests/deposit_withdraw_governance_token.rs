use solana_program_test::*;
use solana_sdk::{pubkey::Pubkey, signature::Signer};

#[tokio::test]
async fn test_deposit_governance_token() {
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

    let deposit_amount = 100;

    let deposit_governance_token_instruction = deposit_governance_token(
        program_id,
        voter_weight_record_account,
        voter_token_account,
        governance_token_mint,
        registrar_config_account,
        token_program,
        authority,
        deposit_amount,
    );

    banks_client
        .process_transaction_with_preflight(deposit_governance_token_instruction, &[&payer])
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
            weight: deposit_amount * 10,
            last_deposit_or_withdrawal_slot: banks_client.get_latest_slot().await.unwrap(),
        }
    );
}