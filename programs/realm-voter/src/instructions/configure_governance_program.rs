use anchor_lang::{
    account,
    prelude::{Context, Signer},
    Accounts,
};

use anchor_lang::prelude::*;
use spl_governance::state::realm;

use crate::error::RealmVoterError;
use crate::state::{GovernanceProgramConfig, Registrar};

/// Creates or updates spl-governance configuration which defines what spl-governance can be used for governance
/// and what weight they have
#[derive(Accounts)]
pub struct ConfigureGovernanceProgram<'info> {
    /// Registrar for which we configure this spl-governance
    #[account(mut)]
    pub registrar: Account<'info, Registrar>,

    #[account(
       address = registrar.realm @ RealmVoterError::InvalidRealmForRegistrar,
       owner = registrar.governance_program_id
    )]
    /// CHECK: Owned by spl-governance instance specified in registrar.governance_program_id
    pub realm: UncheckedAccount<'info>,

    /// Authority of the Realm must sign and match Realm.authority
    pub realm_authority: Signer<'info>,

    // spl-governance which is going to be used for governance
    /// CHECK: Can be any instance of spl-governance and it's not known at the compilation time
    #[account(executable)]
    pub governance_program_id: UncheckedAccount<'info>,
}

pub fn configure_governance_program(
    ctx: Context<ConfigureGovernanceProgram>,
    weight: u64,
) -> Result<()> {
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
        weight,
        reserved: [0; 8],
    };

    let governance_program_config_idx = registrar
        .governance_program_configs
        .iter()
        .position(|cc| cc.program_id == governance_program_id.key());

    if let Some(config_idx) = governance_program_config_idx {
        if weight == 0 {
            registrar.governance_program_configs.remove(config_idx);
        } else {
            registrar.governance_program_configs[config_idx] = governance_program_config;
        }
    } else {
        require_gt!(weight, 0, RealmVoterError::InvalidGovernanceProgramWeight);

        // Note: In the current version push() would throw an error if we exceed
        // max_governance_programs specified when the Registrar was created
        registrar
            .governance_program_configs
            .push(governance_program_config);
    }

    Ok(())
}
