use anchor_lang::{
    prelude::{Pubkey, Rent},
    AccountDeserialize,
};
use std::cell::RefCell;
use std::convert::TryInto;

use anchor_spl::associated_token;
use borsh::BorshDeserialize;
use solana_program::system_program;
use solana_program_test::{BanksClientError, ProgramTest, ProgramTestContext};
use solana_sdk::{
    account::{Account, ReadableAccount},
    borsh1::try_from_slice_unchecked,
    instruction::{AccountMeta, Instruction},
    program_option::COption,
    program_pack::Pack,
    signature::Keypair,
    signer::Signer,
    system_instruction, sysvar,
    transaction::Transaction,
    transport::TransportError,
};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};
use spl_token_2022::extension::ExtensionType;
use spl_token_client::token::ExtensionInitializationParams;
use spl_transfer_hook_interface::{
    get_extra_account_metas_address,
    instruction::{initialize_extra_account_meta_list, update_extra_account_meta_list},
};

use crate::program_test::tools::clone_keypair;

use super::token_voter_test::UserCookie;

use spl_token_2022::extension::transfer_fee::{TransferFee, TransferFeeConfig};

pub struct TransferFeeConfigWithKeypairs {
    pub transfer_fee_config: TransferFeeConfig,
    pub transfer_fee_config_authority: Keypair,
    pub withdraw_withheld_authority: Keypair,
}

const TEST_MAXIMUM_FEE: u64 = 10_000_000;
const TEST_FEE_BASIS_POINTS: u16 = 250;

fn test_transfer_fee() -> TransferFee {
    TransferFee {
        epoch: 0.into(),
        transfer_fee_basis_points: TEST_FEE_BASIS_POINTS.into(),
        maximum_fee: TEST_MAXIMUM_FEE.into(),
    }
}

pub fn test_transfer_fee_config_with_keypairs() -> TransferFeeConfigWithKeypairs {
    let transfer_fee = test_transfer_fee();
    let transfer_fee_config_authority = Keypair::new();
    let withdraw_withheld_authority = Keypair::new();
    let transfer_fee_config = TransferFeeConfig {
        transfer_fee_config_authority: COption::Some(transfer_fee_config_authority.pubkey())
            .try_into()
            .unwrap(),
        withdraw_withheld_authority: COption::Some(withdraw_withheld_authority.pubkey())
            .try_into()
            .unwrap(),
        withheld_amount: 0.into(),
        older_transfer_fee: transfer_fee,
        newer_transfer_fee: transfer_fee,
    };
    TransferFeeConfigWithKeypairs {
        transfer_fee_config,
        transfer_fee_config_authority,
        withdraw_withheld_authority,
    }
}

#[derive(Debug, PartialEq)]
pub struct MintCookie {
    pub address: Pubkey,
    pub mint_authority: Keypair,
    pub freeze_authority: Option<Keypair>,
    pub index: usize,
    pub decimals: u8,
    pub unit: f64,
    pub base_lot: f64,
    pub quote_lot: f64,
    pub is_token_2022: bool,
}

