use anchor_lang::prelude::*;

use crate::{id, tools::anchor::DISCRIMINATOR_SIZE};

/// Receipt for depositing tokens against a voting NFT to enable
/// token owners to withdraw tokens from the NFT's holding account
#[account]
#[derive(Debug, Copy, PartialEq)]
pub struct NftVoterTokenOwnerRecord {
    /// The Realm the TokenOwnerRecord belongs to
    pub realm: Pubkey,

    /// Governing Token Mint the TokenOwnerRecord holds deposit for
    pub governing_token_mint: Pubkey,

    /// The mint of the NFT being backed
    pub nft_mint: Pubkey,

    /// The owner (either single or multisig) of the deposited governing SPL Tokens
    /// This is who can authorize a withdrawal of the tokens
    pub governing_token_owner: Pubkey,

    /// The amount of governing tokens deposited into the NFT's holding account
    /// This amount affects the voter weight used when voting on proposals
    pub governing_token_deposit_amount: u64,
}

impl NftVoterTokenOwnerRecord {
    pub fn get_space() -> usize {
        DISCRIMINATOR_SIZE +
        32 + // realm
        32 + // governing token mint
        32 + // nft mint
        32 + // governing token owner
        8 // deposit amount
    }
}

impl Default for NftVoterTokenOwnerRecord {
    fn default() -> Self {
        Self {
            realm: Default::default(),
            governing_token_mint: Default::default(),
            nft_mint: Default::default(),
            governing_token_owner: Default::default(),
            governing_token_deposit_amount: 0,
        }
    }
}

/// Returns NftVoterTokenOwnerRecord PDA seeds
pub fn get_nft_voter_token_owner_record_seeds<'a>(
    realm: &'a Pubkey,
    governing_token_mint: &'a Pubkey,
    nft_mint: &'a Pubkey,
    governing_token_owner: &'a Pubkey,
) -> [&'a [u8]; 5] {
    [
        b"nft-voter-token-owner-record",
        realm.as_ref(),
        governing_token_mint.as_ref(),
        nft_mint.as_ref(),
        governing_token_owner.as_ref(),
    ]
}

/// Returns NftVoterTokenOwnerRecord PDA address
pub fn get_nft_voter_token_owner_record_address(
    realm: &Pubkey,
    governing_token_mint: &Pubkey,
    nft_mint: &Pubkey,
    governing_token_owner: &Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_nft_voter_token_owner_record_seeds(
            realm,
            governing_token_mint,
            nft_mint,
            governing_token_owner,
        ),
        &id(),
    )
    .0
}

#[cfg(test)]
mod test {

    use crate::tools::anchor::DISCRIMINATOR_SIZE;

    use super::*;

    #[test]
    fn test_get_space() {
        // Arrange
        let expected_space = NftVoterTokenOwnerRecord::get_space();

        // Act
        let actual_space = DISCRIMINATOR_SIZE
            + NftVoterTokenOwnerRecord::default()
                .try_to_vec()
                .unwrap()
                .len();

        // Assert
        assert_eq!(expected_space, actual_space);
    }
}
