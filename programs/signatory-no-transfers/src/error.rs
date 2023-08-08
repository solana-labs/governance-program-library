use anchor_lang::prelude::*;

#[error_code]
pub enum TransactionCheckerError {
    #[msg("Proposal has not been fully checked.")]
    ProposalNotFullyChecked,
    #[msg("Wrong transaction provided: must provide next transaction in order to check.")]
    WrongTransaction,
    #[msg("Transaction does not match provided option.")]
    TransactionWrongOption,
    #[msg("Wrong beneficiary account provided. Must provide original payer.")]
    WrongBeneficiary,
    #[msg("Proposal has rejected transactions.")]
    ProposalRejected,
}
