use anchor_lang::prelude::ERROR_CODE_OFFSET;
// use gpl_nft_voter::error::NftVoterError;
use no_transfers_signatory::error::TransactionCheckerError;
use solana_program::instruction::InstructionError;
use solana_sdk::{signature::Keypair, transaction::TransactionError, transport::TransportError};
// use spl_governance_tools::error::GovernanceToolsError;

pub fn clone_keypair(source: &Keypair) -> Keypair {
    Keypair::from_bytes(&source.to_bytes()).unwrap()
}

/// NOP (No Operation) Override function
#[allow(non_snake_case)]
pub fn NopOverride<T>(_: &mut T) {}

#[allow(dead_code)]
pub fn assert_transaction_checker_err(
    banks_client_error: TransportError,
    transaction_checker_error: TransactionCheckerError,
) {
    let tx_error = banks_client_error.unwrap();

    match tx_error {
        TransactionError::InstructionError(_, instruction_error) => match instruction_error {
            InstructionError::Custom(code) => {
                assert_eq!(code, transaction_checker_error as u32 + ERROR_CODE_OFFSET)
            }
            _ => panic!("{:?} Is not InstructionError::Custom()", instruction_error),
        },
        _ => panic!("{:?} Is not InstructionError", tx_error),
    };
}

#[allow(dead_code)]
pub fn assert_anchor_err(
    banks_client_error: TransportError,
    anchor_error: anchor_lang::error::ErrorCode,
) {
    let tx_error = banks_client_error.unwrap();

    match tx_error {
        TransactionError::InstructionError(_, instruction_error) => match instruction_error {
            InstructionError::Custom(e) => {
                assert_eq!(e, anchor_error as u32)
            }
            _ => panic!("{:?} Is not InstructionError::Custom()", instruction_error),
        },
        _ => panic!("{:?} Is not InstructionError", tx_error),
    };
}

#[allow(dead_code)]
pub fn assert_ix_err(banks_client_error: TransportError, ix_error: InstructionError) {
    let tx_error = banks_client_error.unwrap();

    match tx_error {
        TransactionError::InstructionError(_, instruction_error) => {
            assert_eq!(instruction_error, ix_error);
        }
        _ => panic!("{:?} Is not InstructionError", tx_error),
    };
}
