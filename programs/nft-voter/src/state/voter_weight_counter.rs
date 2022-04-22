use anchor_lang::prelude::*;

use crate::tools::anchor::{DISCRIMINATOR_SIZE, PUBKEY_SIZE};

/// VoterWeightCounter account used to calculate cumulative voter weight using multiple instructions
#[account]
#[derive(Debug, PartialEq, Default)]
pub struct VoterWeightCounter {
    /// Proposal which the voter weight is counted for
    pub proposal: Pubkey,

    /// The voter whose weight is being counted
    /// It's a Realm member pubkey corresponding to TokenOwnerRecord.governing_token_owner
    pub governing_token_owner: Pubkey,

    /// Voter's weight
    pub voter_weight: u64,

    /// Reserved for future upgrades
    pub reserved: [u8; 8],
}

impl VoterWeightCounter {
    pub fn get_space() -> usize {
        DISCRIMINATOR_SIZE + PUBKEY_SIZE * 2 + 8 + 8
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_get_space() {
        // Arrange
        let expected_space = VoterWeightCounter::get_space();

        // Act
        let actual_space =
            DISCRIMINATOR_SIZE + VoterWeightCounter::default().try_to_vec().unwrap().len();

        // Assert
        assert_eq!(expected_space, actual_space);
    }
}
