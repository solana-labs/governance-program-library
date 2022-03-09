use anchor_lang::prelude::*;

#[error_code]
pub enum NftLockerErrorCode {
    #[msg("Invalid authority provided")]
    InvalidAuthority,
    #[msg("Msg")]
    RatesFull,
    #[msg("Invalid Realm Authority")]
    InvalidRealmAuthority,
    #[msg("Failed to decode metadata")]
    DecodeMetadataFailed,
    #[msg("Collection is invalid")]
    InvalidCollection,
    #[msg("Collection is not verified")]
    UnverifiedCollection,
    #[msg("There is no NFT in the account")]
    InsufficientAmountOnNFTAccount,
}
