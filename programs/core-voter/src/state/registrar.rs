use crate::{
    error::NftVoterError,
    id,
    state::{CollectionConfig, VoterWeightRecord},
    tools::{
        anchor::DISCRIMINATOR_SIZE, spl_token::get_spl_token_amount,
        token_metadata::get_token_metadata_for_mint,
    },
};
use anchor_lang::prelude::*;
use solana_program::pubkey::PUBKEY_BYTES;
use spl_governance::state::token_owner_record;
use spl_governance::tools::spl_token::{get_spl_token_mint, get_spl_token_owner};

/// Registrar which stores NFT voting configuration for the given Realm
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

    /// MPL Collection used for voting
    pub collection_configs: Vec<CollectionConfig>,

    /// Reserved for future upgrades
    pub reserved: [u8; 128],
}

impl Registrar {
    pub fn get_space(max_collections: u8) -> usize {
        DISCRIMINATOR_SIZE
            + PUBKEY_BYTES * 3
            + 4
            + max_collections as usize * (PUBKEY_BYTES + 4 + 8 + 8)
            + 128
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

impl Registrar {
    pub fn get_collection_config(&self, collection: Pubkey) -> Result<&CollectionConfig> {
        return self
            .collection_configs
            .iter()
            .find(|cc| cc.collection == collection)
            .ok_or_else(|| NftVoterError::CollectionNotFound.into());
    }
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
        NftVoterError::InvalidTokenOwnerForVoterWeightRecord
    );

    Ok(voter_token_owner_record.governing_token_owner)
}

/// Resolves vote weight and voting mint for the given NFT
pub fn resolve_nft_vote_weight_and_mint(
    registrar: &Registrar,
    governing_token_owner: &Pubkey,
    nft_info: &AccountInfo,
    nft_metadata_info: &AccountInfo,
    unique_nft_mints: &mut Vec<Pubkey>,
) -> Result<(u64, Pubkey)> {
    let nft_owner = get_spl_token_owner(nft_info)?;

    // voter_weight_record.governing_token_owner must be the owner of the NFT
    require!(
        nft_owner == *governing_token_owner,
        NftVoterError::VoterDoesNotOwnNft
    );

    let nft_mint = get_spl_token_mint(nft_info)?;

    // Ensure the same NFT was not provided more than once
    if unique_nft_mints.contains(&nft_mint) {
        return Err(NftVoterError::DuplicatedNftDetected.into());
    }
    unique_nft_mints.push(nft_mint);

    // Ensure the token amount is exactly 1
    let nft_amount = get_spl_token_amount(nft_info)?;

    require!(nft_amount == 1, NftVoterError::InvalidNftAmount);

    let nft_metadata = get_token_metadata_for_mint(nft_metadata_info, &nft_mint)?;

    // The NFT must have a collection and the collection must be verified
    let collection = nft_metadata
        .collection
        .ok_or(NftVoterError::MissingMetadataCollection)?;

    require!(collection.verified, NftVoterError::CollectionMustBeVerified);

    let collection_config = registrar.get_collection_config(collection.key)?;

    Ok((collection_config.weight, nft_mint))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_get_space() {
        // Arrange
        let expected_space = Registrar::get_space(3);

        let registrar = Registrar {
            governance_program_id: Pubkey::default(),
            realm: Pubkey::default(),
            governing_token_mint: Pubkey::default(),
            collection_configs: vec![
                CollectionConfig::default(),
                CollectionConfig::default(),
                CollectionConfig::default(),
            ],
            reserved: [0; 128],
        };

        // Act
        let actual_space = DISCRIMINATOR_SIZE + registrar.try_to_vec().unwrap().len();

        // Assert
        assert_eq!(expected_space, actual_space);
    }
}
