use anchor_lang::prelude::*;

#[error_code]
pub enum TokenVoterError {
    #[msg("Invalid Realm Authority")]
    InvalidRealmAuthority,

    #[msg("Invalid Realm for Registrar")]
    InvalidRealmForRegistrar,

    #[msg("Invalid Collection Size")]
    InvalidCollectionSize,

    #[msg("Invalid MaxVoterWeightRecord Realm")]
    InvalidMaxVoterWeightRecordRealm,

    #[msg("Invalid MaxVoterWeightRecord Mint")]
    InvalidMaxVoterWeightRecordMint,

    #[msg("CastVote Is Not Allowed")]
    CastVoteIsNotAllowed,

    #[msg("Invalid VoterWeightRecord Realm")]
    InvalidVoterWeightRecordRealm,

    #[msg("Invalid VoterWeightRecord Mint")]
    InvalidVoterWeightRecordMint,

    #[msg("Invalid TokenOwner for VoterWeightRecord")]
    InvalidTokenOwnerForVoterWeightRecord,

    #[msg("Collection must be verified")]
    CollectionMustBeVerified,

    #[msg("Voter does not own NFT")]
    VoterDoesNotOwnNft,

    #[msg("Collection not found")]
    CollectionNotFound,

    #[msg("Missing Metadata collection")]
    MissingMetadataCollection,

    #[msg("Token Metadata doesn't match")]
    TokenMetadataDoesNotMatch,

    #[msg("Invalid account owner")]
    InvalidAccountOwner,

    #[msg("Invalid token metadata account")]
    InvalidTokenMetadataAccount,

    #[msg("Duplicated NFT detected")]
    DuplicatedNftDetected,

    #[msg("Invalid NFT amount")]
    InvalidNftAmount,

    #[msg("NFT already voted")]
    NftAlreadyVoted,

    #[msg("Invalid Proposal for NftVoteRecord")]
    InvalidProposalForNftVoteRecord,

    #[msg("Invalid TokenOwner for NftVoteRecord")]
    InvalidTokenOwnerForNftVoteRecord,

    #[msg("VoteRecord must be withdrawn")]
    VoteRecordMustBeWithdrawn,

    #[msg("Invalid VoteRecord for NftVoteRecord")]
    InvalidVoteRecordForNftVoteRecord,

    #[msg("VoterWeightRecord must be expired")]
    VoterWeightRecordMustBeExpired,

    #[msg("Mathematical Overflow")]
    Overflow,

    /// Invalid Token account owner
    #[msg("Invalid Token account owner")]
    SplTokenAccountWithInvalidOwner,

    /// Invalid Mint account owner
    #[msg("Invalid Mint account owner")]
    SplTokenMintWithInvalidOwner,

    /// Token Account is not initialized
    #[msg("Token Account is not initialized")]
    SplTokenAccountNotInitialized,

    /// Token Account doesn't exist
    #[msg("Token Account doesn't exist")]
    SplTokenAccountDoesNotExist,

    /// Token account data is invalid
    #[msg("Token account data is invalid")]
    SplTokenInvalidTokenAccountData,

    /// Token mint account data is invalid
    #[msg("Token mint account data is invalid")]
    SplTokenInvalidMintAccountData,

    /// Token Mint is not initialized
    #[msg("Token Mint account is not initialized")]
    SplTokenMintNotInitialized,

    /// Token Mint account doesn't exist
    #[msg("Token Mint account doesn't exist")]
    SplTokenMintDoesNotExist,

    /// Current mint authority must sign transaction
    #[msg("Current mint authority must sign transaction")]
    MintAuthorityMustSign,

    /// Invalid mint authority
    #[msg("Invalid mint authority")]
    InvalidMintAuthority,

    /// Mint has no authority
    #[msg("Mint has no authority")]
    MintHasNoAuthority,

    /// Invalid token owner
    #[msg("Invalid token owner")]
    InvalidTokenOwner,

    /// Current token owner must sign transaction
    #[msg("Current token owner must sign transaction")]
    TokenOwnerMustSign,

    /// Account data is empty or invalid
    #[msg("Account Data is empty or invalid")]
    InvalidAccountData,

    /// Invalid Governing Token Mint
    #[msg("Invalid Governing Token Mint")]
    InvalidGoverningTokenMint,

    /// Math Overflow in VoterWeight
    #[msg("Math Overflow in VoterWeight")]
    VoterWeightOverflow,

    #[msg("Invalid Proposal for NftVoteRecord")]
    InvalidProposalForTokenVoteRecord,

    #[msg("Invalid TokenOwner for NftVoteRecord")]
    InvalidTokenOwnerForTokenVoteRecord,

    #[msg("Mint Not Found in Mint Configs")]
    MintNotFound,

    #[msg("Governing TokenOwner must match")]
    GoverningTokenOwnerMustMatch,

    #[msg("Invalid Token Owner Records")]
    InvalidTokenOwnerRecord,

    #[msg("Index is out of Deposit Entry bounds")]
    OutOfBoundsDepositEntryIndex,

    #[msg("No Cpi Allowed")]
    ForbiddenCpi,

    #[msg("Voting Tokens are not withdrawn")]
    VotingTokenNonZero,

    #[msg("Vault Tokens are not withdrawn")]
    VaultTokenNonZero,

    #[msg("Invalid Voter Token Authority")]
    InvalidAuthority,

    /// Token Amount Overflow
    #[msg("Math Overflow in Token Amount")]
    TokenAmountOverflow,

    /// Withdrawal in the same slot.
    #[msg("Cannot Withdraw in the same slot")]
    CannotWithdraw,

    #[msg("Voting Mint has Incorrect Index")]
    VotingMintConfiguredWithDifferentIndex,

    #[msg("Voting Mint index is already in use")]
    VotingMintConfigIndexAlreadyInUse,

    #[msg("Index is out of Voting Mint Config bounds")]
    OutOfBoundsVotingMintConfigIndex,

    #[msg("Resizing Max Mints cannot be smaller than Configure Mint Configs")]
    InvalidResizeMaxMints,
}