impl Clone for MintCookie {
    fn clone(&self) -> Self {
        let freeze_authority = if let Some(freeze_authority) = &self.freeze_authority {
            Some(clone_keypair(&freeze_authority))
        } else {
            None
        };

        Self {
            mint_authority: clone_keypair(&self.mint_authority),
            freeze_authority,
            index: self.index,
            decimals: self.decimals,
            unit: self.unit,
            base_lot: self.base_lot,
            quote_lot: self.quote_lot,
            address: self.address.clone(),
            is_token_2022: self.is_token_2022,
        }
    }
}
pub struct TokenAccountCookie {
    pub address: Pubkey,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct WalletCookie {
    pub address: Pubkey,
    pub account: Account,
    pub signer: Keypair,
}

trait AddPacked {
    fn add_packable_account<T: Pack>(
        &mut self,
        pubkey: Pubkey,
        amount: u64,
        data: &T,
        owner: &Pubkey,
    );
}

impl AddPacked for ProgramTest {
    fn add_packable_account<T: Pack>(
        &mut self,
        pubkey: Pubkey,
        amount: u64,
        data: &T,
        owner: &Pubkey,
    ) {
        let mut account = solana_sdk::account::Account::new(amount, T::get_packed_len(), owner);
        data.pack_into_slice(&mut account.data);
        self.add_account(pubkey, account);
    }
}

pub struct ProgramTestBench {
    pub context: RefCell<ProgramTestContext>,
    pub payer: Keypair,
    pub rent: Rent,
}

#[allow(dead_code)]
pub enum MintType {
    SplToken,
    SplTokenExtensions,
    SplTokenExtensionsWithTransferFees,
    SplTokenExtensionsWithTransferHook,
}

impl ProgramTestBench {
    /// Create new bench given a ProgramTest instance populated with all of the
    /// desired programs.
    pub async fn start_new(program_test: ProgramTest) -> Self {
        let mut context = program_test.start_with_context().await;

        let payer = clone_keypair(&context.payer);

        let rent = context.banks_client.get_rent().await.unwrap();

        Self {
            payer,
            context: RefCell::new(context),
            rent,
        }
    }

    #[allow(dead_code)]
    pub fn add_mints_and_user_cookies_spl_token(
        program_test: &mut ProgramTest,
        mint_type: MintType,
    ) -> (Vec<MintCookie>, Vec<UserCookie>) {
        let (token_program_id, is_token_2022) = match mint_type {
            MintType::SplToken => (spl_token::id(), false),
            _ => (spl_token_2022::id(), true),
        };
        // Mints
        let mints: Vec<MintCookie> = vec![
            MintCookie {
                index: 0,
                address: Pubkey::new_unique(),
                decimals: 6,
                unit: 10u64.pow(6) as f64,
                base_lot: 100 as f64,
                quote_lot: 10 as f64,
                mint_authority: Keypair::new(),
                freeze_authority: None,
                is_token_2022,
            }, // symbol: "MNGO".to_string()
            MintCookie {
                index: 1,
                address: Pubkey::new_unique(),
                decimals: 6,
                unit: 10u64.pow(6) as f64,
                base_lot: 0 as f64,
                quote_lot: 0 as f64,
                mint_authority: Keypair::new(),
                freeze_authority: None,
                is_token_2022,
            }, // symbol: "USDC".to_string()
        ];
        // Add mints in loop
        for mint_index in 0..mints.len() {
            let mint_pk: Pubkey;
            mint_pk = mints[mint_index].address;

            program_test.add_packable_account(
                mint_pk,
                u32::MAX as u64,
                &spl_token_2022::state::Mint {
                    is_initialized: true,
                    mint_authority: COption::Some(mints[mint_index].mint_authority.pubkey()),
                    decimals: mints[mint_index].decimals,
                    supply: 100,
                    ..spl_token_2022::state::Mint::default()
                },
                &token_program_id,
            );
        }

        // Users
        let num_users = 4;
        let mut users = Vec::new();
        for _ in 0..num_users {
            let user_key = Keypair::new();
            program_test.add_account(
                user_key.pubkey(),
                solana_sdk::account::Account::new(
                    u32::MAX as u64,
                    0,
                    &solana_sdk::system_program::id(),
                ),
            );

            // give every user 10^18 (< 2^60) of every token
            // ~~ 1 trillion in case of 6 decimals
            let mut token_accounts = Vec::new();
            for mint_index in 0..mints.len() {
                let token_key = get_associated_token_address_with_program_id(
                    &user_key.pubkey(),
                    &mints[mint_index].address,
                    &token_program_id,
                );
                program_test.add_packable_account(
                    token_key,
                    u32::MAX as u64,
                    &spl_token_2022::state::Account {
                        mint: mints[mint_index].address,
                        owner: user_key.pubkey(),
                        amount: 10,
                        state: spl_token_2022::state::AccountState::Initialized,
                        ..spl_token_2022::state::Account::default()
                    },
                    &token_program_id,
                );

                token_accounts.push(token_key);
            }
            users.push(UserCookie {
                key: user_key,
                token_accounts,
            });
        }

        (mints, users)
    }

