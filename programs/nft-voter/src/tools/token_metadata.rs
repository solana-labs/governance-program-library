use anchor_lang::prelude::*;

use mpl_token_metadata::state::Metadata;

use crate::error::NftVoterError;

pub fn get_token_metadata(account_info: &AccountInfo) -> Result<Metadata> {
    if *account_info.owner != mpl_token_metadata::ID {
        return Err(NftVoterError::InvalidAccountOwner.into());
    }

    let metadata = Metadata::from_account_info(account_info)?;

    Ok(metadata)
}

pub fn get_token_metadata_for_mint(account_info: &AccountInfo, mint: Pubkey) -> Result<Metadata> {
    let token_metadata = get_token_metadata(account_info)?;

    if token_metadata.mint != mint {
        return Err(NftVoterError::TokenMetadataDoesNotMatch.into());
    }

    Ok(token_metadata)
}
