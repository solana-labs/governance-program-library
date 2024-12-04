use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct MaxVoterWeightRecord {
    pub max_weight: u64,
}

impl Sealed for MaxVoterWeightRecord {}

impl Pack for MaxVoterWeightRecord {
    const LEN: usize = 8;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let max_weight = src.get(..8).ok_or(ProgramError::InvalidAccountData)?.get_u64();
        Ok(MaxVoterWeightRecord { max_weight })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst.get_mut(..8)
            .ok_or(ProgramError::InvalidAccountData)
            .unwrap()
            .copy_from_slice(&self.max_weight.to_le_bytes());
    }
}
