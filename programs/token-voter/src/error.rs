use anchor_lang::prelude::*;

#[error_code]
pub enum TokenVoterError {
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

    #[msg("Mathematical Overflow")]
    Overflow,

    /// Invalid Token account owner
    #[msg("Invalid Token account owner")]
    SplTokenAccountWithInvalidOwner,

    /// Invalid Mint account owner
    #[msg("Invalid Mint account owner")]
    SplTokenMintWithInvalidOwner,

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

    /// Account data is empty or invalid
    #[msg("Account Data is empty or invalid")]
    InvalidAccountData,

    /// Math Overflow in VoterWeight
    #[msg("Math Overflow in VoterWeight")]
    VoterWeightOverflow,

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

    #[msg("Resizing Max Mints cannot be smaller than Configure Mint Configs")]
    InvalidResizeMaxMints,

    #[msg("Mint Index mismatch!")]
    MintIndexMismatch,

    #[msg("Inactive Deposit Index!")]
    DepositIndexInactive,
}
