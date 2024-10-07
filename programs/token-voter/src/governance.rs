/// A macro is exposed so that we can embed the program ID.
#[macro_export]
macro_rules! vote_weight_record {
    ($id:expr) => {
        /// Anchor wrapper for the SPL governance program's VoterWeightRecord type.
        #[derive(Clone)]
        pub struct VoterWeightRecord(spl_governance_addin_api::voter_weight::VoterWeightRecord);

        impl VoterWeightRecord {
            pub fn get_space() -> usize {
                8 + 32 * 4 + 8 + 1 + 8 + 1 + 1 + 1 + 8
            }

            pub fn new(
                realm: Pubkey,
                governing_token_mint: Pubkey,
                governing_token_owner: Pubkey,
                voter_weight: u64,
                voter_weight_expiry: Option<u64>,
                weight_action: Option<spl_governance_addin_api::voter_weight::VoterWeightAction>,
                weight_action_target: Option<Pubkey>,
            ) -> Self {
                let vwr = spl_governance_addin_api::voter_weight::VoterWeightRecord {
                    account_discriminator: spl_governance_addin_api::voter_weight::VoterWeightRecord::ACCOUNT_DISCRIMINATOR,
                    realm,
                    governing_token_mint,
                    governing_token_owner,
                    voter_weight,
                    voter_weight_expiry,
                    weight_action,
                    weight_action_target,
                    reserved: [0; 8],
                };
                VoterWeightRecord(vwr)
            }
        }

        impl anchor_lang::AccountDeserialize for VoterWeightRecord {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data = buf;
                let vwr: spl_governance_addin_api::voter_weight::VoterWeightRecord =
                    anchor_lang::AnchorDeserialize::deserialize(&mut data)
                        .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize)?;
                if !solana_program::program_pack::IsInitialized::is_initialized(&vwr) {
                    return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                }
                Ok(VoterWeightRecord(vwr))
            }

            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data = buf;
                let vwr: spl_governance_addin_api::voter_weight::VoterWeightRecord =
                    anchor_lang::AnchorDeserialize::deserialize(&mut data)
                        .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize)?;
                Ok(VoterWeightRecord(vwr))
            }
        }

        impl anchor_lang::AccountSerialize for VoterWeightRecord {
            fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
                anchor_lang::AnchorSerialize::serialize(&self.0, writer)
                    .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotSerialize)?;
                Ok(())
            }
        }

        impl anchor_lang::Owner for VoterWeightRecord {
            fn owner() -> Pubkey {
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

        #[cfg(feature = "idl-build")]
        impl anchor_lang::IdlBuild for VoterWeightRecord {}

        #[cfg(feature = "idl-build")]
        impl anchor_lang::Discriminator for VoterWeightRecord {
            const DISCRIMINATOR: [u8; 8] = [0; 8];
            fn discriminator() -> [u8; 8] {
                spl_governance_addin_api::voter_weight::VoterWeightRecord::ACCOUNT_DISCRIMINATOR
            }
        }
    };
}

/// A macro is exposed so that we can embed the program ID.
#[macro_export]
macro_rules! max_voter_weight_record {
    ($id:expr) => {
        /// Anchor wrapper for the SPL governance program's MaxVoterWeightRecord type.
        #[derive(Clone)]
        pub struct MaxVoterWeightRecord(spl_governance_addin_api::max_voter_weight::MaxVoterWeightRecord);

        impl MaxVoterWeightRecord {
            pub fn get_space() -> usize {
                8 + 32 * 2 + 8 + 1 + 8 + 8
            }

            pub fn new(
                realm: Pubkey,
                governing_token_mint: Pubkey,
                max_voter_weight: u64,
                max_voter_weight_expiry: Option<solana_program::clock::Slot>,
            ) -> Self {
                let mvwr = spl_governance_addin_api::max_voter_weight::MaxVoterWeightRecord {
                    account_discriminator: spl_governance_addin_api::max_voter_weight::MaxVoterWeightRecord::ACCOUNT_DISCRIMINATOR,
                    realm,
                    governing_token_mint,
                    max_voter_weight,
                    max_voter_weight_expiry,
                    reserved: [0; 8],
                };
                MaxVoterWeightRecord(mvwr)
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
                    &Self::get_max_voter_weight_record_seeds(realm, governing_token_mint),
                    &id(),
                )
                .0
            }
        }


        impl anchor_lang::AccountDeserialize for MaxVoterWeightRecord {
            fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data = buf;
                let mvwr: spl_governance_addin_api::max_voter_weight::MaxVoterWeightRecord =
                    anchor_lang::AnchorDeserialize::deserialize(&mut data)
                        .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize)?;
                if !solana_program::program_pack::IsInitialized::is_initialized(&mvwr) {
                    return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
                }
                Ok(MaxVoterWeightRecord(mvwr))
            }

            fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
                let mut data = buf;
                let mvwr: spl_governance_addin_api::max_voter_weight::MaxVoterWeightRecord =
                    anchor_lang::AnchorDeserialize::deserialize(&mut data)
                        .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize)?;
                Ok(MaxVoterWeightRecord(mvwr))
            }
        }

        impl anchor_lang::AccountSerialize for MaxVoterWeightRecord {
            fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
                anchor_lang::AnchorSerialize::serialize(&self.0, writer)
                    .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotSerialize)?;
                Ok(())
            }
        }

        impl anchor_lang::Owner for MaxVoterWeightRecord {
            fn owner() -> Pubkey {
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

        #[cfg(feature = "idl-build")]
        impl anchor_lang::IdlBuild for MaxVoterWeightRecord {}

        #[cfg(feature = "idl-build")]
        impl anchor_lang::Discriminator for MaxVoterWeightRecord {
            const DISCRIMINATOR: [u8; 8] = [0; 8];
            fn discriminator() -> [u8; 8] {
                spl_governance_addin_api::max_voter_weight::MaxVoterWeightRecord::ACCOUNT_DISCRIMINATOR
            }
        }
    };
}
