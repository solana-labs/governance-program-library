use std::cell::RefCell;

use anchor_lang::{
    prelude::{Pubkey, Rent},
    AccountDeserialize,
};

use solana_program::{borsh::try_from_slice_unchecked, system_program};
use solana_program_test::{BanksClientError, ProgramTest, ProgramTestContext};
use solana_sdk::{
    account::{Account, ReadableAccount},
    instruction::Instruction,
    program_pack::Pack,
    signature::Keypair,
    signer::Signer,
    system_instruction,
    transaction::Transaction,
    transport::TransportError,
};

use borsh::BorshDeserialize;

use crate::program_test::tools::clone_keypair;

pub struct MintCookie {
    pub address: Pubkey,
    pub mint_authority: Keypair,
    pub freeze_authority: Option<Keypair>,
}
pub struct TokenAccountCookie {
    pub address: Pubkey,
}

#[derive(Debug)]
pub struct WalletCookie {
    pub address: Pubkey,
    pub account: Account,

    pub signer: Keypair,
}

pub struct ProgramTestBench {
    pub context: RefCell<ProgramTestContext>,
    pub payer: Keypair,
    pub rent: Rent,
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

    pub async fn with_mint(&self) -> Result<MintCookie, TransportError> {
        let mint_keypair = Keypair::new();
        let mint_authority = Keypair::new();
        let freeze_authority = clone_keypair(&mint_authority);

        self.create_mint(
            &mint_keypair,
            &mint_authority.pubkey(),
            Some(&freeze_authority.pubkey()),
        )
        .await?;

        Ok(MintCookie {
            address: mint_keypair.pubkey(),
            mint_authority,
            freeze_authority: Some(freeze_authority),
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
    pub async fn with_token_account(
        &self,
        token_mint: &Pubkey,
    ) -> Result<TokenAccountCookie, TransportError> {
        let token_account_keypair = Keypair::new();
        self.create_token_account(&token_account_keypair, token_mint, &self.payer.pubkey())
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
    ) -> Result<TokenAccountCookie, TransportError> {
        let token_account_keypair = Keypair::new();

        self.create_token_account(&token_account_keypair, &mint_cookie.address, owner)
            .await?;

        self.mint_tokens(
            &mint_cookie.address,
            &mint_cookie.mint_authority,
            &token_account_keypair.pubkey(),
            amount,
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
    ) -> Result<(), BanksClientError> {
        let mint_instruction = spl_token::instruction::mint_to(
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

    #[allow(dead_code)]
    pub async fn create_token_account(
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
            rent.minimum_balance(spl_token::state::Account::get_packed_len()),
            spl_token::state::Account::get_packed_len() as u64,
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
    pub async fn get_borsh_account<T: BorshDeserialize>(&self, address: &Pubkey) -> T {
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
    pub async fn get_anchor_account<T: AccountDeserialize>(&self, address: Pubkey) -> T {
        let data = self.get_account_data(address).await;
        let mut data_slice: &[u8] = &data;
        AccountDeserialize::try_deserialize(&mut data_slice).unwrap()
    }
}
