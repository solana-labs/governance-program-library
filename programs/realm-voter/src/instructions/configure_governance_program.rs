use anchor_lang::{
    account,
    prelude::{Context, Signer},
    Accounts,
};

use anchor_lang::prelude::*;
use spl_governance::state::realm;

use crate::error::RealmVoterError;
use crate::state::{GovernanceProgramConfig, Registrar};

/// Creates or updates spl-governance configuration which defines which spl-governance instances can be used for governance
#[derive(Accounts)]
pub struct ConfigureGovernanceProgram<'info> {
    /// Registrar which we configure the provided spl-governance instance for
    #[account(mut)]
    pub registrar: Account<'info, Registrar>,

    #[account(
       address = registrar.realm @ RealmVoterError::InvalidRealmForRegistrar,
       owner = registrar.governance_program_id
    )]
    /// CHECK: Owned by spl-governance instance specified in registrar.governance_program_id
    pub realm: UncheckedAccount<'info>,

    /// Authority of the Realm must sign the transaction and must match realm.authority
    pub realm_authority: Signer<'info>,

    // spl-governance instance which will be added to configured instances allowed to participate in governance
    /// CHECK: Can be any instance of spl-governance and it's not known at the compilation time
    #[account(executable)]
    pub governance_program_id: UncheckedAccount<'info>,
}

pub fn configure_governance_program(ctx: Context<ConfigureGovernanceProgram>) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;

    let realm = realm::get_realm_data_for_governing_token_mint(
        &registrar.governance_program_id,
        &ctx.accounts.realm,
        &registrar.governing_token_mint,
    )?;

    require!(
        realm.authority.unwrap() == ctx.accounts.realm_authority.key(),
        RealmVoterError::InvalidRealmAuthority
    );

    let governance_program_id = &ctx.accounts.governance_program_id;

    let governance_program_config = GovernanceProgramConfig {
        program_id: governance_program_id.key(),
        reserved: [0; 8],
    };

    let governance_program_config_idx = registrar
        .governance_program_configs
        .iter()
        .position(|cc| cc.program_id == governance_program_id.key());

    if let Some(config_idx) = governance_program_config_idx {
        registrar.governance_program_configs[config_idx] = governance_program_config;
    } else {
        // Note: In the current version push() would throw an error if we exceed
        // max_governance_programs specified when the Registrar was created
        registrar
            .governance_program_configs
            .push(governance_program_config);
    }

    Ok(())
}
