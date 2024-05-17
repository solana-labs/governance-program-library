use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct VoterWeightRecord {
    pub voter: Pubkey,
    pub weight: u64,
    pub last_deposit_or_withdrawal_slot: u64,
}

impl Sealed for VoterWeightRecord {}

impl Pack for VoterWeightRecord {
    const LEN: usize = 8 + 32 + 8;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let (voter, weight, last_deposit_or_withdrawal_slot) =
            array_refs![src, 0, 32, 40; Pubkey, u64, u64];
        Ok(VoterWeightRecord {
            voter: *voter,
            weight: *weight,
            last_deposit_or_withdrawal_slot: *last_deposit_or_withdrawal_slot,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let (voter, weight, last_deposit_or_withdrawal_slot) =
            array_mut_refs![dst, 0, 32, 40; Pubkey, u64, u64];
        *voter = self.voter;
        *weight = self.weight;
        *last_deposit_or_withdrawal_slot = self.last_deposit_or_withdrawal_slot;
    }
}
