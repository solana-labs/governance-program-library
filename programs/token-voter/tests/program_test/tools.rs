use anchor_lang::prelude::ERROR_CODE_OFFSET;
use solana_program::instruction::InstructionError;
use solana_program_test::BanksClientError;
use solana_sdk::{signature::Keypair, transaction::TransactionError, transport::TransportError};
use spl_governance_tools::error::GovernanceToolsError;
use gpl_token_voter::error::TokenVoterError;

pub fn clone_keypair(source: &Keypair) -> Keypair {
    Keypair::from_bytes(&source.to_bytes()).unwrap()
}

/// NOP (No Operation) Override function
#[allow(non_snake_case)]
pub fn NopOverride<T>(_: &mut T) {}

#[allow(dead_code)]
pub fn assert_token_voter_err(
    banks_client_error: BanksClientError,
    token_voter_error: TokenVoterError,
) {
    let tx_error = banks_client_error.unwrap();

    match tx_error {
        TransactionError::InstructionError(_, instruction_error) => match instruction_error {
            InstructionError::Custom(e) => {
                assert_eq!(e, token_voter_error as u32 + ERROR_CODE_OFFSET)
            }
            _ => panic!("{:?} Is not InstructionError::Custom()", instruction_error),
        },
        _ => panic!("{:?} Is not InstructionError", tx_error),
    };
}

#[allow(dead_code)]
pub fn assert_gov_tools_err(
    banks_client_error: TransportError,
    gov_tools_error: GovernanceToolsError,
) {
    let tx_error = banks_client_error.unwrap();

    match tx_error {
        TransactionError::InstructionError(_, instruction_error) => match instruction_error {
            InstructionError::Custom(e) => {
                assert_eq!(e, gov_tools_error as u32)
            }
            _ => panic!("{:?} Is not InstructionError::Custom()", instruction_error),
        },
        _ => panic!("{:?} Is not InstructionError", tx_error),
    };
}

#[allow(dead_code)]
pub fn assert_anchor_err(
    banks_client_error: BanksClientError,
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
pub fn assert_ix_err(banks_client_error: BanksClientError, ix_error: InstructionError) {
    let tx_error = banks_client_error.unwrap();

    match tx_error {
        TransactionError::InstructionError(_, instruction_error) => {
            assert_eq!(instruction_error, ix_error);
        }
        _ => panic!("{:?} Is not InstructionError", tx_error),
    };
}

#[allow(dead_code)]
pub fn assert_ix_err_transport(
    banks_client_error: TransportError,
    ix_error: InstructionError
) {
    let tx_error = banks_client_error.unwrap();

    match tx_error {
        TransactionError::InstructionError(_, instruction_error) => {
            assert_eq!(instruction_error, ix_error);
        }
        _ => panic!("{:?} Is not InstructionError", tx_error),
    };
}
