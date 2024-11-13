use {crate::state::VotingMintConfig, anchor_lang::prelude::*};

/// Bookkeeping for a single deposit for a given mint.
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, Copy, PartialEq)]
pub struct DepositEntry {
    /// Amount in deposited, in native currency.
    /// Withdraws directly reduce this amount.
    ///
    /// This directly tracks the total amount added by the user. They may
    /// never withdraw more than this amount.
    pub amount_deposited_native: u64,

    /// Points to the VotingMintConfig this deposit uses.
    pub voting_mint_config_idx: u8,

    /// Deposit slot hash.
    /// saves deposit slot hash so that depositor cannot withdraw at the same slot.
    pub deposit_slot_hash: u64,

    // True if the deposit entry is being used.
    pub is_used: bool,

    /// Reserved for future upgrades
    pub reserved: [u8; 38],
}

const_assert!(std::mem::size_of::<DepositEntry>() == 8 + 1 + 8 + 1 + 38);
const_assert!(std::mem::size_of::<DepositEntry>() % 8 == 0);

impl DepositEntry {
    /// Creates a new DepositEntry with default values
    pub fn new() -> Self {
        Self {
            amount_deposited_native: 0,
            voting_mint_config_idx: 0,
            deposit_slot_hash: 0,
            is_used: false,
            reserved: [0; 38],
        }
    }
    /// Initializes a vector of DepositEntry with a given length
    pub fn init_deposits(length: usize) -> Vec<Self> {
        vec![Self::new(); length]
    }

    /// Voting Power Caclulation
    /// Returns the voting power for the deposit.
    pub fn voting_power(&self, mint_config: &VotingMintConfig) -> Result<u64> {
        let vote_weight = mint_config.digit_shift_native(self.amount_deposited_native)?;

        Ok(vote_weight)
    }
}

impl Default for DepositEntry {
    fn default() -> Self {
        Self::new()
    }
}
