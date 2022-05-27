use crate::error::GatewayError;
use crate::state::*;
use anchor_lang::prelude::*;
use solana_gateway::Gateway;
use solana_program::program_pack::{Pack, IsInitialized};
use spl_governance::state::token_owner_record::TokenOwnerRecordV2;
use spl_governance_tools::account::get_account_data;

/// Updates VoterWeightRecord to evaluate governance power for non voting use cases: CreateProposal, CreateGovernance etc...
/// This instruction updates VoterWeightRecord which is valid for the current Slot and the given target action only
/// and hence the instruction has to be executed inside the same transaction as the corresponding spl-gov instruction
#[derive(Accounts)]
#[instruction(voter_weight_action: VoterWeightAction, target: Option<Pubkey>)]
pub struct UpdateVoterWeightRecord<'info> {
    /// The Gateway Registrar
    pub registrar: Account<'info, Registrar>,

    /// An account that is either of type TokenOwnerRecordV2 or VoterWeightRecord
    /// depending on whether the registrar includes a predecessor or not
    /// CHECK: Checked in the code depending on the registrar
    #[account()]
    pub input_voting_weight: UncheckedAccount<'info>,

    /// Proves that the holder is permitted to take an action.
    /// CHECK: Checked in the gateway library.
    #[account()]
    pub gateway_token: UncheckedAccount<'info>,


    #[account(
        mut,
        constraint = voter_weight_record.realm == registrar.realm
        @ GatewayError::InvalidVoterWeightRecordRealm,

        constraint = voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ GatewayError::InvalidVoterWeightRecordMint,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,
}

pub fn update_voter_weight_record(
    ctx: Context<UpdateVoterWeightRecord>,
    voter_weight_action: VoterWeightAction,
    target: Option<Pubkey>
) -> Result<()> {
    // Gateway: Check if the voter has a valid gateway token and fail if not
    Gateway::verify_gateway_token_account_info(
        &ctx.accounts.gateway_token.to_account_info(),
        &ctx.accounts.voter_weight_record.governing_token_owner,
        &ctx.accounts.registrar.gatekeeper_network,
        None
    ).or(Err(error!(GatewayError::InvalidGatewayToken)))?;


    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    voter_weight_record.voter_weight = extract_input_voter_weight(&ctx.accounts.input_voting_weight.to_account_info(), &ctx.accounts.registrar);
    msg!("voter_weight_record.voter_weight: {}", voter_weight_record.voter_weight);

    // Record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);

    // Set the action to make it specific and prevent being used for voting
    voter_weight_record.weight_action = Some(voter_weight_action);
    voter_weight_record.weight_action_target = target;

    Ok(())
}


/// Attempt to parse the account as a VoterWeightRecord or a TokenOwnerRecordV2
/// depending on which one succeeds, return the voter weight.
fn extract_input_voter_weight(input_account: &AccountInfo, registrar: &Registrar) -> u64 {
    msg!("extract_input_voter_weight");
    match registrar.predecessor_plugin_registrar {
        None => {
            msg!("Extracting voter weight from TokenOwnerRecordV2");
            // If there is no predecessor plugin registrar, then the input account must be a TokenOwnerRecordV2
            let parse_result: core::result::Result<TokenOwnerRecordV2, ProgramError> = get_account_data(&registrar.governance_program_id, input_account);
            match parse_result {
                Ok(token_owner_record) => token_owner_record.governing_token_deposit_amount,
                Err(e) => {
                    msg!("Failed to parse input account as TokenOwnerRecordV2: {:?}", e);
                    DEFAULT_VOTE_WEIGHT
                }, // TODO should probably be an error
            }
        }
        Some(predecessor) => {
            msg!("Extracting voter weight from VoterWeightRecord");
            let parse_result: core::result::Result<VoterWeightRecord, ProgramError> = get_account_data(&predecessor, input_account);
            match parse_result {
                Ok(predecessor_voter_weight_record) => predecessor_voter_weight_record.voter_weight,
                Err(e) => {
                    msg!("Failed to parse input account as TokenOwnerRecordV2: {:?}", e);
                    DEFAULT_VOTE_WEIGHT // TODO should probably be an error   
                }
            }
        }
    }
}