    #[allow(dead_code)]
    pub async fn process_transaction(
        &self,
        instructions: &[Instruction],
        signers: Option<&[&Keypair]>,
    ) -> Result<(), BanksClientError> {
        let mut context = self.context.borrow_mut();

        let mut transaction =
            Transaction::new_with_payer(&instructions, Some(&context.payer.pubkey()));

        let mut all_signers = vec![&context.payer];

        if let Some(signers) = signers {
            all_signers.extend_from_slice(signers);
        }

        transaction.sign(&all_signers, context.last_blockhash);

        context
            .banks_client
            .process_transaction_with_commitment(
                transaction,
                solana_sdk::commitment_config::CommitmentLevel::Processed,
            )
            .await
    }

    pub async fn get_clock(&self) -> solana_program::clock::Clock {
        self.context
            .borrow_mut()
            .banks_client
            .get_sysvar::<solana_program::clock::Clock>()
            .await
            .unwrap()
    }

    #[allow(dead_code)]
    pub async fn advance_clock(&self) {
        let clock = self.get_clock().await;
        self.context
            .borrow_mut()
            .warp_to_slot(clock.slot + 2)
            .unwrap();
    }

    pub async fn with_mint(
        &self,
        mint_type: &MintType,
        transfer_hook_program_id: Option<&Pubkey>,
    ) -> Result<MintCookie, TransportError> {
        let mint_keypair = Keypair::new();
        let mint_authority = Keypair::new();
        let freeze_authority = Keypair::new();

        match mint_type {
            MintType::SplToken => {
                self.create_mint(&mint_keypair, &mint_authority.pubkey(), None)
                    .await?;
            }
            MintType::SplTokenExtensions => {
                self.create_mint_token_extension(&mint_keypair, &mint_authority.pubkey(), None)
                    .await?;
            }
            MintType::SplTokenExtensionsWithTransferFees => {
                self.create_mint_token_extension_with_transfer_fees(
                    &mint_keypair,
                    &mint_authority.pubkey(),
                    None,
                )
                .await?;
            }
            MintType::SplTokenExtensionsWithTransferHook => {
                self.create_mint_token_extension_with_transfer_hook(
                    &mint_keypair,
                    &mint_authority.pubkey(),
                    None,
                    transfer_hook_program_id.unwrap(),
                )
                .await?;
            }
        }

        Ok(MintCookie {
            address: mint_keypair.pubkey(),
            mint_authority,
            freeze_authority: Some(freeze_authority),
            index: 1,
            decimals: 6,
            unit: 10u64.pow(6) as f64,
            base_lot: 0 as f64,
            quote_lot: 0 as f64,
            is_token_2022: false,
        })
    }

    #[allow(dead_code)]
    pub async fn create_mint(
        &self,
        mint_keypair: &Keypair,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    ) -> Result<(), BanksClientError> {
        let mint_rent = self.rent.minimum_balance(spl_token::state::Mint::LEN);

        let instructions = [
            system_instruction::create_account(
                &self.context.borrow().payer.pubkey(),
                &mint_keypair.pubkey(),
                mint_rent,
                spl_token::state::Mint::LEN as u64,
                &spl_token::id(),
            ),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &mint_keypair.pubkey(),
                mint_authority,
                freeze_authority,
                0,
            )
            .unwrap(),
        ];

        self.process_transaction(&instructions, Some(&[mint_keypair]))
            .await
    }

