use gpl_nft_voter::state::get_registrar_address;

use solana_program::instruction::Instruction;
use solana_program_test::*;
use solana_sdk::{signer::Signer, transaction::Transaction, transport::TransportError};

#[tokio::test]
async fn test_create_registrar() -> Result<(), TransportError> {
    let mut program_test = ProgramTest::default();
    program_test.add_program("gpl_nft_voter", gpl_nft_voter::id(), None);

    let mut context = program_test.start_with_context().await;

    let registrar_key = get_registrar_address();

    let data = anchor_lang::InstructionData::data(&gpl_nft_voter::instruction::CreateRegistrar {});

    let accounts = anchor_lang::ToAccountMetas::to_account_metas(
        &gpl_nft_voter::accounts::CreateRegistrar {
            registrar: registrar_key,
            payer: context.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        },
        None,
    );

    let create_registrar_ix = Instruction {
        program_id: gpl_nft_voter::id(),
        accounts,
        data,
    };
    let mut transaction =
        Transaction::new_with_payer(&[create_registrar_ix], Some(&context.payer.pubkey()));

    let signers = vec![&context.payer];

    transaction.sign(&signers, context.last_blockhash);

    context
        .banks_client
        .process_transaction_with_commitment(
            transaction,
            solana_sdk::commitment_config::CommitmentLevel::Processed,
        )
        .await
        .unwrap();

    Ok(())
}
