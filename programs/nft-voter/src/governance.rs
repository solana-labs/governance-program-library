use crate::id;
use anchor_lang::prelude::Pubkey;

/// A macro is exposed so that we can embed the program ID.
#[macro_export]
macro_rules! voter_weight_record {
    ($id:expr) => {
        /// Anchor wrapper for the SPL governance program's VoterWeightRecord type.
        #[derive(Clone)]
        pub struct VoterWeightRecord(spl_governance_addin_api::voter_weight::VoterWeightRecord);

        impl anchor_lang::AccountDeserialize for VoterWeightRecord {
            fn try_deserialize(
                buf: &mut &[u8],
            ) -> std::result::Result<Self, anchor_lang::error::Error> {
                let mut data = buf;
                let vwr: spl_governance_addin_api::voter_weight::VoterWeightRecord =
                    anchor_lang::AnchorDeserialize::deserialize(&mut data)
                        .map_err(|_| anchor_lang::prelude::ErrorCode::AccountDidNotDeserialize)?;
                if !solana_program::program_pack::IsInitialized::is_initialized(&vwr) {
                    return Err(anchor_lang::prelude::ErrorCode::AccountDidNotSerialize.into());
                }
                Ok(VoterWeightRecord(vwr))
            }

            fn try_deserialize_unchecked(
                buf: &mut &[u8],
            ) -> std::result::Result<Self, anchor_lang::error::Error> {
                let mut data = buf;
                let vwr: spl_governance_addin_api::voter_weight::VoterWeightRecord =
                    anchor_lang::AnchorDeserialize::deserialize(&mut data)
                        .map_err(|_| anchor_lang::prelude::ErrorCode::AccountDidNotDeserialize)?;
                Ok(VoterWeightRecord(vwr))
            }
        }

        impl anchor_lang::AccountSerialize for VoterWeightRecord {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> std::result::Result<(), anchor_lang::error::Error> {
                anchor_lang::AnchorSerialize::serialize(&self.0, writer)
                    .map_err(|_| anchor_lang::prelude::ErrorCode::AccountDidNotSerialize)?;
                Ok(())
            }
        }

        impl anchor_lang::Owner for VoterWeightRecord {
            fn owner() -> anchor_lang::prelude::Pubkey {
                $id
            }
        }

        impl std::ops::Deref for VoterWeightRecord {
            type Target = spl_governance_addin_api::voter_weight::VoterWeightRecord;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for VoterWeightRecord {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Default for VoterWeightRecord {
            fn default() -> Self {

                VoterWeightRecord( spl_governance_addin_api::voter_weight::VoterWeightRecord{
                account_discriminator: spl_governance_addin_api::voter_weight::VoterWeightRecord::ACCOUNT_DISCRIMINATOR,
                realm: anchor_lang::prelude::Pubkey::default(),
                governing_token_mint: anchor_lang::prelude::Pubkey::default(),
                governing_token_owner: anchor_lang::prelude::Pubkey::default(),
                voter_weight:0,
                voter_weight_expiry:Some(0),
                weight_action:Some(spl_governance_addin_api::voter_weight::VoterWeightAction::CastVote),
                weight_action_target: Some(anchor_lang::prelude::Pubkey::default()),
                reserved: [0; 8]
                })
            }
        }
    };
}

/// A macro is exposed so that we can embed the program ID.
#[macro_export]
macro_rules! max_voter_weight_record {
    ($id:expr) => {
        /// Anchor wrapper for the SPL governance program's VoterWeightRecord type.
        #[derive(Clone)]
        pub struct MaxVoterWeightRecord(spl_governance_addin_api::max_voter_weight::MaxVoterWeightRecord);

        impl anchor_lang::AccountDeserialize for MaxVoterWeightRecord {
            fn try_deserialize(
                buf: &mut &[u8],
            ) -> std::result::Result<Self, anchor_lang::error::Error> {
                let mut data = buf;
                let vwr: spl_governance_addin_api::max_voter_weight::MaxVoterWeightRecord =
                    anchor_lang::AnchorDeserialize::deserialize(&mut data)
                        .map_err(|_| anchor_lang::prelude::ErrorCode::AccountDidNotDeserialize)?;
                if !solana_program::program_pack::IsInitialized::is_initialized(&vwr) {
                    return Err(anchor_lang::prelude::ErrorCode::AccountDidNotSerialize.into());
                }
                Ok(MaxVoterWeightRecord(vwr))
            }

            fn try_deserialize_unchecked(
                buf: &mut &[u8],
            ) -> std::result::Result<Self, anchor_lang::error::Error> {
                let mut data = buf;
                let vwr: spl_governance_addin_api::max_voter_weight::MaxVoterWeightRecord =
                    anchor_lang::AnchorDeserialize::deserialize(&mut data)
                        .map_err(|_| anchor_lang::prelude::ErrorCode::AccountDidNotDeserialize)?;
                Ok(MaxVoterWeightRecord(vwr))
            }
        }

        impl anchor_lang::AccountSerialize for MaxVoterWeightRecord {
            fn try_serialize<W: std::io::Write>(
                &self,
                writer: &mut W,
            ) -> std::result::Result<(), anchor_lang::error::Error> {
                anchor_lang::AnchorSerialize::serialize(&self.0, writer)
                    .map_err(|_| anchor_lang::prelude::ErrorCode::AccountDidNotSerialize)?;
                Ok(())
            }
        }

        impl anchor_lang::Owner for MaxVoterWeightRecord {
            fn owner() -> anchor_lang::prelude::Pubkey {
                $id
            }
        }

        impl std::ops::Deref for MaxVoterWeightRecord {
            type Target = spl_governance_addin_api::max_voter_weight::MaxVoterWeightRecord;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for MaxVoterWeightRecord {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Default for MaxVoterWeightRecord {
            fn default() -> Self {

                MaxVoterWeightRecord( spl_governance_addin_api::max_voter_weight::MaxVoterWeightRecord{
                account_discriminator: spl_governance_addin_api::max_voter_weight::MaxVoterWeightRecord::ACCOUNT_DISCRIMINATOR,
                realm: anchor_lang::prelude::Pubkey::default(),
                governing_token_mint: anchor_lang::prelude::Pubkey::default(),
                max_voter_weight:0,
                max_voter_weight_expiry:Some(0),
                reserved: [0; 8]
                })
            }
        }
    };
}

/// Returns MaxVoterWeightRecord PDA seeds
pub fn get_max_voter_weight_record_seeds<'a>(
    realm: &'a Pubkey,
    governing_token_mint: &'a Pubkey,
) -> [&'a [u8]; 3] {
    [
        b"max-voter-weight-record",
        realm.as_ref(),
        governing_token_mint.as_ref(),
    ]
}

/// Returns MaxVoterWeightRecord PDA address
pub fn get_max_voter_weight_record_address(
    realm: &Pubkey,
    governing_token_mint: &Pubkey,
) -> Pubkey {
    Pubkey::find_program_address(
        &get_max_voter_weight_record_seeds(realm, governing_token_mint),
        &id(),
    )
    .0
}
