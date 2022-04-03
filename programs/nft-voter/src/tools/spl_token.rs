use anchor_lang::prelude::*;
use arrayref::array_ref;
use spl_governance::tools::spl_token::assert_is_valid_spl_token_account;

/// Computationally cheap method to get amount from a token account
/// It reads amount without deserializing full account data
pub fn get_spl_token_amount(token_account_info: &AccountInfo) -> Result<u64> {
    assert_is_valid_spl_token_account(token_account_info)?;

    // TokeAccount layout:   mint(32), owner(32), amount(8), ...
    let data = token_account_info.try_borrow_data()?;
    let amount_bytes = array_ref![data, 64, 8];

    Ok(u64::from_le_bytes(*amount_bytes))
}
