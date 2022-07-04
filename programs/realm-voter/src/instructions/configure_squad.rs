use anchor_lang::{
    account,
    prelude::{Context, Signer},
    Accounts,
};

use anchor_lang::prelude::*;
use spl_governance::state::realm;

use crate::error::SquadsVoterError;
use crate::state::{GovernanceProgramConfig, Registrar};

/// Creates or updates Squad configuration which defines what Squads can be used for governances
/// and what weight they have
#[derive(Accounts)]
pub struct ConfigureSquad<'info> {
    /// Registrar for which we configure this Squad
    #[account(mut)]
    pub registrar: Account<'info, Registrar>,

    #[account(
       address = registrar.realm @ SquadsVoterError::InvalidRealmForRegistrar,
       owner = registrar.governance_program_id
    )]
    /// CHECK: Owned by spl-governance instance specified in registrar.governance_program_id
    pub realm: UncheckedAccount<'info>,

    /// Authority of the Realm must sign and match Realm.authority
    pub realm_authority: Signer<'info>,

    // Squad which is going to be used for governance
    /// CHECK: Owned by squads-protocol
    pub squad: UncheckedAccount<'info>,
}

pub fn configure_squad(ctx: Context<ConfigureSquad>) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;

    let realm = realm::get_realm_data_for_governing_token_mint(
        &registrar.governance_program_id,
        &ctx.accounts.realm,
        &registrar.governing_token_mint,
    )?;

    require!(
        realm.authority.unwrap() == ctx.accounts.realm_authority.key(),
        SquadsVoterError::InvalidRealmAuthority
    );

    let squad = &ctx.accounts.squad;

    // TODO: Assert Squad owned by squads-protocol

    let squad_config = GovernanceProgramConfig {
        program_id: squad.key(),
        reserved: [0; 8],
    };

    let squad_idx = registrar
        .governance_program_configs
        .iter()
        .position(|cc| cc.program_id == squad.key());

    if let Some(squad_idx) = squad_idx {
        registrar.governance_program_configs[squad_idx] = squad_config;
    } else {
        // Note: In the current runtime version push() would throw an error if we exceed
        // max_squads specified when the Registrar was created
        registrar.governance_program_configs.push(squad_config);
    }

    // TODO: if weight == 0 then remove the Squad from config
    // If weight is set to 0 then the Squad won't be removed but it won't have any governance power

    Ok(())
}
