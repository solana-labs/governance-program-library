use crate::error::SquadsVoterError;
use crate::state::max_voter_weight_record::MaxVoterWeightRecord;
use crate::state::*;
use anchor_lang::prelude::*;

/// Updates MaxVoterWeightRecord to evaluate max governance power for the configured Squads
/// This instruction updates MaxVoterWeightRecord which is valid for the current Slot only
/// The instruction must be executed inside the same transaction as the corresponding spl-gov instruction
#[derive(Accounts)]
pub struct UpdateMaxVoterWeightRecord<'info> {
    /// The Squads voting Registrar
    pub registrar: Account<'info, Registrar>,

    #[account(
        mut,
        constraint = max_voter_weight_record.realm == registrar.realm
        @ SquadsVoterError::InvalidVoterWeightRecordRealm,

        constraint = max_voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ SquadsVoterError::InvalidVoterWeightRecordMint,
    )]
    pub max_voter_weight_record: Account<'info, MaxVoterWeightRecord>,
    //
    // Remaining Accounts: Squads
}

pub fn update_max_voter_weight_record(ctx: Context<UpdateMaxVoterWeightRecord>) -> Result<()> {
    let registrar = &ctx.accounts.registrar;

    for squad_config in registrar.governance_program_configs.iter() {
        let _squad_info = ctx
            .remaining_accounts
            .iter()
            .find(|ai| ai.key() == squad_config.program_id)
            .unwrap();
    }

    let voter_weight_record = &mut ctx.accounts.max_voter_weight_record;

    // TODO: Remove hardcoded value
    voter_weight_record.max_voter_weight = 100;

    // Record is only valid as of the current slot
    voter_weight_record.max_voter_weight_expiry = Some(Clock::get()?.slot);

    Ok(())
}
