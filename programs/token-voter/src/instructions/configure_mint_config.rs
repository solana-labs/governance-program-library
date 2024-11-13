use {
    crate::{error::*, state::*},
    anchor_lang::prelude::*,
    anchor_spl::token_interface::Mint,
    spl_governance::state::realm,
};

/// Creates or updates configuration for spl-governance program instances to
/// define which spl-governance instances can be used to grant governance power
#[derive(Accounts)]
pub struct ConfigureVotingMintConfig<'info> {
    /// Registrar which we configure the provided spl-governance instance for
    #[account(mut)]
    pub registrar: Account<'info, Registrar>,

    #[account(
       owner = registrar.governance_program_id,
       constraint = realm.key() == registrar.realm @ TokenVoterError::InvalidRealmForRegistrar,
    )]
    /// CHECK: Owned by spl-governance instance specified in registrar.governance_program_id
    // Validated below
    pub realm: UncheckedAccount<'info>,

    /// Authority of the Realm must sign the transaction and must match realm.authority
    pub realm_authority: Signer<'info>,

    /// Tokens of this mint will be included in the Mint Configs
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        constraint = max_voter_weight_record.realm == registrar.realm
        @ TokenVoterError::InvalidMaxVoterWeightRecordRealm,

        constraint = max_voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ TokenVoterError::InvalidMaxVoterWeightRecordMint,
    )]
    pub max_voter_weight_record: Account<'info, MaxVoterWeightRecord>,

    /// CHECK: It can be any instance of spl-governance and there is no way to validate it's a correct instance
    /// The onus is entirely on the caller side to ensure the provided instance is correct
    /// In future versions once we have the registry of spl-governance instances it could be validated against the registry
    #[account(executable)]
    pub governance_program_id: UncheckedAccount<'info>,
}

pub fn configure_mint_config(
    ctx: Context<ConfigureVotingMintConfig>,
    digit_shift: i8,
) -> Result<()> {
    let registrar = &mut ctx.accounts.registrar;
    let mint = &ctx.accounts.mint;
    let max_voter_weight_record = &mut ctx.accounts.max_voter_weight_record;

    let realm = realm::get_realm_data_for_governing_token_mint(
        &registrar.governance_program_id,
        &ctx.accounts.realm,
        &registrar.governing_token_mint,
    )?;

    require_eq!(
        realm.authority.unwrap(),
        ctx.accounts.realm_authority.key(),
        TokenVoterError::InvalidRealmAuthority
    );

    let voting_mint_config = VotingMintConfig {
        mint: mint.key(),
        digit_shift,
        mint_supply: mint.supply,
        reserved1: [0; 55],
    };

    let mint_config_idx = registrar
        .voting_mint_configs
        .iter()
        .position(|vmc| vmc.mint == mint.key());

    if let Some(mint_config_idx) = mint_config_idx {
        registrar.voting_mint_configs[mint_config_idx] = voting_mint_config;
    } else {
        // Note: In the current runtime version push() would throw an error if we exceed
        // max_mints specified when the Registrar was created
        registrar.voting_mint_configs.push(voting_mint_config);
    }

    // Update MaxVoterWeightRecord.max_voter_weight
    // recalculate the max voter weight as mint supply has possibly changed
    max_voter_weight_record.max_voter_weight = registrar.max_vote_weight()?;

    max_voter_weight_record.max_voter_weight_expiry = None;

    Ok(())
}
