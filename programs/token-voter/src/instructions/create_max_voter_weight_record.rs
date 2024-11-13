use {
    crate::state::*, anchor_lang::prelude::*, anchor_spl::token_interface::Mint,
    spl_governance::state::realm,
};

/// Creates MaxVoterWeightRecord used by spl-governance
/// This instruction should only be executed once per realm/governing_token_mint to create the account
#[derive(Accounts)]
pub struct CreateMaxVoterWeightRecord<'info> {
    // The Registrar the MaxVoterWeightRecord account belongs to
    pub registrar: Account<'info, Registrar>,

    #[account(
        init,
        seeds = [ b"max-voter-weight-record".as_ref(),
                registrar.realm.key().as_ref(),
                registrar.governing_token_mint.key().as_ref()],
        bump,
        payer = payer,
        space = MaxVoterWeightRecord::get_space()
    )]
    pub max_voter_weight_record: Account<'info, MaxVoterWeightRecord>,

    /// The program id of the spl-governance program the realm belongs to
    /// CHECK: Can be any instance of spl-governance and it's not known at the compilation time
    #[account(executable)]
    pub governance_program_id: UncheckedAccount<'info>,

    #[account(owner = governance_program_id.key())]
    /// CHECK: Owned by spl-governance instance specified in governance_program_id
    pub realm: UncheckedAccount<'info>,

    /// Either the realm community mint or the council mint.
    pub realm_governing_token_mint: InterfaceAccount<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_max_voter_weight_record(ctx: Context<CreateMaxVoterWeightRecord>) -> Result<()> {
    // Deserialize the Realm to validate it
    let _realm = realm::get_realm_data_for_governing_token_mint(
        &ctx.accounts.governance_program_id.key(),
        &ctx.accounts.realm,
        &ctx.accounts.realm_governing_token_mint.key(),
    )?;

    let max_voter_weight_record = &mut ctx.accounts.max_voter_weight_record;
    let registrar = &ctx.accounts.registrar;
    max_voter_weight_record.account_discriminator =
        spl_governance_addin_api::max_voter_weight::MaxVoterWeightRecord::ACCOUNT_DISCRIMINATOR;
    max_voter_weight_record.realm = registrar.realm;
    max_voter_weight_record.governing_token_mint = registrar.governing_token_mint;

    // Set expiry to expired
    max_voter_weight_record.max_voter_weight_expiry = Some(0);

    Ok(())
}
