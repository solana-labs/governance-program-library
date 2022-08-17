use std::str::FromStr;
use std::sync::Arc;

use anchor_lang::prelude::{AccountMeta, Pubkey};

use anchor_spl::token::TokenAccount;
use gpl_nft_voter::state::max_voter_weight_record::{
    get_max_voter_weight_record_address, MaxVoterWeightRecord,
};
use gpl_nft_voter::state::*;
use gpl_nft_voter::tools::governance::find_nft_power_holding_account_address;
use solana_sdk::transport::TransportError;
use spl_governance::instruction::cast_vote;
use spl_governance::state::vote_record::{self, Vote, VoteChoice};

use gpl_nft_voter::state::{
    get_nft_vote_record_address, get_registrar_address, CollectionConfig, NftVoteRecord, Registrar,
};

use solana_program_test::ProgramTest;
use solana_sdk::instruction::Instruction;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;

use crate::program_test::governance_test::GovernanceTest;
use crate::program_test::program_test_bench::ProgramTestBench;

use crate::program_test::governance_test::{ProposalCookie, RealmCookie, TokenOwnerRecordCookie};
use crate::program_test::program_test_bench::WalletCookie;
use crate::program_test::token_metadata_test::{NftCollectionCookie, NftCookie, TokenMetadataTest};
use crate::program_test::tools::NopOverride;

use super::program_test_bench::TokenAccountCookie;

#[derive(Debug, PartialEq)]
pub struct RegistrarCookie {
    pub address: Pubkey,
    pub account: Registrar,

    pub realm_authority: Keypair,
    pub max_collections: u8,
}

pub struct VoterWeightRecordCookie {
    pub address: Pubkey,
    pub account: VoterWeightRecord,
}

pub struct MaxVoterWeightRecordCookie {
    pub address: Pubkey,
    pub account: MaxVoterWeightRecord,
}

pub struct CollectionConfigCookie {
    pub collection_config: CollectionConfig,
}

pub struct ConfigureCollectionArgs {
    pub weight: u64,
    pub size: u32,
}

impl Default for ConfigureCollectionArgs {
    fn default() -> Self {
        Self { weight: 1, size: 3 }
    }
}

pub struct GovernanceTokenHoldingAccountCookie {
    pub address: Pubkey,
    pub account: TokenAccount,
}

#[derive(Debug, PartialEq)]
pub struct NftVoterTokenOwnerRecordCookie {
    pub address: Pubkey,
    pub account: DelegatorTokenOwnerRecord,
}

#[derive(Debug, PartialEq)]
pub struct NftVoteRecordCookie {
    pub address: Pubkey,
    pub account: NftVoteRecord,
}

pub struct CastNftVoteArgs {
    pub cast_spl_gov_vote: bool,
}

impl Default for CastNftVoteArgs {
    fn default() -> Self {
        Self {
            cast_spl_gov_vote: true,
        }
    }
}

pub struct NftVoterTest {
    pub program_id: Pubkey,
    pub bench: Arc<ProgramTestBench>,
    pub governance: GovernanceTest,
    pub token_metadata: TokenMetadataTest,
}

