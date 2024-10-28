use anchor_lang::prelude::*;

#[error_code]
pub enum BonkPluginError {
    #[msg("Invalid Realm Authority")]
    InvalidRealmAuthority,
    #[msg("The mint of the stake pool is different from the realm")]
    InvalidGoverningToken,
    #[msg("Invalid VoterWeightRecord Realm")]
    InvalidVoterWeightRecordRealm,
    #[msg("Invalid VoterWeightRecord Mint")]
    InvalidVoterWeightRecordMint,
    #[msg("Invalid Stake Pool")]
    InvalidStakePool,
    #[msg("Invalid TokenOwner for VoterWeightRecord")]
    InvalidTokenOwnerForVoterWeightRecord,
    #[msg("The owner of the receipt does not match")]
    VoterDoesNotOwnDepositReceipt,
    #[msg("The deposit receipt was already provided")]
    DuplicatedReceiptDetected,
    #[msg("The stake deposit receipt has already expired")]
    ExpiredStakeDepositReceipt,
    #[msg("The stake deposit receipt will expire before proposal")]
    InvalidStakeDuration,
    #[msg("The stake deposit receipts count does not match")]
    ReceiptsCountMismatch,
    #[msg("Proposal account is required for Cast Vote action")]
    ProposalAccountIsRequired,
    #[msg("Action target is different from the public key of the proposal")]
    ActionTargetMismatch,
    #[msg("Maximum deposits length reached")]
    MaximumDepositsReached,
}
