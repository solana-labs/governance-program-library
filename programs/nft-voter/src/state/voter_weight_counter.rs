use anchor_lang::prelude::*;

use crate::id;
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

/// Returns VoterWeightCounter PDA seeds
pub fn get_voter_weight_counter_seeds<'a>(
    proposal: &'a Pubkey,
    governing_token_owner: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [
        b"voter-weight-counter",
        proposal.as_ref(),
        governing_token_owner.as_ref(),
    ]
}

/// Returns VoterWeightCounter PDA address
pub fn get_voter_weight_counter_address(
    proposal: &Pubkey,
    governing_token_owner: &Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_voter_weight_counter_seeds(proposal, governing_token_owner),
        &id(),
    )
    .0
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
