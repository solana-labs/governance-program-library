use anchor_lang::{
    account,
    prelude::{Context, Signer},
    Accounts,
};

use anchor_lang::prelude::*;
use spl_governance::state::realm;

use crate::error::RealmVoterError;
use crate::state::{CollectionItemChangeType, GovernanceProgramConfig, Registrar};

/// Creates or updates configuration for spl-governance program instances to define which spl-governance instances can be used to grant governance power
#[derive(Accounts)]
#[instruction(change_type: CollectionItemChangeType)]
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

    // spl-governance instance which will be inserted, updated or removed to configured instances allowed to participate in governance
    /// CHECK: It can be any instance of spl-governance and there is no way to validate it's a correct instance
    /// The onus is entirely on the  caller side to ensure the provided instance is correct
    /// In future versions once we have the registry of spl-governance instances it could be validated against the registry
    #[account(executable)]
    pub governance_program_id: UncheckedAccount<'info>,
}

pub fn configure_governance_program(
    ctx: Context<ConfigureGovernanceProgram>,
    change_type: CollectionItemChangeType,
) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;

    let realm = realm::get_realm_data_for_governing_token_mint(
        &registrar.governance_program_id,
        &ctx.accounts.realm,
        &registrar.governing_token_mint,
    )?;

    require_eq!(
        realm.authority.unwrap(),
        ctx.accounts.realm_authority.key(),
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

    match (change_type, governance_program_config_idx) {
        // Update
        (CollectionItemChangeType::Upsert, Some(config_idx)) => {
            // Note: Update in this version is nop because we only store governance_program_id
            registrar.governance_program_configs[config_idx] = governance_program_config;
        }
        // Insert
        (CollectionItemChangeType::Upsert, None) => {
            // Note: In the current version push() would throw an error if we exceed
            // max_governance_programs specified when the Registrar was created
            registrar
                .governance_program_configs
                .push(governance_program_config);
        }
        (CollectionItemChangeType::Remove, Some(config_idx)) => {
            registrar.governance_program_configs.remove(config_idx);
        }

        (CollectionItemChangeType::Remove, None) => {
            return err!(RealmVoterError::GovernanceProgramNotConfigured)
        }
    }

    Ok(())
}
