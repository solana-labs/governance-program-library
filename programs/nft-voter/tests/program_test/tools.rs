use anchor_lang::prelude::ERROR_CODE_OFFSET;
use gpl_nft_voter::error::ErrorCode;
use solana_program::instruction::InstructionError;
use solana_program_test::BanksClientError;
use solana_sdk::{signature::Keypair, transaction::TransactionError};

pub fn clone_keypair(source: &Keypair) -> Keypair {
    Keypair::from_bytes(&source.to_bytes()).unwrap()
}

/// NOP (No Operation) Override function
#[allow(non_snake_case)]
#[allow(dead_code)]
pub fn NopOverride<T>(_: &mut T) {}

#[allow(dead_code)]
pub fn assert_err(banks_client_error: BanksClientError, error_code: ErrorCode) {
    let tx_error = banks_client_error.unwrap();

    match tx_error {
        TransactionError::InstructionError(_, instruction_error) => match instruction_error {
            InstructionError::Custom(e) => {
                assert_eq!(e, error_code as u32 + ERROR_CODE_OFFSET)
            }
            _ => panic!("Not Custom InstructionError"),
        },
        _ => panic!("Not InstructionError"),
    };
}

#[allow(dead_code)]
pub fn assert_anchor_err(
    banks_client_error: BanksClientError,
    error_code: anchor_lang::error::ErrorCode,
) {
    let tx_error = banks_client_error.unwrap();

    match tx_error {
        TransactionError::InstructionError(_, instruction_error) => match instruction_error {
            InstructionError::Custom(e) => {
                assert_eq!(e, error_code as u32)
            }
            _ => panic!("Not Custom InstructionError"),
        },
        _ => panic!("Not InstructionError"),
    };
}
