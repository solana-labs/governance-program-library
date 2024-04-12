use crate::error::TokenHaverError;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use spl_governance::state::token_owner_record;

/// Updates VoterWeightRecord based on Realm DAO membership
/// The membership is evaluated via a valid TokenOwnerRecord which must belong to one of the configured spl-governance instances
///
/// This instruction sets VoterWeightRecord.voter_weight which is valid for the current slot only
/// and must be executed inside the same transaction as the corresponding spl-gov instruction
#[derive(Accounts)]
pub struct UpdateVoterWeightRecord<'info> {
    /// The RealmVoter voting Registrar
    pub registrar: Account<'info, Registrar>,

    #[account(
        mut,
        constraint = voter_weight_record.realm == registrar.realm
        @ TokenHaverError::InvalidVoterWeightRecordRealm,

        constraint = voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ TokenHaverError::InvalidVoterWeightRecordMint,
    )]
    pub voter_weight_record: Account<'info, VoterWeightRecord>,

    /// TokenOwnerRecord for any of the configured spl-governance instances
    /// CHECK: Owned by any of the spl-governance instances specified in registrar.governance_program_configs
    pub token_owner_record: UncheckedAccount<'info>,
}

pub fn update_voter_weight_record(ctx: Context<UpdateVoterWeightRecord>) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    let governance_program_id = ctx.accounts.token_owner_record.owner;

    let token_owner_record = token_owner_record::get_token_owner_record_data(
        governance_program_id,
        &ctx.accounts.token_owner_record,
    )?;

    // Ensure VoterWeightRecord and TokenOwnerRecord are for the same governing_token_owner
    require_eq!(
        token_owner_record.governing_token_owner,
        voter_weight_record.governing_token_owner,
        TokenHaverError::GoverningTokenOwnerMustMatch
    );

    let nonzero_token_accounts: Vec<Account<TokenAccount>> = ctx
        .remaining_accounts
        .iter()
        .filter(|account| !account.data_is_empty()) // filter out empty accounts, so we don't need to check them in UI
        .map(|account| Account::<TokenAccount>::try_from(account).unwrap())
        .filter(|account| account.amount > 0) // filter out zero balance accounts
        .collect();

    for account in nonzero_token_accounts.iter() {
        // Throw an error if a token account's owner doesnt match token_owner_record.governing_token_owner
        require_eq!(
            account.owner,
            token_owner_record.governing_token_owner,
            TokenHaverError::TokenAccountWrongOwner
        );
        // Throw an error if a token account's mint isn't in registrar.mints
        require!(
            registrar.mints.contains(&account.mint),
            TokenHaverError::TokenAccountWrongMint
        );
        // Throw an error if a token account is not frozen
        require!(account.is_frozen(), TokenHaverError::TokenAccountWrongMint);
    }

    // Setup voter_weight
    voter_weight_record.voter_weight = nonzero_token_accounts.len() as u64;

    // Record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);

    // Set action and target to None to indicate the weight is valid for any action and target
    voter_weight_record.weight_action = None;
    voter_weight_record.weight_action_target = None;

    Ok(())
}
