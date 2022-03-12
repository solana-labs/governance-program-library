use crate::{error::NftVoterError, id, state::CollectionConfig};
use anchor_lang::prelude::*;

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
    pub fn is_in_collection_configs(&self, collection: Pubkey) -> Result<bool> {
        match self
            .collection_configs
            .iter()
            .any(|r| r.collection == collection)
        {
            true => Ok(true),
            false => Err(NftVoterError::InvalidCollection.into()),
        }
    }

    pub fn collection_config_index(&self, collection: Pubkey) -> Result<usize> {
        self.collection_configs
            .iter()
            .position(|r| r.collection == collection)
            .ok_or_else(|| NftVoterError::InvalidCollection.into())
    }

    pub fn get_collection_config(&self, collection: Pubkey) -> Result<&CollectionConfig> {
        return self
            .collection_configs
            .iter()
            .find(|cc| cc.collection == collection)
            .ok_or_else(|| NftVoterError::CollectionNotFound.into());
    }
}
