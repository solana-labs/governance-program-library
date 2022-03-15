use crate::{
    error::NftVoterError, id, state::CollectionConfig,
    tools::token_metadata::get_token_metadata_for_mint,
};
use anchor_lang::prelude::*;
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
    /// TODO: should be expanded to list of collections
    pub collection_configs: Vec<CollectionConfig>,

    /// Reserved for future upgrades
    pub reserved: [u8; 128],
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

/// Resolves vote weight and voting mint for the given NFT
pub fn resolve_nft_vote_weight_and_mint(
    registrar: &Registrar,
    governing_token_owner: &Pubkey,
    nft_info: &AccountInfo,
    nft_metadata_info: &AccountInfo,
    unique_nft_mints: &mut Vec<Pubkey>,
) -> Result<(u16, Pubkey)> {
    let nft_owner = get_spl_token_owner(nft_info)?;

    // voter_weight_record.governing_token_owner must be the owner of the NFT
    require!(
        nft_owner == *governing_token_owner,
        NftVoterError::VoterDoesNotOwnNft
    );

    // Ensure the same NFT was not provided more than once
    let nft_mint = get_spl_token_mint(nft_info)?;
    if unique_nft_mints.contains(&nft_mint) {
        return Err(NftVoterError::DuplicatedNftDetected.into());
    }

    unique_nft_mints.push(nft_mint);

    let nft_metadata = get_token_metadata_for_mint(nft_metadata_info, &nft_mint)?;

    // The NFT must have a collection and the collection must be verified
    let collection = nft_metadata
        .collection
        .ok_or(NftVoterError::MissingMetadataCollection)?;

    require!(collection.verified, NftVoterError::CollectionMustBeVerified);

    let collection_config = registrar.get_collection_config(collection.key)?;

    Ok((collection_config.weight, nft_mint))
}
