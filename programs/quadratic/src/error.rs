use anchor_lang::prelude::*;

#[error_code]
pub enum QuadraticError {
    #[msg("Invalid realm authority")]
    InvalidRealmAuthority,

    #[msg("Invalid realm for the provided registrar")]
    InvalidRealmForRegistrar,

    #[msg("Invalid TokenOwnerRecord as input voter weight (expecting TokenOwnerRecord V1 or V2)")]
    InvalidPredecessorTokenOwnerRecord,

    #[msg("Invalid VoterWeightRecord as input voter weight (expecting VoterWeightRecord)")]
    InvalidPredecessorVoterWeightRecord,

    #[msg("Invalid VoterWeightRecord realm for input voter weight")]
    InvalidPredecessorVoterWeightRecordRealm,

    #[msg("Invalid VoterWeightRecord governance token mint for input voter weight")]
    InvalidPredecessorVoterWeightRecordGovTokenMint,

    #[msg("Invalid VoterWeightRecord governance token owner for input voter weight")]
    InvalidPredecessorVoterWeightRecordGovTokenOwner,

    #[msg("Invalid VoterWeightRecord realm")]
    InvalidVoterWeightRecordRealm,

    #[msg("Invalid VoterWeightRecord mint")]
    InvalidVoterWeightRecordMint,

    #[msg("Previous voter weight plugin required but not provided")]
    MissingPreviousVoterWeightPlugin,
}
