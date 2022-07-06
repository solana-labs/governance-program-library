use anchor_lang::prelude::*;

#[error_code]
pub enum GatewayError {
    #[msg("Invalid Realm Authority")]
    InvalidRealmAuthority,

    #[msg("Invalid TokenOwnerRecord as input voter weight")]
    InvalidPredecessorTokenOwnerRecord,

    #[msg("Invalid VoterWeightRecord as input voter weight")]
    InvalidPredecessorVoterWeightRecord,

    #[msg("Invalid VoterWeightRecord Realm for input voter weight")]
    InvalidPredecessorVoterWeightRecordRealm,

    #[msg("Invalid VoterWeightRecord Governance Token mint for input voter weight")]
    InvalidPredecessorVoterWeightRecordGovTokenMint,

    #[msg("Invalid VoterWeightRecord Governance Token owner for input voter weight")]
    InvalidPredecessorVoterWeightRecordGovTokenOwner,

    #[msg("Invalid VoterWeightRecord Realm")]
    InvalidVoterWeightRecordRealm,

    #[msg("Invalid VoterWeightRecord Mint")]
    InvalidVoterWeightRecordMint,

    #[msg("Invalid TokenOwner for VoterWeightRecord")]
    InvalidTokenOwnerForVoterWeightRecord,

    #[msg("Invalid gateway token")]
    InvalidGatewayToken,
}
