use {
    crate::{
        error::TokenVoterError, id, state::VotingMintConfig,
        tools::spl_token::get_spl_token_mint_supply, vote_weight_record, max_voter_weight_record
    },
    anchor_lang::{prelude::*, Discriminator},
    solana_program::pubkey::PUBKEY_BYTES,
    spl_governance::state::token_owner_record,
};

// Generate a VoteWeightRecord Anchor wrapper, owned by the current program.
// VoteWeightRecords are unique in that they are defined by the SPL governance
// program, but they are actually owned by this program.
vote_weight_record!(crate::ID);

// Generate a MaxVoterWeightRecord Anchor wrapper, owned by the current program.
// MaxVoterWeightRecord are unique in that they are defined by the SPL governance
// program, but they are actually owned by this program.
max_voter_weight_record!(crate::ID);

/// Registrar which stores Token Voting configuration for the given Realm
#[account]
#[derive(Debug, PartialEq)]
pub struct Registrar {
    /// spl-governance program the Realm belongs to
    pub governance_program_id: Pubkey,

    /// Realm of the Registrar
    pub realm: Pubkey,

    /// Governing token mint the Registrar is for
    /// It can either be the Community or the Council mint of the Realm
    /// When the plugin is used the mint is only used as identity of the governing power (voting population)
    /// and the actual token of the mint is not used
    pub governing_token_mint: Pubkey,

    /// Storage for voting mints and their configuration.
    /// The length should be adjusted for one's use case.
    pub voting_mint_configs: Vec<VotingMintConfig>,

    /// Max mints that voters can create.
    pub max_mints: u8,

    /// Reserved for future upgrades
    pub reserved: [u8; 127],
}

impl Registrar {
    pub fn get_space(max_mints: u8) -> usize {
        Registrar::discriminator().len()
            + PUBKEY_BYTES * 3
            + 4
            + max_mints as usize * (PUBKEY_BYTES + 1 + 63)
            + 128
    }
    
    pub fn voting_mint_config_index(&self, mint: Pubkey) -> Result<usize> {
        self.voting_mint_configs
            .iter()
            .position(|r| r.mint == mint)
            .ok_or_else(|| error!(TokenVoterError::MintNotFound))
    }

    pub fn max_vote_weight(&self, mint_accounts: &[AccountInfo]) -> Result<u64> {
        self.voting_mint_configs
            .iter()
            .try_fold(0u64, |mut sum, mint_config| -> Result<u64> {
                if !mint_config.in_use() {
                    return Ok(sum);
                }
                let mint_account = mint_accounts
                    .iter()
                    .find(|a| a.key() == mint_config.mint)
                    .ok_or_else(|| error!(TokenVoterError::MintNotFound))?;
                let mint_supply = get_spl_token_mint_supply(mint_account)?;
                sum = sum
                    .checked_add(mint_config.digit_shift_native(mint_supply)?)
                    .ok_or_else(|| error!(TokenVoterError::VoterWeightOverflow))?;
                Ok(sum)
            })
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
        TokenVoterError::InvalidTokenOwnerForVoterWeightRecord
    );

    Ok(voter_token_owner_record.governing_token_owner)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_get_space() {
        // Arrange
        let expected_space = Registrar::get_space(3);
        let mint_config = VotingMintConfig {
            mint: Pubkey::default(),
            digit_shift: 0,
            reserved1: [0; 63],
        };

        let registrar = Registrar {
            governance_program_id: Pubkey::default(),
            voting_mint_configs: vec![mint_config, mint_config, mint_config],
            realm: Pubkey::default(),
            governing_token_mint: Pubkey::default(),
            max_mints: 0,
            reserved: [0; 127],
        };

        // Act
        let actual_space = Registrar::discriminator().len() + registrar.try_to_vec().unwrap().len();

        // Assert
        assert_eq!(expected_space, actual_space);
    }
}
