use anchor_lang::prelude::*;

#[error_code]
pub enum NftLockerError {
    #[msg("Invalid Realm Authority")]
    InvalidRealmAuthority,

    #[msg("Invalid Registrar Realm")]
    InvalidRegistrarRealm,

    #[msg("Failed to decode metadata")]
    DecodeMetadataFailed,

    #[msg("Given collection is not valid")]
    InvalidCollection,

    #[msg("Given NFT is not part of a collection or metadata format is not V2")]
    NotPartOfCollection,

    #[msg("Collection is not verified")]
    UnverifiedCollection,

    #[msg("There is no NFT in the account")]
    InsufficientAmountOnNFTAccount,

    #[msg("Invalid Collection Size")]
    InvalidCollectionSize,

    #[msg("Invalid MaxVoterWeightRecord Realm")]
    InvalidMaxVoterWeightRecordRealm,

    #[msg("Invalid MaxVoterWeightRecord Mint")]
    InvalidMaxVoterWeightRecordMint,

    #[msg("Proposal is not in voting state")]
    ProposalNotInVotingState,
}