    #[allow(dead_code)]
    pub async fn create_mint_token_extension(
        &self,
        mint_keypair: &Keypair,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    ) -> Result<(), BanksClientError> {
        let mint_rent = self.rent.minimum_balance(spl_token_2022::state::Mint::LEN);

        let instructions = [
            system_instruction::create_account(
                &self.context.borrow().payer.pubkey(),
                &mint_keypair.pubkey(),
                mint_rent,
                spl_token_2022::state::Mint::LEN as u64,
                &spl_token_2022::id(),
            ),
            spl_token_2022::instruction::initialize_mint(
                &spl_token_2022::id(),
                &mint_keypair.pubkey(),
                mint_authority,
                freeze_authority,
                0,
            )
            .unwrap(),
        ];

        self.process_transaction(&instructions, Some(&[mint_keypair]))
            .await
    }

    #[allow(dead_code)]
    pub async fn create_mint_token_extension_with_transfer_fees(
        &self,
        mint_keypair: &Keypair,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    ) -> Result<(), BanksClientError> {
        let TransferFeeConfigWithKeypairs {
            transfer_fee_config_authority,
            withdraw_withheld_authority,
            transfer_fee_config,
            ..
        } = test_transfer_fee_config_with_keypairs();
        let transfer_fee_basis_points = u16::from(
            transfer_fee_config
                .newer_transfer_fee
                .transfer_fee_basis_points,
        );
        let maximum_fee = u64::from(transfer_fee_config.newer_transfer_fee.maximum_fee);
        let extension_initialization_params =
            vec![ExtensionInitializationParams::TransferFeeConfig {
                transfer_fee_config_authority: transfer_fee_config_authority.pubkey().into(),
                withdraw_withheld_authority: withdraw_withheld_authority.pubkey().into(),
                transfer_fee_basis_points,
                maximum_fee,
            }];
        let extension_types = extension_initialization_params
            .iter()
            .map(|e| e.extension())
            .collect::<Vec<_>>();
        let space = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(
            &extension_types,
        )
        .unwrap();

        let mint_rent = self.rent.minimum_balance(space);

        let mut instructions = vec![system_instruction::create_account(
            &self.context.borrow().payer.pubkey(),
            &mint_keypair.pubkey(),
            mint_rent,
            space as u64,
            &spl_token_2022::id(),
        )];

        for params in extension_initialization_params {
            instructions.push(
                params
                    .instruction(&spl_token_2022::id(), &mint_keypair.pubkey())
                    .unwrap(),
            );
        }

        instructions.push(
            spl_token_2022::instruction::initialize_mint(
                &spl_token_2022::id(),
                &mint_keypair.pubkey(),
                mint_authority,
                freeze_authority,
                0,
            )
            .unwrap(),
        );

        self.process_transaction(&instructions, Some(&[mint_keypair]))
            .await
    }

