use crate::error::TokenHaverError;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

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
}

pub fn update_voter_weight_record<'info>(
    ctx: Context<'_, '_, 'info, 'info, UpdateVoterWeightRecord<'info>>,
) -> Result<()> {
    let registrar = &ctx.accounts.registrar;
    let voter_weight_record = &mut ctx.accounts.voter_weight_record;

    let nonzero_token_accounts: Vec<Account<TokenAccount>> = ctx
        .remaining_accounts
        .iter()
        .map(|account| Account::<TokenAccount>::try_from(account).unwrap())
        .filter(|account| account.amount > 0) // filter out zero balance accounts
        .collect();

    for account in nonzero_token_accounts.iter() {
        // Throw an error if a token account's owner doesnt match token_owner_record.governing_token_owner
        require_eq!(
            account.owner,
            voter_weight_record.governing_token_owner,
            TokenHaverError::TokenAccountWrongOwner
        );

        // Throw an error if a token account's mint is not unique amount the accounts
        require!(
            nonzero_token_accounts
                .iter()
                .filter(|a| a.mint == account.mint)
                .count()
                == 1,
            TokenHaverError::TokenAccountDuplicateMint
        );

        // Throw an error if a token account's mint isn't in registrar.mints
        require!(
            registrar.mints.contains(&account.mint),
            TokenHaverError::TokenAccountWrongMint
        );
        // Throw an error if a token account is not frozen
        require!(account.is_frozen(), TokenHaverError::TokenAccountNotLocked);
    }

    // Setup voter_weight
    voter_weight_record.voter_weight = (nonzero_token_accounts.len() as u64) * 1_000_000;

    // Record is only valid as of the current slot
    voter_weight_record.voter_weight_expiry = Some(Clock::get()?.slot);
    Ok(())
}
