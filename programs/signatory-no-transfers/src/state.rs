use anchor_lang::prelude::*;
use spl_governance::state::proposal::ProposalV2;

use crate::error::TransactionCheckerError;

#[account]
pub struct TransactionsChecked {
    pub transactions_checked: [u16; 256],
    pub payer: Pubkey,
    pub reject: bool,
}

impl TransactionsChecked {
    pub fn next_transaction_to_check(&self, option: u8) -> u16 {
        self.transactions_checked[option as usize]
    }

    pub fn mark_as_checked(&mut self, option: u8) {
        self.transactions_checked[option as usize] = self.transactions_checked[option as usize]
            .checked_add(1)
            .unwrap();
    }

    pub fn assert_full_checked(&self, proposal: &ProposalV2) -> Result<()> {
        let fully_checked = proposal
            .options
            .iter()
            .zip(self.transactions_checked.iter())
            .all(|(option, transactions_checked)| {
                option.transactions_count == *transactions_checked
            });

        match fully_checked {
            true => Ok(()),
            false => Err(TransactionCheckerError::ProposalNotFullyChecked.into()),
        }
    }

    pub fn get_transactions_checked_address(proposal: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(&Self::get_transactions_checked_seeds(proposal), &crate::ID).0
    }

    pub fn get_transactions_checked_seeds(proposal: &Pubkey) -> [&[u8]; 2] {
        [b"transactions-checked".as_ref(), proposal.as_ref()]
    }
}
