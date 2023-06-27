use std::sync::Arc;

use anchor_lang::prelude::Pubkey;

use no_transfers_signatory::{instructions::signing_authority_address, state::TransactionsChecked};
use solana_program::instruction::Instruction;
use solana_program_test::{processor, ProgramTest};
use solana_sdk::{signer::Signer, transport::TransportError};

use super::{
    governance_test::{
        cookies::{
            GovernanceCookie, ProposalCookie, ProposalTransactionCookie, RealmCookie,
            SignatoryRecordCookieWithoutKeypair,
        },
        GovernanceTest,
    },
    program_test_bench::ProgramTestBench,
};

pub struct NoTransfersSignatoryTest {
    pub program_id: Pubkey,
    pub bench: Arc<ProgramTestBench>,
    pub governance: GovernanceTest,
}

pub struct CheckTransactionCookie {
    pub address: Pubkey,
}

impl NoTransfersSignatoryTest {
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program(
            "no_transfers_signatory",
            no_transfers_signatory::id(),
            processor!(no_transfers_signatory::entry),
        )
    }

    pub async fn start_new() -> Self {
        let mut program_test = ProgramTest::default();

        NoTransfersSignatoryTest::add_program(&mut program_test);
        GovernanceTest::add_program(&mut program_test);

        let program_id = no_transfers_signatory::id();
        let bench_rc = Arc::new(ProgramTestBench::start_new(program_test).await);

        Self {
            program_id,
            bench: bench_rc.clone(),
            governance: GovernanceTest::new(bench_rc, None, None),
        }
    }

    #[allow(dead_code)]
    pub async fn check_transaction(
        &mut self,
        realm: &RealmCookie,
        proposal: &ProposalCookie,
        transaction: &ProposalTransactionCookie,
    ) -> Result<CheckTransactionCookie, TransportError> {
        let data = anchor_lang::InstructionData::data(
            &no_transfers_signatory::instruction::CheckTransaction { option: 0 },
        );

        let check_transaction_addr =
            TransactionsChecked::get_transactions_checked_address(&proposal.address);

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &no_transfers_signatory::accounts::CheckTransaction {
                realms_program: self.governance.program_id,
                realm: realm.address,
                governance: proposal.account.governance,
                proposal: proposal.address,
                transactions_checked: check_transaction_addr,
                transaction: transaction.address,
                payer: self.bench.payer.pubkey(),
                system_program: solana_sdk::system_program::id(),
            },
            None,
        );

        let check_transaction_ix = Instruction {
            program_id: no_transfers_signatory::id(),
            accounts,
            data,
        };

        self.bench
            .process_transaction(&[check_transaction_ix], None)
            .await?;

        Ok(CheckTransactionCookie {
            address: check_transaction_addr,
        })
    }

    #[allow(dead_code)]
    pub async fn sign(
        &mut self,
        realm: &RealmCookie,
        proposal: &ProposalCookie,
        signatory_record_cookie: &SignatoryRecordCookieWithoutKeypair,
    ) -> Result<(), TransportError> {
        let data =
            anchor_lang::InstructionData::data(&no_transfers_signatory::instruction::Sign {});

        let accounts = anchor_lang::ToAccountMetas::to_account_metas(
            &no_transfers_signatory::accounts::Sign {
                realms_program: self.governance.program_id,
                realm: realm.address,
                governance: proposal.account.governance,
                proposal: proposal.address,
                signatory: signing_authority_address(),
                signatory_record: signatory_record_cookie.address,
                transactions_checked: TransactionsChecked::get_transactions_checked_address(
                    &proposal.address,
                ),
            },
            None,
        );

        let sign_ix = Instruction {
            program_id: no_transfers_signatory::id(),
            accounts,
            data,
        };

        self.bench.process_transaction(&[sign_ix], None).await?;

        Ok(())
    }
}
