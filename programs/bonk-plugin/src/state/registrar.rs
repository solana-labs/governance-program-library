use gpl_shared::compose::RegistrarBase;
use {
    super::VoterWeightAction,
    crate::{
        error::BonkPluginError, id, state::VoterWeightRecord,
        utils::stake_deposit_receipt::StakeDepositReceipt,
    },
    anchor_lang::prelude::*,
    solana_program::pubkey::Pubkey,
    spl_governance::state::{governance, proposal, token_owner_record},
};

/// Registrar which stores Token Voting configuration for the given Realm
#[account]
#[derive(Debug, PartialEq, InitSpace)]
pub struct Registrar {
    pub governance_program_id: Pubkey,
    pub realm: Pubkey,
    pub realm_authority: Pubkey,
    pub governing_token_mint: Pubkey,
    pub stake_pool: Pubkey,
    pub previous_voter_weight_plugin_program_id: Option<Pubkey>,
    pub reserved: [u8; 8],
}

impl<'a> RegistrarBase<'a> for Registrar {
    fn get_realm(&'a self) -> &'a Pubkey {
        &self.realm
    }

    fn get_governance_program_id(&'a self) -> &'a Pubkey {
        &self.governance_program_id
    }

    fn get_governing_token_mint(&'a self) -> &'a Pubkey {
        &self.governing_token_mint
    }

    fn get_previous_voter_weight_plugin_program_id(&'a self) -> &'a Option<Pubkey> {
        &self.previous_voter_weight_plugin_program_id
    }
}

/// Returns Registrar PDA seeds
pub fn get_registrar_seeds<'a>(
    realm: &'a Pubkey,
    governing_token_mint: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [b"registrar", realm.as_ref(), governing_token_mint.as_ref()]
}

/// Returns Registrar PDA address
pub fn get_registrar_address(realm: &Pubkey, governing_token_mint: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(&get_registrar_seeds(realm, governing_token_mint), &id()).0
}

// Resolves governing_token_owner from voter TokenOwnerRecord and
// 1) asserts it matches the given Registrar and VoterWeightRecord
// 2) asserts governing_token_owner or its delegate is a signer
pub fn resolve_governing_token_owner(
    registrar: &Registrar,
    voter_token_owner_record_info: &AccountInfo,
    voter_authority_info: &AccountInfo,
    voter_weight_record: &VoterWeightRecord,
) -> Result<Pubkey> {
    let voter_token_owner_record =
        token_owner_record::get_token_owner_record_data_for_realm_and_governing_mint(
            &registrar.governance_program_id,
            voter_token_owner_record_info,
            &registrar.realm,
            &registrar.governing_token_mint,
        )?;

    voter_token_owner_record.assert_token_owner_or_delegate_is_signer(voter_authority_info)?;

    // Assert voter TokenOwnerRecord and VoterWeightRecord are for the same governing_token_owner
    require_eq!(
        voter_token_owner_record.governing_token_owner,
        voter_weight_record.governing_token_owner,
        BonkPluginError::InvalidTokenOwnerForVoterWeightRecord
    );

    Ok(voter_token_owner_record.governing_token_owner)
}

/// Resolves vote weight for the given stake deposit receipt
#[allow(clippy::too_many_arguments)]
pub fn resolve_stake_deposit_weight(
    registrar: &Registrar,
    proposal_info: &Option<UncheckedAccount>,
    governance_info: &AccountInfo,
    governing_token_owner: &Pubkey,
    stake_deposit_receipt_info: &AccountInfo,
    unique_stake_deposit_receipts: &mut Vec<Pubkey>,
    action: VoterWeightAction,
    action_target: Pubkey,
) -> Result<u64> {
    require_gt!(
        u8::MAX,
        unique_stake_deposit_receipts.len() as u8,
        BonkPluginError::MaximumDepositsReached
    );

    let stake_deposit_receipt: StakeDepositReceipt =
        StakeDepositReceipt::deserialize_checked(stake_deposit_receipt_info)?;

    let stake_deposit_receipt_key = stake_deposit_receipt_info.key();

    // voter_weight_record.governing_token_owner must be the owner of the stake deposit
    require_keys_eq!(
        stake_deposit_receipt.owner,
        *governing_token_owner,
        BonkPluginError::VoterDoesNotOwnDepositReceipt
    );

    // Stake Pool of the deposit receipt must match the Stake pool in Registrar
    require_keys_eq!(
        stake_deposit_receipt.stake_pool,
        registrar.stake_pool,
        BonkPluginError::InvalidStakePool
    );

    // Ensure the same receipt was not provided more than once
    if unique_stake_deposit_receipts.contains(&stake_deposit_receipt_key) {
        return Err(BonkPluginError::DuplicatedReceiptDetected.into());
    }

    unique_stake_deposit_receipts.push(stake_deposit_receipt_key);

    let stake_deposit_end_time = stake_deposit_receipt
        .deposit_timestamp
        .checked_add(stake_deposit_receipt.lockup_duration as i64)
        .unwrap();

    let current_timestamp = Clock::get()?.unix_timestamp;

    require_gt!(
        stake_deposit_end_time,
        current_timestamp,
        BonkPluginError::ExpiredStakeDepositReceipt
    );

    if action == VoterWeightAction::CastVote {
        if let Some(proposal_info) = proposal_info {
            require_keys_eq!(
                proposal_info.key(),
                action_target,
                BonkPluginError::ActionTargetMismatch
            );

            let proposal_end_time =
                resolve_proposal_end_time(registrar, proposal_info, governance_info)?;

            require_gt!(
                stake_deposit_end_time,
                proposal_end_time,
                BonkPluginError::InvalidStakeDuration
            );
        } else {
            return Err(BonkPluginError::ProposalAccountIsRequired.into());
        }
    }

    Ok(stake_deposit_receipt.deposit_amount)
}

pub fn resolve_proposal_end_time(
    registrar: &Registrar,
    proposal_info: &AccountInfo,
    governance_info: &AccountInfo,
) -> Result<i64> {
    let governance = governance::get_governance_data_for_realm(
        &registrar.governance_program_id,
        governance_info,
        &registrar.realm,
    )?;

    let proposal = proposal::get_proposal_data_for_governance(
        &registrar.governance_program_id,
        proposal_info,
        governance_info.key,
    )?;

    let proposal_end_time = proposal
        .voting_base_time_end(&governance.config)
        .checked_add(governance.config.voting_cool_off_time as i64)
        .unwrap();

    Ok(proposal_end_time)
}
