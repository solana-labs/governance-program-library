#[derive(Debug, PartialEq, Eq)]
pub enum RegistrarError {
    InvalidArgument,
    InvalidAccountData,
    InvalidOperation,
    Overflow,
    InsufficientFunds,
    // Add more error variants as needed
}

impl From<RegistrarError> for ProgramError {
    fn from(error: RegistrarError) -> Self {
        ProgramError::Custom(error as u32)
    }
}
