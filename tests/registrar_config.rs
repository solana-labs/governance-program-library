use solana_program_test::*;
use solana_sdk::{pubkey::Pubkey, signature::Signer};

#[tokio::test]
async fn test_initialize_registrar() {
    let program_id = Pubkey::new_unique();
    let registrar_config_account = Pubkey::new_unique();
    let payer_account = Pubkey::new_unique();

    let mut test_context = ProgramTest::new(
        "registrar",
        program_id,
        processor!(process_instruction),
    );

    test_context
        .add_account_with_data(
            registrar_config_account,
            &[],
            RegistrarConfig::LEN,
            &program_id,
        )
        .await
        .unwrap();

    let (mut banks_client, payer, recent_blockhash) = test_context
        .start_with_new_bank()
        .await
        .unwrap();

    let accepted_tokens = vec![Pubkey::new_unique(), Pubkey::new_unique()];
    let weights = vec![10, 20];

    let initialize_registrar_instruction = initialize_registrar(
        program_id,
        registrar_config_account,
        payer_account,
        accepted_tokens.clone(),
        weights.clone(),
    );

    banks_client
        .process_transaction_with_preflight(initialize_registrar_instruction, &[&payer])
        .await
        .unwrap();

    let registrar_config_account_data = banks_client
        .get_account_data_with_borsh(registrar_config_account)
        .await
        .unwrap();

    assert_eq!(
        registrar_config_account_data,
        RegistrarConfig {
            accepted_tokens,
            weights
        }
    );
}
