use anchor_lang::prelude::*;

use super::VoterWeightAction;

#[account]
pub struct StakeDepositRecord {
    pub deposits: Vec<Pubkey>,
    pub weight_action_target: Option<Pubkey>,
    pub weight_action: Option<VoterWeightAction>,
    pub deposits_len: u8,
    pub bump: u8,
    pub previous_voter_weight: u64,
}

impl StakeDepositRecord {
    pub const FIXED_LEN: usize = 8 + 33 + 4 + 1 + 1 + 8 + 2;

    pub fn realloc_bytes(
        &self,
        new_receipts_len: u8,
        action_target: Pubkey,
        action: VoterWeightAction,
    ) -> usize {
        let new_len = self.new_deposit_len(new_receipts_len, action_target, action);
        StakeDepositRecord::FIXED_LEN + (new_len as usize) * 32
    }

    pub fn new_deposit_len(
        &self,
        new_receipts_len: u8,
        action_target: Pubkey,
        action: VoterWeightAction,
    ) -> u8 {
        let new_len = if Some(action_target) == self.weight_action_target
            && Some(action) == self.weight_action
        {
            let current_len = self.deposits.len() as u8;
            current_len.checked_add(new_receipts_len).unwrap()
        } else {
            new_receipts_len
        };

        if new_len > self.deposits_len {
            new_len
        } else {
            self.deposits_len
        }
    }
}
