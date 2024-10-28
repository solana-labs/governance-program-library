use anchor_lang::prelude::*;
use gpl_shared::compose::VoterWeightRecordBase;
use solana_program::pubkey::PUBKEY_BYTES;

use crate::utils::anchor::DISCRIMINATOR_SIZE;

/// VoterWeightAction enum as defined in spl-governance-addin-api
/// It's redefined here for Anchor to export it to IDL
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq)]
pub enum VoterWeightAction {
    /// Cast vote for a proposal. Target: Proposal
    CastVote,

    /// Comment a proposal. Target: Proposal
    CommentProposal,

    /// Create Governance within a realm. Target: Realm
    CreateGovernance,

    /// Create a proposal for a governance. Target: Governance
    CreateProposal,

    /// Signs off a proposal for a governance. Target: Proposal
    /// Note: SignOffProposal is not supported in the current version
    SignOffProposal,
}

/// VoterWeightRecord account as defined in spl-governance-addin-api
/// It's redefined here without account_discriminator for Anchor to treat it as native account
///
/// The account is used as an api interface to provide voting power to the governance program from external addin contracts
#[account]
#[derive(Debug, PartialEq)]
pub struct VoterWeightRecord {
    pub realm: Pubkey,
    pub governing_token_mint: Pubkey,
    pub governing_token_owner: Pubkey,
    pub voter_weight: u64,
    pub voter_weight_expiry: Option<u64>,
    pub weight_action: Option<VoterWeightAction>,
    pub weight_action_target: Option<Pubkey>,
    pub reserved: [u8; 8],
}

impl VoterWeightRecord {
    pub fn get_space() -> usize {
        DISCRIMINATOR_SIZE + PUBKEY_BYTES * 4 + 8 + 1 + 8 + 1 + 1 + 1 + 8
    }
}

impl<'a> VoterWeightRecordBase<'a> for VoterWeightRecord {
    fn get_governing_token_mint(&'a self) -> &'a Pubkey {
        &self.governing_token_mint
    }

    fn get_governing_token_owner(&'a self) -> &'a Pubkey {
        &self.governing_token_owner
    }
}

impl Default for VoterWeightRecord {
    fn default() -> Self {
        Self {
            realm: Default::default(),
            governing_token_mint: Default::default(),
            governing_token_owner: Default::default(),
            voter_weight: Default::default(),
            voter_weight_expiry: Some(0),
            weight_action: Some(VoterWeightAction::CastVote),
            weight_action_target: Some(Default::default()),
            reserved: Default::default(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_get_space() {
        // Arrange
        let expected_space = VoterWeightRecord::get_space();

        // Act
        let actual_space =
            DISCRIMINATOR_SIZE + VoterWeightRecord::default().try_to_vec().unwrap().len();

        // Assert
        assert_eq!(expected_space, actual_space);
    }
}
