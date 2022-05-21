use anchor_lang::prelude::*;

#[error_code]
pub enum SquadsVoterError {
    #[msg("Invalid Realm Authority")]
    InvalidRealmAuthority,

    #[msg("Invalid Realm for Registrar")]
    InvalidRealmForRegistrar,

    #[msg("Invalid MaxVoterWeightRecord Realm")]
    InvalidMaxVoterWeightRecordRealm,

    #[msg("Invalid MaxVoterWeightRecord Mint")]
    InvalidMaxVoterWeightRecordMint,

    #[msg("Invalid VoterWeightRecord Realm")]
    InvalidVoterWeightRecordRealm,

    #[msg("Invalid VoterWeightRecord Mint")]
    InvalidVoterWeightRecordMint,

    #[msg("Invalid TokenOwner for VoterWeightRecord")]
    InvalidTokenOwnerForVoterWeightRecord,

    #[msg("Squad not found")]
    SquadNotFound,

    #[msg("Duplicated Squad detected")]
    DuplicatedSquadDetected,
}
