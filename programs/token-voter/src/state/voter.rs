use {
    super::{DepositEntry, Registrar},
    crate::error::TokenVoterError,
    anchor_lang::{prelude::*, Discriminator},
    solana_program::pubkey::PUBKEY_BYTES,
    spl_governance::state::token_owner_record,
};

/// User account for mint voting rights.
#[account]
#[derive(Debug, PartialEq)]
pub struct Voter {
    /// Voter Authority who owns the account tokens.
    pub voter_authority: Pubkey,

    /// Registrar in which the voter is created in.
    pub registrar: Pubkey,

    /// Deposit entries for a deposit for a given mint.
    pub deposits: Vec<DepositEntry>,

    /// Voter account bump.
    pub voter_bump: u8,

    /// Voter weight record account bump.
    pub voter_weight_record_bump: u8,

    /// Reserved for future upgrades
    pub reserved: [u8; 94],
}

const_assert!(std::mem::size_of::<Voter>() % 8 == 0);

impl Voter {
    pub fn get_space(max_mints: u8) -> usize {
        Voter::discriminator().len() + PUBKEY_BYTES * 2 + (max_mints as usize * 64) + 1 + 1 + 94
    }

    pub fn clock_unix_timestamp(&self) -> i64 {
        Clock::get().unwrap().unix_timestamp
    }

    /// The full vote weight available to the voter
    pub fn weight(&self, registrar: &Registrar) -> Result<u64> {
        self.deposits
            .iter()
            .filter(|d| d.is_used)
            .try_fold(0u64, |sum, d| {
                d.voting_power(&registrar.voting_mint_configs[d.voting_mint_config_idx as usize])
                    .map(|vp| sum.checked_add(vp).unwrap())
            })
    }

    pub fn active_deposit_mut(&mut self, index: u8) -> Result<&mut DepositEntry> {
        let index = index as usize;
        require_gt!(
            self.deposits.len(),
            index,
            TokenVoterError::OutOfBoundsDepositEntryIndex
        );

        let d = &mut self.deposits[index];
        // if deposit_slot_hash is 0 then deposit is inactive
        if d.deposit_slot_hash == 0 {
            return Err(TokenVoterError::DepositIndexInactive.into());
        }

        Ok(d)
    }

    pub fn load_token_owner_record(
        &self,
        account_info: &AccountInfo,
        registrar: &Registrar,
        voter_authority: &Pubkey,
    ) -> Result<token_owner_record::TokenOwnerRecordV2> {
        let record = token_owner_record::get_token_owner_record_data_for_realm_and_governing_mint(
            &registrar.governance_program_id,
            account_info,
            &registrar.realm,
            &registrar.governing_token_mint,
        )?;
        require_keys_eq!(
            record.governing_token_owner,
            *voter_authority,
            TokenVoterError::InvalidTokenOwnerRecord
        );
        Ok(record)
    }
}

#[macro_export]
macro_rules! voter_seeds_no_seeds {
    ( $voter:expr, $voter_authority:expr  ) => {
        &[
            $voter.registrar.as_ref(),
            b"voter".as_ref(),
            $voter_authority.as_ref(),
        ]
    };
}
#[macro_export]
macro_rules! voter_seeds {
    ( $voter:expr, $voter_authority:expr  ) => {
        &[
            $voter.registrar.as_ref(),
            b"voter".as_ref(),
            $voter_authority.as_ref(),
            &[$voter.voter_bump],
        ]
    };
}

pub use voter_seeds_no_seeds;

pub use voter_seeds;
