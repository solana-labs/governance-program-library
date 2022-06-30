use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use spl_governance::state::realm;

use crate::state::max_voter_weight_record::MaxVoterWeightRecord;

/// Creates MaxVoterWeightRecord used by spl-gov
/// This instruction should only be executed once per realm/governing_token_mint to create the account
#[derive(Accounts)]
pub struct CreateMaxVoterWeightRecord<'info> {
    #[account(
        init,
        seeds = [ b"max-voter-weight-record".as_ref(),
                realm.key().as_ref(),
                realm_governing_token_mint.key().as_ref()],
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
    pub realm_governing_token_mint: Account<'info, Mint>,

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

    max_voter_weight_record.realm = ctx.accounts.realm.key();
    max_voter_weight_record.governing_token_mint = ctx.accounts.realm_governing_token_mint.key();

    // Set expiry to expired
    max_voter_weight_record.max_voter_weight_expiry = Some(0);

    Ok(())
}
