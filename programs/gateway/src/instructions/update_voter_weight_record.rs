use crate::error::GatewayError;
use crate::state::*;
use anchor_lang::prelude::*;
use solana_gateway::Gateway;
use spl_governance::state::token_owner_record::get_token_owner_record_data_for_realm_and_governing_mint;
use spl_governance_tools::account::get_account_data;
use std::cmp::max;

/// Updates VoterWeightRecord to evaluate governance power for non voting use cases: CreateProposal, CreateGovernance etc...
/// This instruction updates VoterWeightRecord which is valid for the current Slot and the given target action only
/// and hence the instruction has to be executed inside the same transaction as the corresponding spl-gov instruction
#[derive(Accounts)]
#[instruction()]
pub struct UpdateVoterWeightRecord<'info> {
    /// The Gateway Registrar
    pub registrar: Account<'info, Registrar>,

    /// An account that is either of type TokenOwnerRecordV2 or VoterWeightRecord
    /// depending on whether the registrar includes a predecessor or not
    /// CHECK: Checked in the code depending on the registrar
    #[account()]
    pub input_voter_weight: UncheckedAccount<'info>,

    /// A gateway token from the gatekeeper network in the registrar.
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

/// Sets the voter weight record value to the default voter weight, if the voter has a valid
/// Civic Pass, or throws an error if not.
pub fn update_voter_weight_record(ctx: Context<UpdateVoterWeightRecord>) -> Result<()> {
    // Gateway: Check if the voter has a valid gateway token and fail if not
    Gateway::verify_gateway_token_account_info(
        &ctx.accounts.gateway_token.to_account_info(),
        &ctx.accounts.voter_weight_record.governing_token_owner,
        &ctx.accounts.registrar.gatekeeper_network,
        None,
    )
    .map_err(|_| error!(GatewayError::InvalidGatewayToken))?;

    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    let input_voter_weight_account = ctx.accounts.input_voter_weight.to_account_info();

    let clone_record = voter_weight_record.clone();
    let input_voter_weight_record = resolve_input_voter_weight(
        &input_voter_weight_account,
        &clone_record,
        &ctx.accounts.registrar,
    )?;

    msg!(
        "input_voter_weight_record.voter_weight: {}",
        input_voter_weight_record.get_voter_weight()
    );
    voter_weight_record.voter_weight = input_voter_weight_record.get_voter_weight();
    voter_weight_record.weight_action = input_voter_weight_record.get_weight_action();
    voter_weight_record.weight_action_target = input_voter_weight_record.get_weight_action_target();

    // If the input voter weight record has an expiry, use the max between that and the current slot
    // Otherwise use the current slot
    let current_slot = Clock::get()?.slot;
    voter_weight_record.voter_weight_expiry =
        input_voter_weight_record.get_voter_weight_expiry().map_or(
            Some(current_slot), // no previous expiry, use current slot
            |previous_expiry| Some(max(previous_expiry, current_slot)),
        );

    Ok(())
}

/// Attempt to parse the input account as a VoterWeightRecord or a TokenOwnerRecordV2
fn resolve_input_voter_weight<'a>(
    input_account: &'a AccountInfo,
    voter_weight_record_to_update: &'a VoterWeightRecord,
    registrar: &'a Registrar,
) -> Result<GenericVoterWeightEnum> {
    let predecessor_generic_voter_weight_record =
        get_generic_voter_weight_record_data(input_account, registrar)?;

    // ensure that the correct governance token is used
    require_eq!(
        voter_weight_record_to_update.governing_token_mint,
        predecessor_generic_voter_weight_record.get_governing_token_mint(),
        GatewayError::InvalidPredecessorVoterWeightRecordGovTokenMint
    );

    // Ensure that the correct governance token is used
    require_eq!(
        voter_weight_record_to_update.governing_token_owner,
        predecessor_generic_voter_weight_record.get_governing_token_owner(),
        GatewayError::InvalidPredecessorVoterWeightRecordGovTokenOwner
    );

    // Ensure that the realm matches the current realm
    require_eq!(
        registrar.realm,
        predecessor_generic_voter_weight_record.get_realm(),
        GatewayError::InvalidPredecessorVoterWeightRecordRealm
    );

    Ok(predecessor_generic_voter_weight_record)
}

fn get_generic_voter_weight_record_data<'a>(
    input_account: &'a AccountInfo,
    registrar: &'a Registrar,
) -> Result<GenericVoterWeightEnum> {
    match registrar.previous_voter_weight_plugin_program_id {
        None => {
            // If there is no predecessor plugin registrar, then the input account must be a TokenOwnerRecordV2
            let record = get_token_owner_record_data_for_realm_and_governing_mint(
                &registrar.governance_program_id,
                input_account,
                &registrar.realm,
                &registrar.governing_token_mint,
            )
            .map_err(|_| error!(GatewayError::InvalidPredecessorTokenOwnerRecord))?;

            Ok(GenericVoterWeightEnum::TokenOwnerRecord(record))
        }
        Some(predecessor) => {
            // If there is a predecessor plugin registrar, then the input account must be a VoterWeightRecord
            let record: spl_governance_addin_api::voter_weight::VoterWeightRecord =
                get_account_data(&predecessor, input_account)
                    .map_err(|_| error!(GatewayError::InvalidPredecessorVoterWeightRecord))?;

            Ok(GenericVoterWeightEnum::VoterWeightRecord(record))
        }
    }
}
