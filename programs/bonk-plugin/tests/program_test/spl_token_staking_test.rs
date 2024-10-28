use std::u64;
use std::{str::FromStr, sync::Arc};

use anchor_lang::prelude::AccountMeta;
use anchor_lang::ToAccountMetas;
use anchor_lang::{prelude::Pubkey, system_program, InstructionData};
use anchor_spl::associated_token::get_associated_token_address;
use anchor_spl::associated_token::spl_associated_token_account::instruction::create_associated_token_account;
use solana_program_test::ProgramTest;
use solana_sdk::{
    instruction::Instruction, signature::Keypair, signer::Signer, sysvar::rent,
    transport::TransportError,
};

use anchor_lang::declare_program;

use super::program_test_bench::ProgramTestBench;

declare_program!(spl_token_staking);

pub struct SplTokenStakingCookie {
    pub program_id: Pubkey,
    pub bench: Arc<ProgramTestBench>,
}

impl SplTokenStakingCookie {
    pub fn program_id() -> Pubkey {
        Pubkey::from_str("STAKEkKzbdeKkqzKpLkNQD3SUuLgshDKCD7U8duxAbB").unwrap()
    }

    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("spl_token_staking", Self::program_id(), None);
    }

    #[allow(dead_code)]
    pub fn new(bench: Arc<ProgramTestBench>) -> Self {
        SplTokenStakingCookie {
            bench,
            program_id: Self::program_id(),
        }
    }

    #[allow(dead_code)]
    pub async fn with_stake_pool(
        &mut self,
        community_token_mint: &Pubkey,
    ) -> Result<Pubkey, TransportError> {
        let create_stake_pool_args = spl_token_staking::client::args::InitializeStakePool {
            nonce: 0,
            max_duration: 1000,
            max_weight: u64::MAX,
            min_duration: 1,
        };
        let stake_pool_key = find_stake_pool_key(
            0,
            *community_token_mint,
            self.bench.payer.pubkey(),
            self.program_id,
        );

        let create_stake_pool_accounts = spl_token_staking::client::accounts::InitializeStakePool {
            payer: self.bench.payer.pubkey(),
            stake_pool: stake_pool_key,
            authority: self.bench.payer.pubkey(),
            mint: *community_token_mint,
            stake_mint: find_stake_mint(stake_pool_key, self.program_id),
            vault: find_vault_key(stake_pool_key, self.program_id),
            token_program: spl_token::id(),
            rent: rent::id(),
            system_program: system_program::ID,
        };
        let initialize_stake_pool_ix = Instruction {
            program_id: self.program_id,
            accounts: create_stake_pool_accounts.to_account_metas(None),
            data: create_stake_pool_args.data(),
        };

        let add_reward_pool_args = spl_token_staking::client::args::AddRewardPool { index: 0 };

        let add_reward_pool_accounts = spl_token_staking::client::accounts::AddRewardPool {
            payer: self.bench.payer.pubkey(),
            stake_pool: stake_pool_key,
            authority: self.bench.payer.pubkey(),
            reward_mint: *community_token_mint,
            reward_vault: find_reward_vault_key(
                stake_pool_key,
                *community_token_mint,
                self.program_id,
            ),
            token_program: spl_token::id(),
            rent: rent::id(),
            system_program: system_program::ID,
        };
        let add_reward_pool_ix = Instruction {
            program_id: self.program_id,
            accounts: add_reward_pool_accounts.to_account_metas(None),
            data: add_reward_pool_args.data(),
        };
        self.bench
            .process_transaction(&[initialize_stake_pool_ix, add_reward_pool_ix], None)
            .await?;

        Ok(stake_pool_key)
    }

    #[allow(dead_code)]
    pub async fn deposit_into_stake_pool(
        &mut self,
        owner: &Keypair,
        stake_pool_key: &Pubkey,
        stake_deposit_receipt: &Pubkey,
        mint_to_be_staked_account: &Pubkey,
        reward_vault_accounts: &[&Pubkey],
    ) -> Result<(), TransportError> {
        let deposit_to_pool_args = spl_token_staking::client::args::Deposit {
            nonce: 0,
            amount: 100,
            lockup_duration: 1000,
        };
        let stake_mint = find_stake_mint(*stake_pool_key, self.program_id);
        let stake_mint_account_key = get_associated_token_address(&owner.pubkey(), &stake_mint);

        let deposit_to_pool_accounts = spl_token_staking::client::accounts::Deposit {
            payer: owner.pubkey(),
            owner: owner.pubkey(),
            stake_pool: *stake_pool_key,
            from: *mint_to_be_staked_account,
            stake_mint,
            destination: stake_mint_account_key,
            vault: find_vault_key(*stake_pool_key, self.program_id),
            stake_deposit_receipt: *stake_deposit_receipt,
            token_program: spl_token::id(),
            rent: rent::id(),
            system_program: system_program::ID,
        };
        let mut account_metas =
            anchor_lang::ToAccountMetas::to_account_metas(&deposit_to_pool_accounts, None);

        for reward_vault_account in reward_vault_accounts {
            account_metas.push(AccountMeta::new_readonly(**reward_vault_account, false));
        }
        let deposit_to_pool_ix = Instruction {
            program_id: self.program_id,
            accounts: account_metas,
            data: deposit_to_pool_args.data(),
        };
        let create_stake_mint_ata_ix: Instruction = create_associated_token_account(
            &self.bench.payer.pubkey(),
            &owner.pubkey(),
            &stake_mint,
            &spl_token::id(),
        );
        self.bench
            .process_transaction(
                &[create_stake_mint_ata_ix, deposit_to_pool_ix],
                Some(&[&owner]),
            )
            .await?;

        Ok(())
    }
}

pub fn find_stake_pool_key(
    stake_pool_nonce: u8,
    stake_pool_mint: Pubkey,
    provider_pubkey: Pubkey,
    program_id: Pubkey,
) -> Pubkey {
    let seeds = &[
        &[stake_pool_nonce],
        stake_pool_mint.as_ref(),
        provider_pubkey.as_ref(),
        b"stakePool",
    ];
    Pubkey::find_program_address(seeds, &program_id).0
}

pub fn find_vault_key(stake_pool_key: Pubkey, program_id: Pubkey) -> Pubkey {
    let seeds = &[stake_pool_key.as_ref(), b"vault"];
    Pubkey::find_program_address(seeds, &program_id).0
}

pub fn find_stake_mint(stake_pool_key: Pubkey, program_id: Pubkey) -> Pubkey {
    let seeds = &[stake_pool_key.as_ref(), b"stakeMint"];
    Pubkey::find_program_address(seeds, &program_id).0
}

#[allow(dead_code)]
pub fn find_stake_receipt_key(
    depositor_pubkey: Pubkey,
    stake_pool_key: Pubkey,
    receipt_nonce: u32,
    program_id: Pubkey,
) -> Pubkey {
    let seeds = &[
        depositor_pubkey.as_ref(),
        stake_pool_key.as_ref(),
        &receipt_nonce.to_le_bytes(),
        b"stakeDepositReceipt",
    ];
    Pubkey::find_program_address(seeds, &program_id).0
}

pub fn find_reward_vault_key(
    stake_pool_key: Pubkey,
    reward_mint: Pubkey,
    program_id: Pubkey,
) -> Pubkey {
    let seeds = &[
        stake_pool_key.as_ref(),
        reward_mint.as_ref(),
        b"rewardVault",
    ];
    Pubkey::find_program_address(seeds, &program_id).0
}
