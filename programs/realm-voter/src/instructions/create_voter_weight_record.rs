use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use spl_governance::state::realm;

/// Creates VoterWeightRecord used by spl-gov
/// This instruction should only be executed once per realm/governing_token_mint/governing_token_owner
/// to create the account
#[derive(Accounts)]
#[instruction(governing_token_owner: Pubkey)]
pub struct CreateVoterWeightRecord<'info> {
    #[account(
        init,
        seeds = [ b"voter-weight-record".as_ref(),
                realm.key().as_ref(),
                realm_governing_token_mint.key().as_ref(),
                governing_token_owner.as_ref()],
        bump,
        payer = payer,
        space = VoterWeightRecord::get_space()
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,

    /// The program id of the spl-governance program the realm belongs to
    /// CHECK: Can be any instance of spl-governance and it's not known at the compilation time
    #[account(executable)]
    pub governance_program_id: UncheckedAccount<'info>,

    /// CHECK: Owned by spl-governance instance specified in governance_program_id
    #[account(owner = governance_program_id.key())]
    pub realm: UncheckedAccount<'info>,

    /// Either the realm community mint or the council mint.
    pub realm_governing_token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn create_voter_weight_record(
    ctx: Context<CreateVoterWeightRecord>,
    governing_token_owner: Pubkey,
) -> Result<()> {
    // Deserialize the Realm to validate it
    let _realm = realm::get_realm_data_for_governing_token_mint(
        &ctx.accounts.governance_program_id.key(),
        &ctx.accounts.realm,
        &ctx.accounts.realm_governing_token_mint.key(),
    )?;

    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    voter_weight_record.realm = ctx.accounts.realm.key();
    voter_weight_record.governing_token_mint = ctx.accounts.realm_governing_token_mint.key();
    voter_weight_record.governing_token_owner = governing_token_owner;

    // Set expiry to expired
    voter_weight_record.voter_weight_expiry = Some(0);

    Ok(())
}
