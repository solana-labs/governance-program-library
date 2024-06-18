use anchor_lang::prelude::*;

#[error_code]
pub enum TokenHaverError {
    #[msg("Invalid Realm Authority")]
    InvalidRealmAuthority,

    #[msg("Invalid Realm for Registrar")]
    InvalidRealmForRegistrar,

    #[msg("Invalid VoterWeightRecord Realm")]
    InvalidVoterWeightRecordRealm,

    #[msg("Invalid VoterWeightRecord Mint")]
    InvalidVoterWeightRecordMint,

    #[msg("Governing TokenOwner must match")]
    GoverningTokenOwnerMustMatch,

    #[msg("All token accounts must be owned by the governing token owner")]
    TokenAccountWrongOwner,

    #[msg("All token accounts' mints must be included in the registrar")]
    TokenAccountWrongMint,

    #[msg("All token accounts must be locked")]
    TokenAccountNotLocked,

    #[msg("All token accounts' mints must be unique")]
    TokenAccountDuplicateMint,
}
