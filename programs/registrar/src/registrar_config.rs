use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

const MAX_ACCEPTED_TOKENS: usize = 10;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct RegistrarConfig {
    pub accepted_tokens: Vec<Pubkey>,
    pub weights: Vec<u64>,
}

impl Sealed for RegistrarConfig {}

impl Pack for RegistrarConfig {
    const LEN: usize = 8 + (4 + 32) * MAX_ACCEPTED_TOKENS;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let (accepted_tokens, weights) = array_refs![src, MAX_ACCEPTED_TOKENS; Pubkey, u64];
        let accepted_tokens = accepted_tokens.to_vec();
        let weights = weights.to_vec();
        Ok(RegistrarConfig {
            accepted_tokens,
            weights,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let (accepted_tokens, weights) = array_mut_refs![dst, MAX_ACCEPTED_TOKENS; Pubkey, u64];
        for (i, token) in self.accepted_tokens.iter().enumerate() {
            accepted_tokens[i] = *token;
            weights[i] = self.weights[i];
        }
    }
}
