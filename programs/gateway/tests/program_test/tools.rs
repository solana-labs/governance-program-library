use crate::{
    program_test::gateway_voter_test::VoterWeightRecordCookie,
    program_test::governance_test::TokenOwnerRecordCookie,
};
use anchor_lang::prelude::ERROR_CODE_OFFSET;
use gpl_civic_gateway::error::GatewayError;
use itertools::Either;
use solana_program::{instruction::InstructionError, pubkey::Pubkey};
use solana_program_test::BanksClientError;
use solana_sdk::{signature::Keypair, transaction::TransactionError, transport::TransportError};
use spl_governance_tools::error::GovernanceToolsError;

pub fn clone_keypair(source: &Keypair) -> Keypair {
    Keypair::from_bytes(&source.to_bytes()).unwrap()
}

/// NOP (No Operation) Override function
#[allow(non_snake_case)]
pub fn NopOverride<T>(_: &mut T) {}

#[allow(dead_code)]
pub fn assert_gateway_err(banks_client_error: BanksClientError, gateway_error: GatewayError) {
    let tx_error = banks_client_error.unwrap();

    match tx_error {
        TransactionError::InstructionError(_, instruction_error) => match instruction_error {
            InstructionError::Custom(e) => {
                assert_eq!(e, gateway_error as u32 + ERROR_CODE_OFFSET)
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

pub fn extract_voting_weight_address(
    account: &Either<&VoterWeightRecordCookie, &TokenOwnerRecordCookie>,
) -> Pubkey {
    account
        .map_left(|cookie| cookie.address)
        .map_right(|cookie| cookie.address)
        .into_inner()
}
