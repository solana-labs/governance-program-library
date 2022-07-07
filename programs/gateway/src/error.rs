use anchor_lang::prelude::*;

#[error_code]
pub enum GatewayError {
    #[msg("Invalid realm authority")]
    InvalidRealmAuthority,

    #[msg("Invalid realm for the provided registrar")]
    InvalidRealmForRegistrar,

    #[msg("Invalid TokenOwnerRecord as input voter weight")]
    InvalidPredecessorTokenOwnerRecord,

    #[msg("Invalid VoterWeightRecord as input voter weight")]
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

    #[msg("Invalid TokenOwner for VoterWeightRecord")]
    InvalidTokenOwnerForVoterWeightRecord,

    #[msg("Invalid gateway token")]
    InvalidGatewayToken,

    #[msg("Previous voter weight plugin required but not provided")]
    MissingPreviousVoterWeightPlugin,
}
