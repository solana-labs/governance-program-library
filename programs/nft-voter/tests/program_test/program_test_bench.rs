use std::cell::RefCell;

use anchor_lang::prelude::Pubkey;
use solana_program_test::{ProgramTest, ProgramTestContext};
use solana_sdk::{
    instruction::Instruction, program_pack::Pack, signature::Keypair, signer::Signer,
    system_instruction, transaction::Transaction,
};

pub struct ProgramTestBench {
    pub context: RefCell<ProgramTestContext>,
    pub payer: Keypair,
}
pub fn clone_keypair(source: &Keypair) -> Keypair {
    Keypair::from_bytes(&source.to_bytes()).unwrap()
}

impl ProgramTestBench {
    /// Create new bench given a ProgramTest instance populated with all of the
    /// desired programs.
    pub async fn start_new(program_test: ProgramTest) -> Self {
        let context = program_test.start_with_context().await;

        let payer = clone_keypair(&context.payer);

        Self {
            payer,
            context: RefCell::new(context),
        }
    }

    #[allow(dead_code)]
    pub async fn process_transaction(
        &self,
        instructions: &[Instruction],
        signers: Option<&[&Keypair]>,
    ) {
        let mut context = self.context.borrow_mut();

        let mut transaction =
            Transaction::new_with_payer(&instructions, Some(&context.payer.pubkey()));

        let mut all_signers = vec![&context.payer];

        if let Some(signers) = signers {
            all_signers.extend_from_slice(signers);
        }

        // This fails when warping is involved - https://gitmemory.com/issue/solana-labs/solana/18201/868325078
        // let recent_blockhash = self.context.banks_client.get_recent_blockhash().await.unwrap();

        transaction.sign(&all_signers, context.last_blockhash);

        context
            .banks_client
            .process_transaction_with_commitment(
                transaction,
                solana_sdk::commitment_config::CommitmentLevel::Processed,
            )
            .await
            .unwrap()
    }

    #[allow(dead_code)]
    pub async fn create_mint(
        &self,
        mint_keypair: &Keypair,
        mint_authority: &Pubkey,
        freeze_authority: Option<&Pubkey>,
    ) {
        let rent = self
            .context
            .borrow_mut()
            .banks_client
            .get_rent()
            .await
            .unwrap();
        let mint_rent = rent.minimum_balance(spl_token::state::Mint::LEN);

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
            .await;
    }
}