impl NftVoterTest {
    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("gpl_nft_voter", gpl_nft_voter::id(), None);
    }

    #[allow(dead_code)]
    pub async fn start_new() -> Self {
        let mut program_test = ProgramTest::default();

        NftVoterTest::add_program(&mut program_test);
        GovernanceTest::add_program(&mut program_test);
        TokenMetadataTest::add_program(&mut program_test);

        let program_id = gpl_nft_voter::id();

        let bench = ProgramTestBench::start_new(program_test).await;
        let bench_rc = Arc::new(bench);

        let governance_bench =
            GovernanceTest::new(bench_rc.clone(), Some(program_id), Some(program_id));
        let token_metadata_bench = TokenMetadataTest::new(bench_rc.clone());

        Self {
            program_id,
            bench: bench_rc,
            governance: governance_bench,
            token_metadata: token_metadata_bench,
        }
    }

    #[allow(dead_code)]
    pub async fn with_registrar(
        &mut self,
        realm_cookie: &RealmCookie,
    ) -> Result<RegistrarCookie, TransportError> {
        self.with_registrar_using_ix(realm_cookie, NopOverride, None)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_registrar_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        realm_cookie: &RealmCookie,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<RegistrarCookie, TransportError> {
        let registrar_key =
            get_registrar_address(&realm_cookie.address, &realm_cookie.account.community_mint);

        let max_collections = 10;

        let data =
            anchor_lang::InstructionData::data(&gpl_nft_voter::instruction::CreateRegistrar {
                max_collections,
            });

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &gpl_nft_voter::accounts::CreateRegistrar {
                registrar: registrar_key,
                realm: realm_cookie.address,
                governance_program_id: self.governance.program_id,
                governing_token_mint: realm_cookie.account.community_mint,
                realm_authority: realm_cookie.get_realm_authority().pubkey(),
                payer: self.bench.payer.pubkey(),
                system_program: solana_sdk::system_program::id(),
            },
            None,
        );

        let mut create_registrar_ix = Instruction {
            program_id: gpl_nft_voter::id(),
            accounts,
            data,
        };

        instruction_override(&mut create_registrar_ix);

        let default_signers = &[&realm_cookie.realm_authority];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[create_registrar_ix], Some(signers))
            .await?;

        let account = Registrar {
            governance_program_id: self.governance.program_id,
            realm: realm_cookie.address,
            governing_token_mint: realm_cookie.account.community_mint,
            collection_configs: vec![],
            reserved: [0; 128],
        };

        Ok(RegistrarCookie {
            address: registrar_key,
            account,
            realm_authority: realm_cookie.get_realm_authority(),
            max_collections,
        })
    }

    #[allow(dead_code)]
    pub async fn with_voter_weight_record(
        &self,
        registrar_cookie: &RegistrarCookie,
        voter_cookie: &WalletCookie,
    ) -> Result<VoterWeightRecordCookie, TransportError> {
        self.with_voter_weight_record_using_ix(registrar_cookie, voter_cookie, NopOverride)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_voter_weight_record_using_ix<F: Fn(&mut Instruction)>(
        &self,
        registrar_cookie: &RegistrarCookie,
        voter_cookie: &WalletCookie,
        instruction_override: F,
    ) -> Result<VoterWeightRecordCookie, TransportError> {
        let governing_token_owner = voter_cookie.address;

        let (voter_weight_record_key, _) = Pubkey::find_program_address(
            &[
                b"voter-weight-record".as_ref(),
                registrar_cookie.account.realm.as_ref(),
                registrar_cookie.account.governing_token_mint.as_ref(),
                governing_token_owner.as_ref(),
            ],
            &gpl_nft_voter::id(),
        );

        let data = anchor_lang::InstructionData::data(
            &gpl_nft_voter::instruction::CreateVoterWeightRecord {
                governing_token_owner,
            },
        );

        let accounts = gpl_nft_voter::accounts::CreateVoterWeightRecord {
            governance_program_id: self.governance.program_id,
            realm: registrar_cookie.account.realm,
            realm_governing_token_mint: registrar_cookie.account.governing_token_mint,
            voter_weight_record: voter_weight_record_key,
            payer: self.bench.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        };

        let mut create_voter_weight_record_ix = Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        };

        instruction_override(&mut create_voter_weight_record_ix);

        self.bench
            .process_transaction(&[create_voter_weight_record_ix], None)
            .await?;

        let account = VoterWeightRecord {
            realm: registrar_cookie.account.realm,
            governing_token_mint: registrar_cookie.account.governing_token_mint,
            governing_token_owner,
            voter_weight: 0,
            voter_weight_expiry: Some(0),
            weight_action: None,
            weight_action_target: None,
            reserved: [0; 8],
        };

        Ok(VoterWeightRecordCookie {
            address: voter_weight_record_key,
            account,
        })
    }

    #[allow(dead_code)]
    pub async fn with_max_voter_weight_record(
        &mut self,
        registrar_cookie: &RegistrarCookie,
    ) -> Result<MaxVoterWeightRecordCookie, TransportError> {
        self.with_max_voter_weight_record_using_ix(registrar_cookie, NopOverride)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_max_voter_weight_record_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        registrar_cookie: &RegistrarCookie,
        instruction_override: F,
    ) -> Result<MaxVoterWeightRecordCookie, TransportError> {
        let max_voter_weight_record_key = get_max_voter_weight_record_address(
            &registrar_cookie.account.realm,
            &registrar_cookie.account.governing_token_mint,
        );

        let data = anchor_lang::InstructionData::data(
            &gpl_nft_voter::instruction::CreateMaxVoterWeightRecord {},
        );

        let accounts = gpl_nft_voter::accounts::CreateMaxVoterWeightRecord {
            governance_program_id: self.governance.program_id,
            realm: registrar_cookie.account.realm,
            realm_governing_token_mint: registrar_cookie.account.governing_token_mint,
            max_voter_weight_record: max_voter_weight_record_key,
            payer: self.bench.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        };

        let mut create_max_voter_weight_record_ix = Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        };

        instruction_override(&mut create_max_voter_weight_record_ix);

        self.bench
            .process_transaction(&[create_max_voter_weight_record_ix], None)
            .await?;

        let account = MaxVoterWeightRecord {
            realm: registrar_cookie.account.realm,
            governing_token_mint: registrar_cookie.account.governing_token_mint,
            max_voter_weight: 0,
            max_voter_weight_expiry: Some(0),
            reserved: [0; 8],
        };

        Ok(MaxVoterWeightRecordCookie {
            account,
            address: max_voter_weight_record_key,
        })
    }

    #[allow(dead_code)]
    pub async fn update_voter_weight_record(
        &self,
        registrar_cookie: &RegistrarCookie,
        voter_weight_record_cookie: &mut VoterWeightRecordCookie,
        voter_weight_action: VoterWeightAction,
        nft_cookies: &[&NftCookie],
    ) -> Result<(), TransportError> {
        let data = anchor_lang::InstructionData::data(
            &gpl_nft_voter::instruction::UpdateVoterWeightRecord {
                voter_weight_action,
            },
        );

        let accounts = gpl_nft_voter::accounts::UpdateVoterWeightRecord {
            registrar: registrar_cookie.address,
            voter_weight_record: voter_weight_record_cookie.address,
        };

        let mut account_metas = anchor_lang::ToAccountMetas::to_account_metas(&accounts, None);

        for nft_cookie in nft_cookies {
            account_metas.push(AccountMeta::new_readonly(nft_cookie.address, false));
            account_metas.push(AccountMeta::new_readonly(nft_cookie.metadata, false));
        }

        let instructions = vec![Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: account_metas,
            data,
        }];

        self.bench.process_transaction(&instructions, None).await
    }

    #[allow(dead_code)]
    pub async fn relinquish_nft_vote(
        &mut self,
        registrar_cookie: &RegistrarCookie,
        voter_weight_record_cookie: &VoterWeightRecordCookie,
        proposal_cookie: &ProposalCookie,
        voter_cookie: &WalletCookie,
        voter_token_owner_record_cookie: &TokenOwnerRecordCookie,
        nft_vote_record_cookies: &Vec<NftVoteRecordCookie>,
    ) -> Result<(), TransportError> {
        let data =
            anchor_lang::InstructionData::data(&gpl_nft_voter::instruction::RelinquishNftVote {});

        let vote_record_key = vote_record::get_vote_record_address(
            &self.governance.program_id,
            &proposal_cookie.address,
            &voter_token_owner_record_cookie.address,
        );

        let accounts = gpl_nft_voter::accounts::RelinquishNftVote {
            registrar: registrar_cookie.address,
            voter_weight_record: voter_weight_record_cookie.address,
            governance: proposal_cookie.account.governance,
            proposal: proposal_cookie.address,
            vote_record: vote_record_key,
            beneficiary: self.bench.payer.pubkey(),
            voter_token_owner_record: voter_token_owner_record_cookie.address,
            voter_authority: voter_cookie.address,
        };

        let mut account_metas = anchor_lang::ToAccountMetas::to_account_metas(&accounts, None);

        for nft_vote_record_cookie in nft_vote_record_cookies {
            account_metas.push(AccountMeta::new(nft_vote_record_cookie.address, false));
        }

        let relinquish_nft_vote_ix = Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: account_metas,
            data,
        };

        self.bench
            .process_transaction(&[relinquish_nft_vote_ix], Some(&[&voter_cookie.signer]))
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn with_collection(
        &mut self,
        registrar_cookie: &RegistrarCookie,
        nft_collection_cookie: &NftCollectionCookie,
        max_voter_weight_record_cookie: &MaxVoterWeightRecordCookie,
        args: Option<ConfigureCollectionArgs>,
    ) -> Result<CollectionConfigCookie, TransportError> {
        self.with_collection_using_ix(
            registrar_cookie,
            nft_collection_cookie,
            max_voter_weight_record_cookie,
            args,
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_collection_using_ix<F: Fn(&mut Instruction)>(
        &mut self,
        registrar_cookie: &RegistrarCookie,
        nft_collection_cookie: &NftCollectionCookie,
        max_voter_weight_record_cookie: &MaxVoterWeightRecordCookie,
        args: Option<ConfigureCollectionArgs>,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<CollectionConfigCookie, TransportError> {
        let args = args.unwrap_or_default();

        let data =
            anchor_lang::InstructionData::data(&gpl_nft_voter::instruction::ConfigureCollection {
                weight: args.weight,
                size: args.size,
            });

        let accounts = gpl_nft_voter::accounts::ConfigureCollection {
            registrar: registrar_cookie.address,
            realm: registrar_cookie.account.realm,
            realm_authority: registrar_cookie.realm_authority.pubkey(),
            collection: nft_collection_cookie.mint,
            max_voter_weight_record: max_voter_weight_record_cookie.address,
        };

        let mut configure_collection_ix = Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        };

        instruction_override(&mut configure_collection_ix);

        let default_signers = &[&registrar_cookie.realm_authority];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[configure_collection_ix], Some(signers))
            .await?;

        let collection_config = CollectionConfig {
            collection: nft_collection_cookie.mint,
            size: args.size,
            weight: args.weight,
            reserved: [0; 8],
        };

        Ok(CollectionConfigCookie { collection_config })
    }

    /// Casts NFT Vote and spl-gov Vote
    #[allow(dead_code)]
    pub async fn cast_nft_vote(
        &mut self,
        registrar_cookie: &RegistrarCookie,
        voter_weight_record_cookie: &VoterWeightRecordCookie,
        max_voter_weight_record_cookie: &MaxVoterWeightRecordCookie,
        proposal_cookie: &ProposalCookie,
        nft_voter_cookie: &WalletCookie,
        voter_token_owner_record_cookie: &TokenOwnerRecordCookie,
        nft_cookies: &[&NftCookie],
        args: Option<CastNftVoteArgs>,
    ) -> Result<Vec<NftVoteRecordCookie>, TransportError> {
        let args = args.unwrap_or_default();

        let data = anchor_lang::InstructionData::data(&gpl_nft_voter::instruction::CastNftVote {
            proposal: proposal_cookie.address,
        });

        let accounts = gpl_nft_voter::accounts::CastNftVote {
            registrar: registrar_cookie.address,
            voter_weight_record: voter_weight_record_cookie.address,
            voter_token_owner_record: voter_token_owner_record_cookie.address,
            voter_authority: nft_voter_cookie.address,
            payer: self.bench.payer.pubkey(),
            system_program: solana_sdk::system_program::id(),
        };

        let mut account_metas = anchor_lang::ToAccountMetas::to_account_metas(&accounts, None);
        let mut nft_vote_record_cookies = vec![];

        for nft_cookie in nft_cookies {
            account_metas.push(AccountMeta::new_readonly(nft_cookie.address, false));
            account_metas.push(AccountMeta::new_readonly(nft_cookie.metadata, false));

            let nft_vote_record_key = get_nft_vote_record_address(
                &proposal_cookie.address,
                &nft_cookie.mint_cookie.address,
            );
            account_metas.push(AccountMeta::new(nft_vote_record_key, false));

            let account = NftVoteRecord {
                proposal: proposal_cookie.address,
                nft_mint: nft_cookie.mint_cookie.address,
                governing_token_owner: voter_weight_record_cookie.account.governing_token_owner,
                account_discriminator: NftVoteRecord::ACCOUNT_DISCRIMINATOR,
                reserved: [0; 8],
            };

            nft_vote_record_cookies.push(NftVoteRecordCookie {
                address: nft_vote_record_key,
                account,
            })
        }

        let cast_nft_vote_ix = Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: account_metas,
            data,
        };

        let mut instruction = vec![cast_nft_vote_ix];

        if args.cast_spl_gov_vote {
            // spl-gov cast vote
            let vote = Vote::Approve(vec![VoteChoice {
                rank: 0,
                weight_percentage: 100,
            }]);

            let cast_vote_ix = cast_vote(
                &self.governance.program_id,
                &registrar_cookie.account.realm,
                &proposal_cookie.account.governance,
                &proposal_cookie.address,
                &proposal_cookie.account.token_owner_record,
                &voter_token_owner_record_cookie.address,
                &nft_voter_cookie.address,
                &proposal_cookie.account.governing_token_mint,
                &self.bench.payer.pubkey(),
                Some(voter_weight_record_cookie.address),
                Some(max_voter_weight_record_cookie.address),
                vote,
            );

            instruction.push(cast_vote_ix);
        }

        self.bench
            .process_transaction(&instruction, Some(&[&nft_voter_cookie.signer]))
            .await?;

        Ok(nft_vote_record_cookies)
    }

    #[allow(dead_code)]
    pub async fn get_registrar_account(&mut self, registrar: &Pubkey) -> Registrar {
        self.bench.get_anchor_account::<Registrar>(*registrar).await
    }

    #[allow(dead_code)]
    pub async fn get_nft_vote_record_account(&mut self, nft_vote_record: &Pubkey) -> NftVoteRecord {
        self.bench
            .get_borsh_account::<NftVoteRecord>(nft_vote_record)
            .await
    }

    #[allow(dead_code)]
    pub async fn get_max_voter_weight_record(
        &self,
        max_voter_weight_record: &Pubkey,
    ) -> MaxVoterWeightRecord {
        self.bench
            .get_anchor_account(*max_voter_weight_record)
            .await
    }

    #[allow(dead_code)]
    pub async fn get_voter_weight_record(&self, voter_weight_record: &Pubkey) -> VoterWeightRecord {
        self.bench.get_anchor_account(*voter_weight_record).await
    }

    #[allow(dead_code)]
    pub async fn with_governance_token_holding_account(
        &self,
        registrar_cookie: &RegistrarCookie,
        nft_cookie: &NftCookie,
    ) -> Result<GovernanceTokenHoldingAccountCookie, TransportError> {
        self.with_governance_token_holding_account_using_ix(
            registrar_cookie,
            nft_cookie,
            NopOverride,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_governance_token_holding_account_using_ix<F: Fn(&mut Instruction)>(
        &self,
        registrar_cookie: &RegistrarCookie,
        nft_cookie: &NftCookie,
        instruction_override: F,
    ) -> Result<GovernanceTokenHoldingAccountCookie, TransportError> {
        let holding_account_key = find_nft_power_holding_account_address(
            &registrar_cookie.account.realm,
            &registrar_cookie.account.governing_token_mint,
            &nft_cookie.mint_cookie.address,
        );

        let data = anchor_lang::InstructionData::data(
            &gpl_nft_voter::instruction::CreateGovernanceTokenHoldingAccount {},
        );

        let accounts = gpl_nft_voter::accounts::CreateGovernanceTokenHoldingAccount {
            holding_account_info: holding_account_key,
            governance_program_id: self.governance.program_id,
            registrar: registrar_cookie.address,
            realm_governing_token_mint: registrar_cookie.account.governing_token_mint,
            payer: self.bench.payer.pubkey(),
            nft_mint: nft_cookie.mint_cookie.address,
            nft_metadata: nft_cookie.metadata,
            associated_token_program: anchor_spl::associated_token::ID,
            token_program: spl_token::id(),
            system_program: solana_sdk::system_program::id(),
            rent: Pubkey::from_str("SysvarRent111111111111111111111111111111111")
                .map_err(|e| TransportError::Custom(e.to_string()))?,
        };

        let mut create_governing_token_holding_account_ix = Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        };

        instruction_override(&mut create_governing_token_holding_account_ix);

        self.bench
            .process_transaction(
                &[create_governing_token_holding_account_ix],
                Some(&[&self.bench.payer]),
            )
            .await?;

        let account = self.bench.get_anchor_account(holding_account_key).await;

        Ok(GovernanceTokenHoldingAccountCookie {
            address: holding_account_key,
            account,
        })
    }

    #[allow(dead_code)]
    pub async fn with_nft_voter_token_owner_record(
        &self,
        realm_cookie: &RealmCookie,
        nft_cookie: &NftCookie,
        governing_token_holding_account_cookie: &GovernanceTokenHoldingAccountCookie,
        governing_token_owner_cookie: &WalletCookie,
        governing_token_source_cookie: &TokenAccountCookie,
        deposit_amount: Option<u64>,
    ) -> Result<NftVoterTokenOwnerRecordCookie, TransportError> {
        self.with_nft_voter_token_owner_record_using_ix(
            realm_cookie,
            nft_cookie,
            governing_token_holding_account_cookie,
            governing_token_owner_cookie,
            governing_token_source_cookie,
            deposit_amount,
            NopOverride,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_nft_voter_token_owner_record_using_ix<F: Fn(&mut Instruction)>(
        &self,
        realm_cookie: &RealmCookie,
        nft_cookie: &NftCookie,
        governing_token_holding_account_cookie: &GovernanceTokenHoldingAccountCookie,
        governing_token_owner_cookie: &WalletCookie,
        governing_token_source_cookie: &TokenAccountCookie,
        deposit_amount: Option<u64>,
        instruction_override: F,
    ) -> Result<NftVoterTokenOwnerRecordCookie, TransportError> {
        let token_owner_record_pubkey = DelegatorTokenOwnerRecord::find_address(
            &realm_cookie.address,
            &realm_cookie.account.community_mint,
            &nft_cookie.mint_cookie.address,
            &governing_token_owner_cookie.address,
        );

        let data = anchor_lang::InstructionData::data(
            &gpl_nft_voter::instruction::DepositGovernanceTokens {
                amount: deposit_amount.unwrap_or(0),
            },
        );

        let accounts = gpl_nft_voter::accounts::DepositGovernanceTokens {
            token_owner_record: token_owner_record_pubkey,
            holding_account_info: governing_token_holding_account_cookie.address,
            governance_program_id: self.governance.program_id,
            realm: realm_cookie.address,
            realm_governing_token_mint: realm_cookie.community_mint_cookie.address,
            governing_token_owner: governing_token_owner_cookie.address,
            nft_mint: nft_cookie.mint_cookie.address,
            governing_token_source_account: governing_token_source_cookie.address,
            system_program: solana_sdk::system_program::id(),
            token_program: spl_token::id(),
        };

        let mut deposit_governing_token_ix = Instruction {
            program_id: gpl_nft_voter::id(),
            accounts: anchor_lang::ToAccountMetas::to_account_metas(&accounts, None),
            data,
        };

        instruction_override(&mut deposit_governing_token_ix);

        self.bench
            .process_transaction(
                &[deposit_governing_token_ix],
                Some(&[&governing_token_owner_cookie.signer]),
            )
            .await?;

        let account = self
            .bench
            .get_anchor_account(token_owner_record_pubkey)
            .await;

        Ok(NftVoterTokenOwnerRecordCookie {
            address: token_owner_record_pubkey,
            account,
        })
    }
}
