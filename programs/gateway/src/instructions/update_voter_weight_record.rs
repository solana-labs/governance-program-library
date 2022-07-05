use crate::error::GatewayError;
use crate::state::*;
use anchor_lang::prelude::*;
use solana_gateway::Gateway;

/// The default vote weight matches the default decimal places of a governance token
/// so that a single vote using this plugin matches a single vote with a governance token
/// This is a temporary solution, until SPL-Governance supports plugin-composition,
/// at which point, the deposited tokens will be passed in as an input vote weight.
pub const DEFAULT_VOTE_WEIGHT: u64 = 1000000;

/// Updates VoterWeightRecord to evaluate governance power for non voting use cases: CreateProposal, CreateGovernance etc...
/// This instruction updates VoterWeightRecord which is valid for the current Slot and the given target action only
/// and hence the instruction has to be executed inside the same transaction as the corresponding spl-gov instruction
#[derive(Accounts)]
#[instruction(voter_weight_action: VoterWeightAction, target: Option<Pubkey>)]
pub struct UpdateVoterWeightRecord<'info> {
    /// The Gateway Registrar
    pub registrar: Account<'info, Registrar>,

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
/// Note: Once SPL-Governance supports plugin-composition, this will be changed to accept an
/// input vote weight, and will set the output vote weight to the input vote weight.
/// The action and target are not used in this case.
pub fn update_voter_weight_record(
    ctx: Context<UpdateVoterWeightRecord>,
    voter_weight_action: VoterWeightAction,
    target: Option<Pubkey>,
) -> Result<()> {
    // Gateway: Check if the voter has a valid gateway token and fail if not
    Gateway::verify_gateway_token_account_info(
        &ctx.accounts.gateway_token.to_account_info(),
        &ctx.accounts.voter_weight_record.governing_token_owner,
        &ctx.accounts.registrar.gatekeeper_network,
        None,
    )
    .map_err(|_| error!(GatewayError::InvalidGatewayToken))?;

    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    voter_weight_record.voter_weight = DEFAULT_VOTE_WEIGHT;

    // Record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);

    // Set the action to make it specific and prevent being used for voting
    voter_weight_record.weight_action = Some(voter_weight_action);
    voter_weight_record.weight_action_target = target;

    Ok(())
}
