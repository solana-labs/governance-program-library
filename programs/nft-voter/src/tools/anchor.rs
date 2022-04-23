use anchor_lang::prelude::AccountInfo;

pub const DISCRIMINATOR_SIZE: usize = 8;

pub const PUBKEY_SIZE: usize = 32;

pub const EMPTY_DISCRIMINATOR: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

// Checks if the given account is a newly initialized account
pub fn is_new_account(account_info: &AccountInfo) -> bool {
    account_info.data.borrow()[..8] == EMPTY_DISCRIMINATOR
}
