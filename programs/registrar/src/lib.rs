pub mod error;
pub mod initialize_registrar;
pub mod deposit_governance_token;
pub mod withdraw_governance_token;
pub mod registrar_config;
pub mod voter_weight_record;
pub mod max_voter_weight_record;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = RegistrarInstruction::try_from_slice(instruction_data)?;

    match instruction {
        RegistrarInstruction::InitializeRegistrar {
            accepted_tokens,
            weights,
        } => {
            let accounts = initialize_registrar::InitializeRegistrar::try_accounts(
                program_id,
                accounts,
                &accepted_tokens,
                &weights,
            )?;
            initialize_registrar::initialize_registrar(accounts, accepted_tokens, weights)
        }
        RegistrarInstruction::DepositGovernanceToken { amount } => {
            let accounts = deposit_governance_token::DepositGovernanceToken::try_accounts(
                program_id,
                accounts,
                &amount,
            )?;
            deposit_governance_token::deposit_governance_token(accounts, amount)
        }
        RegistrarInstruction::WithdrawGovernanceToken { amount } => {
            let accounts = withdraw_governance_token::WithdrawGovernanceToken::try_accounts(
                program_id,
                accounts,
                &amount,
            )?;
            withdraw_governance_token::withdraw_governance_token(accounts, amount)
        }
    }
}