    #[allow(dead_code)]
    pub async fn initialize_transfer_hook_account_metas(
        &self,
        mint_address: &Pubkey,
        mint_authority: &Keypair,
        payer: &Keypair,
        program_id: &Pubkey,
        source: &Pubkey,
        destination: &Pubkey,
        writable_pubkey: &Pubkey,
        amount: u64,
    ) -> Vec<AccountMeta> {
        let extra_account_metas_address =
            get_extra_account_metas_address(&mint_address, &program_id);

        let init_extra_account_metas = [
            ExtraAccountMeta::new_with_pubkey(&sysvar::instructions::id(), false, false).unwrap(),
            ExtraAccountMeta::new_with_pubkey(&mint_authority.pubkey(), false, false).unwrap(),
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal {
                        bytes: b"seed-prefix".to_vec(),
                    },
                    Seed::AccountKey { index: 0 },
                ],
                false,
                true,
            )
            .unwrap(),
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::InstructionData {
                        index: 8,  // After instruction discriminator
                        length: 8, // `u64` (amount)
                    },
                    Seed::AccountKey { index: 2 },
                ],
                false,
                true,
            )
            .unwrap(),
            ExtraAccountMeta::new_with_pubkey(&writable_pubkey, false, true).unwrap(),
        ];

        let extra_pda_1 = Pubkey::find_program_address(
            &[
                b"seed-prefix",  // Literal prefix
                source.as_ref(), // Account at index 0
            ],
            &program_id,
        )
        .0;
        let extra_pda_2 = Pubkey::find_program_address(
            &[
                &amount.to_le_bytes(), // Instruction data bytes 8 to 16
                destination.as_ref(),  // Account at index 2
            ],
            &program_id,
        )
        .0;

        let extra_account_metas = [
            AccountMeta::new(extra_account_metas_address, false),
            AccountMeta::new(*program_id, false),
            AccountMeta::new_readonly(sysvar::instructions::id(), false),
            AccountMeta::new_readonly(mint_authority.pubkey(), false),
            AccountMeta::new(extra_pda_1, false),
            AccountMeta::new(extra_pda_2, false),
            AccountMeta::new(*writable_pubkey, false),
        ];

        let rent_lamports = self.rent.minimum_balance(
            ExtraAccountMetaList::size_of(init_extra_account_metas.len()).unwrap(),
        );

        let instructions = &[
            system_instruction::transfer(
                &payer.pubkey(),
                &extra_account_metas_address,
                rent_lamports,
            ),
            initialize_extra_account_meta_list(
                &program_id,
                &extra_account_metas_address,
                &mint_address,
                &mint_authority.pubkey(),
                &init_extra_account_metas,
            ),
        ];
        self.process_transaction(instructions, Some(&[&payer, &mint_authority]))
            .await
            .unwrap();

        extra_account_metas.to_vec()
    }

    #[allow(dead_code)]
    pub async fn update_transfer_hook_account_metas(
        &self,
        mint_address: &Pubkey,
        mint_authority: &Keypair,
        payer: &Keypair,
        program_id: &Pubkey,
        source: &Pubkey,
        destination: &Pubkey,
        updated_writable_pubkey: &Pubkey,
        amount: u64,
    ) -> Vec<AccountMeta> {
        let extra_account_metas_address =
            get_extra_account_metas_address(&mint_address, &program_id);

        let updated_extra_account_metas = [
            ExtraAccountMeta::new_with_pubkey(&sysvar::instructions::id(), false, false).unwrap(),
            ExtraAccountMeta::new_with_pubkey(&mint_authority.pubkey(), false, false).unwrap(),
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::Literal {
                        bytes: b"updated-seed-prefix".to_vec(),
                    },
                    Seed::AccountKey { index: 0 },
                ],
                false,
                true,
            )
            .unwrap(),
            ExtraAccountMeta::new_with_seeds(
                &[
                    Seed::InstructionData {
                        index: 8,  // After instruction discriminator
                        length: 8, // `u64` (amount)
                    },
                    Seed::AccountKey { index: 2 },
                ],
                false,
                true,
            )
            .unwrap(),
            ExtraAccountMeta::new_with_pubkey(&updated_writable_pubkey, false, true).unwrap(),
        ];

        let extra_pda_1 = Pubkey::find_program_address(
            &[
                b"updated-seed-prefix", // Literal prefix
                source.as_ref(),        // Account at index 0
            ],
            &program_id,
        )
        .0;
        let extra_pda_2 = Pubkey::find_program_address(
            &[
                &amount.to_le_bytes(), // Instruction data bytes 8 to 16
                destination.as_ref(),  // Account at index 2
            ],
            &program_id,
        )
        .0;

        let extra_account_metas = [
            AccountMeta::new(extra_account_metas_address, false),
            AccountMeta::new(*program_id, false),
            AccountMeta::new_readonly(sysvar::instructions::id(), false),
            AccountMeta::new_readonly(mint_authority.pubkey(), false),
            AccountMeta::new(extra_pda_1, false),
            AccountMeta::new(extra_pda_2, false),
            AccountMeta::new(*updated_writable_pubkey, false),
        ];

        let rent_lamports = self.rent.minimum_balance(
            ExtraAccountMetaList::size_of(updated_extra_account_metas.len()).unwrap(),
        );

        let instructions = &[
            system_instruction::transfer(
                &payer.pubkey(),
                &extra_account_metas_address,
                rent_lamports,
            ),
            update_extra_account_meta_list(
                &program_id,
                &extra_account_metas_address,
                &mint_address,
                &mint_authority.pubkey(),
                &updated_extra_account_metas,
            ),
        ];

        self.process_transaction(instructions, Some(&[&payer, &mint_authority]))
            .await
            .unwrap();

        extra_account_metas.to_vec()
    }

    #[allow(dead_code)]
    pub async fn create_mint_token_extension_with_transfer_hook(
        &self,
        mint_keypair: &Keypair,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
        program_id: &Pubkey,
    ) -> Result<(), BanksClientError> {
        let extension_initialization_params = vec![ExtensionInitializationParams::TransferHook {
            authority: Some(*mint_authority),
            program_id: Some(*program_id),
        }];

        let extension_types = extension_initialization_params
            .iter()
            .map(|e| e.extension())
            .collect::<Vec<_>>();
        let space = ExtensionType::try_calculate_account_len::<spl_token_2022::state::Mint>(
            &extension_types,
        )
        .unwrap();
        let mint_rent = self.rent.minimum_balance(space);

        let mut instructions = vec![system_instruction::create_account(
            &self.context.borrow().payer.pubkey(),
            &mint_keypair.pubkey(),
            mint_rent,
            space as u64,
            &spl_token_2022::id(),
        )];

        for params in extension_initialization_params {
            instructions.push(
                params
                    .instruction(&spl_token_2022::id(), &mint_keypair.pubkey())
                    .unwrap(),
            );
        }

        instructions.push(
            spl_token_2022::instruction::initialize_mint(
                &spl_token_2022::id(),
                &mint_keypair.pubkey(),
                mint_authority,
                freeze_authority,
                0,
            )
            .unwrap(),
        );

        self.process_transaction(&instructions, Some(&[mint_keypair]))
            .await
    }

    #[allow(dead_code)]
    pub async fn with_token_account(
        &self,
        token_mint: &Pubkey,
        mint_type: &MintType,
        is_token_account: bool,
    ) -> Result<TokenAccountCookie, TransportError> {
        let token_account_keypair = Keypair::new();

        self.create_token_account(
            &token_account_keypair,
            token_mint,
            &self.payer.pubkey(),
            mint_type,
            is_token_account,
        )
        .await?;
        Ok(TokenAccountCookie {
            address: token_account_keypair.pubkey(),
        })
    }

    #[allow(dead_code)]
    pub async fn with_tokens(
        &self,
        mint_cookie: &MintCookie,
        owner: &Pubkey,
        amount: u64,
        mint_type: &MintType,
        is_token_account: bool,
    ) -> Result<TokenAccountCookie, TransportError> {
        let token_account_keypair = Keypair::new();

        self.create_token_account(
            &token_account_keypair,
            &mint_cookie.address,
            owner,
            mint_type,
            is_token_account,
        )
        .await?;

        self.mint_tokens(
            &mint_cookie.address,
            &mint_cookie.mint_authority,
            &token_account_keypair.pubkey(),
            amount,
            mint_type,
            owner,
            is_token_account,
        )
        .await?;
        Ok(TokenAccountCookie {
            address: token_account_keypair.pubkey(),
        })
    }

    pub async fn mint_tokens(
        &self,
        token_mint: &Pubkey,
        token_mint_authority: &Keypair,
        token_account: &Pubkey,
        amount: u64,
        mint_type: &MintType,
        owner: &Pubkey,
        is_token_account: bool,
    ) -> Result<(), BanksClientError> {
        match mint_type {
            MintType::SplToken => {
                let mint_instruction = spl_token_2022::instruction::mint_to(
                    &spl_token::id(),
                    token_mint,
                    token_account,
                    &token_mint_authority.pubkey(),
                    &[],
                    amount,
                )
                .unwrap();

                self.process_transaction(&[mint_instruction], Some(&[token_mint_authority]))
                    .await
            }
            _ => {
                let mint_instruction = if is_token_account {
                    spl_token_2022::instruction::mint_to(
                        &spl_token_2022::id(),
                        token_mint,
                        token_account,
                        &token_mint_authority.pubkey(),
                        &[],
                        amount,
                    )
                    .unwrap()
                } else {
                    spl_token_2022::instruction::mint_to(
                        &spl_token_2022::id(),
                        token_mint,
                        &associated_token::get_associated_token_address_with_program_id(
                            &owner,
                            &token_mint,
                            &spl_token_2022::id(),
                        ),
                        &token_mint_authority.pubkey(),
                        &[],
                        amount,
                    )
                    .unwrap()
                };

                self.process_transaction(&[mint_instruction], Some(&[token_mint_authority]))
                    .await
            }
        }
    }

    #[allow(dead_code)]
    pub async fn create_token_account(
        &self,
        token_account_keypair: &Keypair,
        token_mint: &Pubkey,
        owner: &Pubkey,
        mint_type: &MintType,
        is_token_account: bool,
    ) -> Result<(), BanksClientError> {
        match mint_type {
            MintType::SplToken => {
                let create_account_instruction = system_instruction::create_account(
                    &self.context.borrow().payer.pubkey(),
                    &token_account_keypair.pubkey(),
                    self.rent.minimum_balance(spl_token::state::Account::LEN),
                    spl_token::state::Account::LEN as u64,
                    &spl_token::id(),
                );

                let initialize_account_instruction = spl_token::instruction::initialize_account(
                    &spl_token::id(),
                    &token_account_keypair.pubkey(),
                    token_mint,
                    owner,
                )
                .unwrap();

                self.process_transaction(
                    &[create_account_instruction, initialize_account_instruction],
                    Some(&[token_account_keypair]),
                )
                .await
            }
            MintType::SplTokenExtensionsWithTransferFees => {
                let extension_type_space = ExtensionType::try_calculate_account_len::<
                    spl_token_2022::state::Account,
                >(&[ExtensionType::TransferFeeConfig])
                .unwrap();
                if is_token_account {
                    let create_account_instruction = system_instruction::create_account(
                        &self.context.borrow().payer.pubkey(),
                        &token_account_keypair.pubkey(),
                        self.rent.minimum_balance(extension_type_space),
                        extension_type_space as u64,
                        &spl_token_2022::id(),
                    );

                    let initialize_account_instruction =
                        spl_token_2022::instruction::initialize_account(
                            &spl_token_2022::id(),
                            &token_account_keypair.pubkey(),
                            token_mint,
                            owner,
                        )
                        .unwrap();
                    self.process_transaction(
                        &[create_account_instruction, initialize_account_instruction],
                        Some(&[token_account_keypair]),
                    )
                    .await
                } else {
                    let create_ata_account =
                        spl_associated_token_account::instruction::create_associated_token_account(
                            &self.context.borrow().payer.pubkey(),
                            owner,
                            token_mint,
                            &spl_token_2022::id(),
                        );
                    self.process_transaction(&[create_ata_account], None).await
                }
            }
            _ => {
                if is_token_account {
                    let create_account_instruction = system_instruction::create_account(
                        &self.context.borrow().payer.pubkey(),
                        &token_account_keypair.pubkey(),
                        self.rent
                            .minimum_balance(spl_token_2022::state::Account::get_packed_len()),
                        spl_token_2022::state::Account::get_packed_len() as u64,
                        &spl_token_2022::id(),
                    );

                    let initialize_account_instruction =
                        spl_token_2022::instruction::initialize_account(
                            &spl_token_2022::id(),
                            &token_account_keypair.pubkey(),
                            token_mint,
                            owner,
                        )
                        .unwrap();

                    self.process_transaction(
                        &[create_account_instruction, initialize_account_instruction],
                        Some(&[token_account_keypair]),
                    )
                    .await
                } else {
                    let create_ata_account =
                        spl_associated_token_account::instruction::create_associated_token_account(
                            &self.context.borrow().payer.pubkey(),
                            owner,
                            token_mint,
                            &spl_token_2022::id(),
                        );
                    self.process_transaction(&[create_ata_account], None).await
                }
            }
        }
    }

    #[allow(dead_code)]
    pub async fn create_token_extensions_account(
        &self,
        token_account_keypair: &Keypair,
        token_mint: &Pubkey,
        owner: &Pubkey,
    ) -> Result<(), BanksClientError> {
        let rent = self
            .context
            .borrow_mut()
            .banks_client
            .get_rent()
            .await
            .unwrap();

        let create_account_instruction = system_instruction::create_account(
            &self.context.borrow().payer.pubkey(),
            &token_account_keypair.pubkey(),
            rent.minimum_balance(spl_token_2022::state::Account::get_packed_len()),
            spl_token_2022::state::Account::get_packed_len() as u64,
            &spl_token_2022::id(),
        );

        let initialize_account_instruction = spl_token_2022::instruction::initialize_account(
            &spl_token_2022::id(),
            &token_account_keypair.pubkey(),
            token_mint,
            owner,
        )
        .unwrap();

        self.process_transaction(
            &[create_account_instruction, initialize_account_instruction],
            Some(&[token_account_keypair]),
        )
        .await
    }
    #[allow(dead_code)]
    pub async fn with_wallet(&self) -> WalletCookie {
        let account_rent = self.rent.minimum_balance(0);
        let account_keypair = Keypair::new();

        let create_account_ix = system_instruction::create_account(
            &self.context.borrow().payer.pubkey(),
            &account_keypair.pubkey(),
            account_rent,
            0,
            &system_program::id(),
        );

        self.process_transaction(&[create_account_ix], Some(&[&account_keypair]))
            .await
            .unwrap();

        let account = Account {
            lamports: account_rent,
            data: vec![],
            owner: system_program::id(),
            executable: false,
            rent_epoch: 0,
        };

        WalletCookie {
            address: account_keypair.pubkey(),
            account,
            signer: account_keypair,
        }
    }

    #[allow(dead_code)]
    pub async fn get_account(&self, address: &Pubkey) -> Option<Account> {
        self.context
            .borrow_mut()
            .banks_client
            .get_account(*address)
            .await
            .unwrap()
    }

    #[allow(dead_code)]
    pub async fn get_borsh_account<
        T: BorshDeserialize
            + anchor_spl::token_2022_extensions::spl_token_metadata_interface::borsh::BorshDeserialize,
    >(
        &self,
        address: &Pubkey,
    ) -> T {
        #[allow(deprecated)]
        self.get_account(address)
            .await
            .map(|a| try_from_slice_unchecked(&a.data).unwrap())
            .unwrap_or_else(|| panic!("GET-TEST-ACCOUNT-ERROR: Account {} not found", address))
    }

    #[allow(dead_code)]
    pub async fn get_account_data(&self, address: Pubkey) -> Vec<u8> {
        self.context
            .borrow_mut()
            .banks_client
            .get_account(address)
            .await
            .unwrap()
            .unwrap()
            .data()
            .to_vec()
    }

    #[allow(dead_code)]
    pub async fn advance_clock_by_slots(&self, slots: u64) {
        let clock = self.get_clock().await;
        self.context
            .borrow_mut()
            .warp_to_slot(clock.slot + slots)
            .unwrap();
    }

    #[allow(dead_code)]
    pub async fn get_anchor_account<T: AccountDeserialize>(&self, address: Pubkey) -> T {
        let data = self.get_account_data(address).await;
        let mut data_slice: &[u8] = &data;
        AccountDeserialize::try_deserialize(&mut data_slice).unwrap()
    }
}
