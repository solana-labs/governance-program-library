use anchor_lang::prelude::Pubkey;
use spl_governance::state::{token_owner_record, vote_record};

use crate::id;

pub const NFT_POWER_HOLDING_ACCOUNT_SEED_PREFIX: [u8; 25] = *b"nft-power-holding-account";

pub fn find_nft_power_holding_account_address(
    realm: &Pubkey,
    governance_token_mint: &Pubkey,
    nft_mint: &Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &[
            &NFT_POWER_HOLDING_ACCOUNT_SEED_PREFIX,
            realm.as_ref(),
            governance_token_mint.as_ref(),
            nft_mint.as_ref(),
        ],
        &id(),
    )
    .0
}

pub fn make_nft_power_holding_account_seeds<'a>(
    realm: &'a Pubkey,
    governance_token_mint: &'a Pubkey,
    nft_mint: &'a Pubkey,
) -> [&'a [u8]; 4] {
    [
        &NFT_POWER_HOLDING_ACCOUNT_SEED_PREFIX,
        realm.as_ref(),
        governance_token_mint.as_ref(),
        nft_mint.as_ref(),
    ]
}

pub fn get_vote_record_address(
    program_id: &Pubkey,
    realm: &Pubkey,
    governing_token_mint: &Pubkey,
    governing_token_owner: &Pubkey,
    proposal: &Pubkey,
) -> Pubkey {
    let token_owner_record_key = token_owner_record::get_token_owner_record_address(
        program_id,
        realm,
        governing_token_mint,
        governing_token_owner,
    );

    vote_record::get_vote_record_address(program_id, proposal, &token_owner_record_key)
}
