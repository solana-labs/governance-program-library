use anchor_lang::prelude::*;

use crate::{id, tools::anchor::DISCRIMINATOR_SIZE};

/// Receipt for depositing tokens against a voting NFT to enable
/// token owners to withdraw tokens from the NFT's holding account
#[account]
#[derive(Debug, Copy, PartialEq, Default)]
pub struct DelegatorTokenOwnerRecord {
    /// The Realm the token owner is participating in
    pub realm: Pubkey,

    /// Governing Token Mint the token owner holds deposit for
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

impl DelegatorTokenOwnerRecord {
    pub const SEED_PREFIX: [u8; 28] = *b"delegator-token-owner-record";

    pub const SPACE: usize = DISCRIMINATOR_SIZE +
    32 + // realm
    32 + // governing token mint
    32 + // nft mint
    32 + // governing token owner
    8; // deposit amount

    pub fn make_seeds<'a>(
        realm: &'a Pubkey,
        governing_token_mint: &'a Pubkey,
        nft_mint: &'a Pubkey,
        governing_token_owner: &'a Pubkey,
    ) -> [&'a [u8]; 5] {
        [
            &DelegatorTokenOwnerRecord::SEED_PREFIX,
            realm.as_ref(),
            governing_token_mint.as_ref(),
            nft_mint.as_ref(),
            governing_token_owner.as_ref(),
        ]
    }

    pub fn get_seeds(&self) -> [&[u8]; 5] {
        DelegatorTokenOwnerRecord::make_seeds(
            &self.realm,
            &self.governing_token_mint,
            &self.nft_mint,
            &self.governing_token_owner,
        )
    }

    pub fn find_address(
        realm: &Pubkey,
        governing_token_mint: &Pubkey,
        nft_mint: &Pubkey,
        governing_token_owner: &Pubkey,
    ) -> Pubkey {
        Pubkey::find_program_address(
            &DelegatorTokenOwnerRecord::make_seeds(
                realm,
                governing_token_mint,
                nft_mint,
                governing_token_owner,
            ),
            &id(),
        )
        .0
    }
}

#[cfg(test)]
mod test {

    use crate::tools::anchor::DISCRIMINATOR_SIZE;

    use super::*;

    #[test]
    fn test_get_space() {
        // Arrange
        let expected_space = DelegatorTokenOwnerRecord::SPACE;

        // Act
        let actual_space = DISCRIMINATOR_SIZE
            + DelegatorTokenOwnerRecord::default()
                .try_to_vec()
                .unwrap()
                .len();

        // Assert
        assert_eq!(expected_space, actual_space);
    }
}
