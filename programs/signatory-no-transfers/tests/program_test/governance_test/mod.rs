pub mod args;
pub mod cookies;

use std::{rc::Rc, str::FromStr};

use solana_program::{
    bpf_loader_upgradeable::{self, UpgradeableLoaderState},
    clock::{Slot, UnixTimestamp},
    instruction::{AccountMeta, Instruction},
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    system_instruction,
};

use solana_program_test::*;

use solana_sdk::{
    signature::{Keypair, Signer},
    transport::TransportError,
};

use spl_governance::{
    instruction::{
        add_required_signatory, add_signatory, cancel_proposal, cast_vote, create_governance,
        create_mint_governance, create_native_treasury, create_program_governance, create_proposal,
        create_realm, create_token_governance, create_token_owner_record, deposit_governing_tokens,
        execute_transaction, finalize_vote, flag_transaction_error, insert_transaction,
        remove_transaction, set_governance_config, set_governance_delegate, set_realm_authority,
        set_realm_config, sign_off_proposal, upgrade_program_metadata, withdraw_governing_tokens,
        AddSignatoryAuthority,
    },
    state::{
        enums::{
            GovernanceAccountType, InstructionExecutionFlags, MintMaxVoterWeightSource,
            ProposalState, TransactionExecutionStatus, VoteThreshold,
        },
        governance::{
            get_governance_address, get_mint_governance_address, get_program_governance_address,
            get_token_governance_address, GovernanceConfig, GovernanceV2,
        },
        native_treasury::{get_native_treasury_address, NativeTreasury},
        program_metadata::{get_program_metadata_address, ProgramMetadata},
        proposal::{get_proposal_address, OptionVoteResult, ProposalOption, ProposalV2, VoteType},
        proposal_transaction::{
            get_proposal_transaction_address, InstructionData, ProposalTransactionV2,
        },
        realm::{
            get_governing_token_holding_address, get_realm_address, GoverningTokenConfigArgs,
            RealmConfig, RealmConfigArgs, RealmV2, SetRealmAuthorityAction,
        },
        realm_config::{get_realm_config_address, RealmConfigAccount},
        signatory_record::{get_signatory_record_address, SignatoryRecordV2},
        token_owner_record::{get_token_owner_record_address, TokenOwnerRecordV2},
        vote_record::{get_vote_record_address, Vote, VoteChoice, VoteRecordV2},
    },
    tools::bpf_loader_upgradeable::get_program_data_address,
};
use spl_governance_addin_api::{
    max_voter_weight::MaxVoterWeightRecord,
    voter_weight::{VoterWeightAction, VoterWeightRecord},
};
use spl_governance_addin_mock::instruction::{
    setup_max_voter_weight_record, setup_voter_weight_record,
};

use self::{
    args::SetRealmConfigArgs,
    cookies::{
        clone_keypair, GovernanceCookie, GovernedAccountCookie, GovernedMintCookie,
        GovernedProgramCookie, GovernedTokenCookie, MaxVoterWeightRecordCookie,
        NativeTreasuryCookie, ProgramMetadataCookie, ProposalCookie, ProposalTransactionCookie,
        RealmConfigCookie, RealmCookie, SignatoryRecordCookie, SignatoryRecordCookieWithoutKeypair,
        TokenOwnerRecordCookie, VoteRecordCookie, VoterWeightRecordCookie,
    },
};

use super::{
    program_test_bench::{ProgramTestBench, WalletCookie},
    tools::NopOverride,
};

pub struct GovernanceTest {
    pub program_id: Pubkey,
    pub bench: Rc<ProgramTestBench>,
    pub next_id: u8,
    pub community_voter_weight_addin: Option<Pubkey>,
    pub max_community_voter_weight_addin: Option<Pubkey>,
}

/// Yes/No Vote
pub enum YesNoVote {
    /// Yes vote
    #[allow(dead_code)]
    Yes,
    /// No vote
    #[allow(dead_code)]
    No,
}

impl GovernanceTest {
    pub fn program_id() -> Pubkey {
        Pubkey::from_str("GovER5Lthms3bLBqWub97yVrMmEogzX7xNjdXpPPCVZw").unwrap()
    }

    #[allow(dead_code)]
    pub fn add_program(program_test: &mut ProgramTest) {
        program_test.add_program("spl_governance", Self::program_id(), None);
    }

    #[allow(dead_code)]
    pub fn new(
        bench: Rc<ProgramTestBench>,
        community_voter_weight_addin: Option<Pubkey>,
        max_community_voter_weight_addin: Option<Pubkey>,
    ) -> Self {
        GovernanceTest {
            bench,
            program_id: Self::program_id(),
            next_id: 0,
            community_voter_weight_addin,
            max_community_voter_weight_addin,
        }
    }

    #[allow(dead_code)]
    pub fn get_default_set_realm_config_args(&mut self) -> SetRealmConfigArgs {
        let realm_config_args = RealmConfigArgs {
            use_council_mint: true,
            community_mint_max_voter_weight_source: MintMaxVoterWeightSource::FULL_SUPPLY_FRACTION,
            min_community_weight_to_create_governance: 10,
            community_token_config_args: GoverningTokenConfigArgs {
                use_voter_weight_addin: self.community_voter_weight_addin.is_some(),
                use_max_voter_weight_addin: self.max_community_voter_weight_addin.is_some(),
                token_type: spl_governance::state::realm_config::GoverningTokenType::Liquid,
            },
            council_token_config_args: GoverningTokenConfigArgs {
                use_voter_weight_addin: false,
                use_max_voter_weight_addin: false,
                token_type: spl_governance::state::realm_config::GoverningTokenType::Dormant,
            },
        };

        let community_voter_weight_addin = if realm_config_args
            .community_token_config_args
            .use_max_voter_weight_addin
        {
            self.community_voter_weight_addin
        } else {
            None
        };

        let max_community_voter_weight_addin = if realm_config_args
            .community_token_config_args
            .use_max_voter_weight_addin
        {
            self.max_community_voter_weight_addin
        } else {
            None
        };

        SetRealmConfigArgs {
            realm_config_args,
            community_voter_weight_addin,
            max_community_voter_weight_addin,
        }
    }

    #[allow(dead_code)]
    pub async fn with_realm(&mut self) -> RealmCookie {
        let set_realm_config_args = self.get_default_set_realm_config_args();
        self.with_realm_using_config_args(&set_realm_config_args)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_realm_using_config_args(
        &mut self,
        set_realm_config_args: &SetRealmConfigArgs,
    ) -> RealmCookie {
        let name = format!("Realm #{}", self.next_id).to_string();
        self.next_id += 1;

        let realm_address = get_realm_address(&self.program_id, &name);

        let community_token_mint_keypair = Keypair::new();
        let community_token_mint_authority = Keypair::new();

        let community_token_holding_address = get_governing_token_holding_address(
            &self.program_id,
            &realm_address,
            &community_token_mint_keypair.pubkey(),
        );

        self.bench
            .create_mint(
                &community_token_mint_keypair,
                &community_token_mint_authority.pubkey(),
                None,
            )
            .await
            .unwrap();

        let (
            council_token_mint_pubkey,
            council_token_holding_address,
            council_token_mint_authority,
        ) = if set_realm_config_args.realm_config_args.use_council_mint {
            let council_token_mint_keypair = Keypair::new();
            let council_token_mint_authority = Keypair::new();

            let council_token_holding_address = get_governing_token_holding_address(
                &self.program_id,
                &realm_address,
                &council_token_mint_keypair.pubkey(),
            );

            self.bench
                .create_mint(
                    &council_token_mint_keypair,
                    &council_token_mint_authority.pubkey(),
                    None,
                )
                .await
                .unwrap();

            (
                Some(council_token_mint_keypair.pubkey()),
                Some(council_token_holding_address),
                Some(council_token_mint_authority),
            )
        } else {
            (None, None, None)
        };

        let realm_authority = Keypair::new();

        let create_realm_ix = create_realm(
            &self.program_id,
            &realm_authority.pubkey(),
            &community_token_mint_keypair.pubkey(),
            &self.bench.payer.pubkey(),
            council_token_mint_pubkey,
            Some(set_realm_config_args.community()),
            Some(set_realm_config_args.council()),
            name.clone(),
            set_realm_config_args
                .realm_config_args
                .min_community_weight_to_create_governance,
            MintMaxVoterWeightSource::FULL_SUPPLY_FRACTION,
        );

        self.bench
            .process_transaction(&[create_realm_ix], None)
            .await
            .unwrap();

        let account = RealmV2 {
            account_type: GovernanceAccountType::RealmV2,
            community_mint: community_token_mint_keypair.pubkey(),

            name,
            reserved: [0; 6],
            authority: Some(realm_authority.pubkey()),
            config: RealmConfig {
                council_mint: council_token_mint_pubkey,
                reserved: [0; 6],

                min_community_weight_to_create_governance: set_realm_config_args
                    .realm_config_args
                    .min_community_weight_to_create_governance,
                community_mint_max_voter_weight_source:
                    MintMaxVoterWeightSource::FULL_SUPPLY_FRACTION,
                legacy1: Default::default(),
                legacy2: Default::default(),
            },
            reserved_v2: [0; 128],
            legacy1: Default::default(),
        };

        let realm_config_cookie = if set_realm_config_args.community_voter_weight_addin.is_some()
            || set_realm_config_args
                .max_community_voter_weight_addin
                .is_some()
        {
            Some(RealmConfigCookie {
                address: get_realm_config_address(&self.program_id, &realm_address),
                account: RealmConfigAccount {
                    account_type: GovernanceAccountType::RealmConfig,
                    realm: realm_address,
                    community_token_config: set_realm_config_args.community_args(),
                    council_token_config: set_realm_config_args.council_args(),
                    reserved: Default::default(),
                },
            })
        } else {
            None
        };

        RealmCookie {
            address: realm_address,
            account,

            community_mint_authority: community_token_mint_authority,
            community_token_holding_account: community_token_holding_address,

            council_token_holding_account: council_token_holding_address,
            council_mint_authority: council_token_mint_authority,
            realm_authority: Some(realm_authority),
            realm_config: realm_config_cookie,
        }
    }

    #[allow(dead_code)]
    pub async fn with_realm_using_mints(&mut self, realm_cookie: &RealmCookie) -> RealmCookie {
        let name = format!("Realm #{}", self.next_id).to_string();
        self.next_id += 1;

        let realm_address = get_realm_address(&self.program_id, &name);
        let council_mint = realm_cookie.account.config.council_mint.unwrap();

        let realm_authority = Keypair::new();

        let community_mint_max_vote_weight_source = MintMaxVoterWeightSource::FULL_SUPPLY_FRACTION;
        let min_community_weight_to_create_governance = 10;

        let create_realm_ix = create_realm(
            &self.program_id,
            &realm_authority.pubkey(),
            &realm_cookie.account.community_mint,
            &self.bench.payer.pubkey(),
            Some(council_mint),
            None,
            None,
            name.clone(),
            min_community_weight_to_create_governance,
            community_mint_max_vote_weight_source.clone(),
        );

        self.bench
            .process_transaction(&[create_realm_ix], None)
            .await
            .unwrap();

        let account = RealmV2 {
            account_type: GovernanceAccountType::RealmV2,
            community_mint: realm_cookie.account.community_mint,

            name,
            reserved: [0; 6],
            authority: Some(realm_authority.pubkey()),
            config: RealmConfig {
                council_mint: Some(council_mint),
                reserved: [0; 6],
                legacy1: Default::default(),
                legacy2: Default::default(),
                min_community_weight_to_create_governance,
                community_mint_max_voter_weight_source: community_mint_max_vote_weight_source,
            },
            legacy1: Default::default(),
            reserved_v2: [0; 128],
        };

        let community_token_holding_address = get_governing_token_holding_address(
            &self.program_id,
            &realm_address,
            &realm_cookie.account.community_mint,
        );

        let council_token_holding_address =
            get_governing_token_holding_address(&self.program_id, &realm_address, &council_mint);

        RealmCookie {
            address: realm_address,
            account,

            community_mint_authority: clone_keypair(&realm_cookie.community_mint_authority),
            community_token_holding_account: community_token_holding_address,

            council_token_holding_account: Some(council_token_holding_address),
            council_mint_authority: Some(clone_keypair(
                realm_cookie.council_mint_authority.as_ref().unwrap(),
            )),
            realm_authority: Some(realm_authority),
            realm_config: None,
        }
    }

    // Creates TokenOwner which owns 100 community tokens and deposits them into the given Realm
    #[allow(dead_code)]
    pub async fn with_community_token_deposit(
        &mut self,
        realm_cookie: &RealmCookie,
    ) -> Result<TokenOwnerRecordCookie, TransportError> {
        self.with_initial_governing_token_deposit(
            &realm_cookie.address,
            &realm_cookie.account.community_mint,
            &realm_cookie.community_mint_authority,
            100,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_community_token_owner_record(
        &mut self,
        realm_cookie: &RealmCookie,
    ) -> TokenOwnerRecordCookie {
        let token_owner = Keypair::new();

        let create_token_owner_record_ix = create_token_owner_record(
            &self.program_id,
            &realm_cookie.address,
            &token_owner.pubkey(),
            &realm_cookie.account.community_mint,
            &self.bench.payer.pubkey(),
        );

        self.bench
            .process_transaction(&[create_token_owner_record_ix], None)
            .await
            .unwrap();

        let account = TokenOwnerRecordV2 {
            account_type: GovernanceAccountType::TokenOwnerRecordV2,
            realm: realm_cookie.address,
            governing_token_mint: realm_cookie.account.community_mint,
            governing_token_owner: token_owner.pubkey(),
            governing_token_deposit_amount: 0,
            governance_delegate: None,
            unrelinquished_votes_count: 0,
            outstanding_proposal_count: 0,
            reserved_v2: [0; 128],
            version: 3,
            reserved: Default::default(),
        };

        let token_owner_record_address = get_token_owner_record_address(
            &self.program_id,
            &realm_cookie.address,
            &realm_cookie.account.community_mint,
            &token_owner.pubkey(),
        );

        TokenOwnerRecordCookie {
            address: token_owner_record_address,
            account,
            token_source_amount: 0,
            token_source: Pubkey::new_unique(),
            token_owner,
            governance_authority: None,
            governance_delegate: Keypair::new(),
            voter_weight_record: None,
            max_voter_weight_record: None,
        }
    }

    #[allow(dead_code)]
    pub async fn with_program_metadata(&mut self) -> ProgramMetadataCookie {
        let update_program_metadata_ix =
            upgrade_program_metadata(&self.program_id, &self.bench.payer.pubkey());

        self.bench
            .process_transaction(&[update_program_metadata_ix], None)
            .await
            .unwrap();

        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let clock = self.bench.get_clock().await;

        let account = ProgramMetadata {
            account_type: GovernanceAccountType::ProgramMetadata,
            updated_at: clock.slot,
            version: VERSION.to_string(),
            reserved: [0; 64],
        };

        let program_metadata_address = get_program_metadata_address(&self.program_id);

        ProgramMetadataCookie {
            address: program_metadata_address,
            account,
        }
    }

    #[allow(dead_code)]
    pub async fn with_native_treasury(
        &mut self,
        governance_cookie: &GovernanceCookie,
    ) -> NativeTreasuryCookie {
        let create_treasury_ix = create_native_treasury(
            &self.program_id,
            &governance_cookie.address,
            &self.bench.payer.pubkey(),
        );

        let treasury_address =
            get_native_treasury_address(&self.program_id, &governance_cookie.address);

        let transfer_ix = system_instruction::transfer(
            &self.bench.payer.pubkey(),
            &treasury_address,
            1_000_000_000,
        );

        self.bench
            .process_transaction(&[create_treasury_ix, transfer_ix], None)
            .await
            .unwrap();

        NativeTreasuryCookie {
            address: treasury_address,
            account: NativeTreasury {},
        }
    }

    #[allow(dead_code)]
    pub async fn with_community_token_deposit_amount(
        &mut self,
        realm_cookie: &RealmCookie,
        amount: u64,
    ) -> Result<TokenOwnerRecordCookie, TransportError> {
        self.with_initial_governing_token_deposit(
            &realm_cookie.address,
            &realm_cookie.account.community_mint,
            &realm_cookie.community_mint_authority,
            amount,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_subsequent_community_token_deposit(
        &mut self,
        realm_cookie: &RealmCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        amount: u64,
    ) {
        self.with_subsequent_governing_token_deposit(
            &realm_cookie.address,
            &realm_cookie.account.community_mint,
            &realm_cookie.community_mint_authority,
            token_owner_record_cookie,
            amount,
        )
        .await;
    }

    #[allow(dead_code)]
    pub async fn with_subsequent_council_token_deposit(
        &mut self,
        realm_cookie: &RealmCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        amount: u64,
    ) {
        self.with_subsequent_governing_token_deposit(
            &realm_cookie.address,
            &realm_cookie.account.config.council_mint.unwrap(),
            realm_cookie.council_mint_authority.as_ref().unwrap(),
            token_owner_record_cookie,
            amount,
        )
        .await;
    }

    #[allow(dead_code)]
    pub async fn with_council_token_deposit_amount(
        &mut self,
        realm_cookie: &RealmCookie,
        amount: u64,
    ) -> Result<TokenOwnerRecordCookie, TransportError> {
        self.with_initial_governing_token_deposit(
            &realm_cookie.address,
            &realm_cookie.account.config.council_mint.unwrap(),
            realm_cookie.council_mint_authority.as_ref().unwrap(),
            amount,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_council_token_deposit(
        &mut self,
        realm_cookie: &RealmCookie,
    ) -> Result<TokenOwnerRecordCookie, TransportError> {
        self.with_initial_governing_token_deposit(
            &realm_cookie.address,
            &realm_cookie.account.config.council_mint.unwrap(),
            realm_cookie.council_mint_authority.as_ref().unwrap(),
            100,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_community_token_deposit_by_owner(
        &mut self,
        realm_cookie: &RealmCookie,
        amount: u64,
        token_owner: Keypair,
    ) -> Result<TokenOwnerRecordCookie, TransportError> {
        self.with_initial_governing_token_deposit(
            &realm_cookie.address,
            &realm_cookie.account.community_mint,
            &realm_cookie.community_mint_authority,
            amount,
            Some(token_owner),
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_initial_governing_token_deposit(
        &mut self,
        realm_address: &Pubkey,
        governing_mint: &Pubkey,
        governing_mint_authority: &Keypair,
        amount: u64,
        token_owner: Option<Keypair>,
    ) -> Result<TokenOwnerRecordCookie, TransportError> {
        let token_owner = token_owner.unwrap_or_else(Keypair::new);
        let token_source = Keypair::new();

        let transfer_authority = Keypair::new();

        self.bench
            .create_token_account_with_transfer_authority(
                &token_source,
                governing_mint,
                governing_mint_authority,
                amount,
                &token_owner,
                &transfer_authority.pubkey(),
            )
            .await;

        let deposit_governing_tokens_ix = deposit_governing_tokens(
            &self.program_id,
            realm_address,
            &token_source.pubkey(),
            &token_owner.pubkey(),
            &token_owner.pubkey(),
            &self.bench.payer.pubkey(),
            amount,
            governing_mint,
        );

        self.bench
            .process_transaction(&[deposit_governing_tokens_ix], Some(&[&token_owner]))
            .await?;

        let token_owner_record_address = get_token_owner_record_address(
            &self.program_id,
            realm_address,
            governing_mint,
            &token_owner.pubkey(),
        );

        let account = TokenOwnerRecordV2 {
            account_type: GovernanceAccountType::TokenOwnerRecordV2,
            realm: *realm_address,
            governing_token_mint: *governing_mint,
            governing_token_owner: token_owner.pubkey(),
            governing_token_deposit_amount: amount,
            governance_delegate: None,
            unrelinquished_votes_count: 0,
            outstanding_proposal_count: 0,
            reserved: [0; 6],
            reserved_v2: [0; 128],
            version: 3,
        };

        let governance_delegate = Keypair::from_base58_string(&token_owner.to_base58_string());

        Ok(TokenOwnerRecordCookie {
            address: token_owner_record_address,
            account,

            token_source_amount: amount,
            token_source: token_source.pubkey(),
            token_owner,
            governance_authority: None,
            governance_delegate,
            voter_weight_record: None,
            max_voter_weight_record: None,
        })
    }

    #[allow(dead_code)]
    pub async fn mint_community_tokens(&mut self, realm_cookie: &RealmCookie, amount: u64) {
        let token_account_keypair = Keypair::new();

        self.bench
            .create_empty_token_account(
                &token_account_keypair,
                &realm_cookie.account.community_mint,
                &self.bench.payer.pubkey(),
            )
            .await;

        self.bench
            .mint_tokens(
                &realm_cookie.account.community_mint,
                &realm_cookie.community_mint_authority,
                &token_account_keypair.pubkey(),
                amount,
            )
            .await
            .unwrap();
    }

    #[allow(dead_code)]
    pub async fn mint_council_tokens(&mut self, realm_cookie: &RealmCookie, amount: u64) {
        let token_account_keypair = Keypair::new();
        let council_mint = realm_cookie.account.config.council_mint.unwrap();

        self.bench
            .create_empty_token_account(
                &token_account_keypair,
                &council_mint,
                &self.bench.payer.pubkey(),
            )
            .await;

        self.bench
            .mint_tokens(
                &council_mint,
                realm_cookie.council_mint_authority.as_ref().unwrap(),
                &token_account_keypair.pubkey(),
                amount,
            )
            .await
            .unwrap();
    }

    #[allow(dead_code)]
    async fn with_subsequent_governing_token_deposit(
        &mut self,
        realm: &Pubkey,
        governing_token_mint: &Pubkey,
        governing_token_mint_authority: &Keypair,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        amount: u64,
    ) {
        self.bench
            .mint_tokens(
                governing_token_mint,
                governing_token_mint_authority,
                &token_owner_record_cookie.token_source,
                amount,
            )
            .await
            .unwrap();

        let deposit_governing_tokens_ix = deposit_governing_tokens(
            &self.program_id,
            realm,
            &token_owner_record_cookie.token_source,
            &token_owner_record_cookie.token_owner.pubkey(),
            &token_owner_record_cookie.token_owner.pubkey(),
            &self.bench.payer.pubkey(),
            amount,
            governing_token_mint,
        );

        self.bench
            .process_transaction(
                &[deposit_governing_tokens_ix],
                Some(&[&token_owner_record_cookie.token_owner]),
            )
            .await
            .unwrap();
    }

    #[allow(dead_code)]
    pub async fn with_community_governance_delegate(
        &mut self,
        realm_cookie: &RealmCookie,
        token_owner_record_cookie: &mut TokenOwnerRecordCookie,
    ) {
        self.with_governing_token_governance_delegate(
            realm_cookie,
            &realm_cookie.account.community_mint,
            token_owner_record_cookie,
        )
        .await;
    }

    #[allow(dead_code)]
    pub async fn with_council_governance_delegate(
        &mut self,
        realm_cookie: &RealmCookie,
        token_owner_record_cookie: &mut TokenOwnerRecordCookie,
    ) {
        self.with_governing_token_governance_delegate(
            realm_cookie,
            &realm_cookie.account.config.council_mint.unwrap(),
            token_owner_record_cookie,
        )
        .await;
    }

    #[allow(dead_code)]
    pub async fn with_governing_token_governance_delegate(
        &mut self,
        realm_cookie: &RealmCookie,
        governing_token_mint: &Pubkey,
        token_owner_record_cookie: &mut TokenOwnerRecordCookie,
    ) {
        let new_governance_delegate = Keypair::new();

        self.set_governance_delegate(
            realm_cookie,
            token_owner_record_cookie,
            &token_owner_record_cookie.token_owner,
            governing_token_mint,
            &Some(new_governance_delegate.pubkey()),
        )
        .await;

        token_owner_record_cookie.governance_delegate = new_governance_delegate;
    }

    #[allow(dead_code)]
    pub async fn set_governance_delegate(
        &mut self,
        realm_cookie: &RealmCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        signing_governance_authority: &Keypair,
        governing_token_mint: &Pubkey,
        new_governance_delegate: &Option<Pubkey>,
    ) {
        let set_governance_delegate_ix = set_governance_delegate(
            &self.program_id,
            &signing_governance_authority.pubkey(),
            &realm_cookie.address,
            governing_token_mint,
            &token_owner_record_cookie.token_owner.pubkey(),
            new_governance_delegate,
        );

        self.bench
            .process_transaction(
                &[set_governance_delegate_ix],
                Some(&[signing_governance_authority]),
            )
            .await
            .unwrap();
    }

    #[allow(dead_code)]
    pub async fn set_realm_authority(
        &mut self,
        realm_cookie: &RealmCookie,
        new_realm_authority: Option<&Pubkey>,
    ) -> Result<(), TransportError> {
        self.set_realm_authority_using_instruction(
            realm_cookie,
            new_realm_authority,
            true,
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn set_realm_authority_impl(
        &mut self,
        realm_cookie: &RealmCookie,
        new_realm_authority: Option<&Pubkey>,
        check_authority: bool,
    ) -> Result<(), TransportError> {
        self.set_realm_authority_using_instruction(
            realm_cookie,
            new_realm_authority,
            check_authority,
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn set_realm_authority_using_instruction<F: Fn(&mut Instruction)>(
        &mut self,
        realm_cookie: &RealmCookie,
        new_realm_authority: Option<&Pubkey>,
        check_authority: bool,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<(), TransportError> {
        let action = if new_realm_authority.is_some() {
            if check_authority {
                SetRealmAuthorityAction::SetChecked
            } else {
                SetRealmAuthorityAction::SetUnchecked
            }
        } else {
            SetRealmAuthorityAction::Remove
        };

        let mut set_realm_authority_ix = set_realm_authority(
            &self.program_id,
            &realm_cookie.address,
            &realm_cookie.realm_authority.as_ref().unwrap().pubkey(),
            new_realm_authority,
            action,
        );

        instruction_override(&mut set_realm_authority_ix);

        let default_signers = &[realm_cookie.realm_authority.as_ref().unwrap()];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[set_realm_authority_ix], Some(signers))
            .await
    }

    #[allow(dead_code)]
    pub async fn set_realm_config(
        &mut self,
        realm_cookie: &mut RealmCookie,
        set_realm_config_args: &SetRealmConfigArgs,
    ) -> Result<(), TransportError> {
        self.set_realm_config_using_instruction(
            realm_cookie,
            set_realm_config_args,
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn set_realm_config_using_instruction<F: Fn(&mut Instruction)>(
        &mut self,
        realm_cookie: &mut RealmCookie,
        set_realm_config_args: &SetRealmConfigArgs,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<(), TransportError> {
        let council_token_mint = if set_realm_config_args.realm_config_args.use_council_mint {
            realm_cookie.account.config.council_mint
        } else {
            None
        };

        if set_realm_config_args
            .realm_config_args
            .community_token_config_args
            .use_voter_weight_addin
        {
            set_realm_config_args.community_voter_weight_addin
        } else {
            None
        };

        if set_realm_config_args
            .realm_config_args
            .community_token_config_args
            .use_max_voter_weight_addin
        {
            set_realm_config_args.max_community_voter_weight_addin
        } else {
            None
        };

        let mut set_realm_config_ix = set_realm_config(
            &self.program_id,
            &realm_cookie.address,
            &realm_cookie.realm_authority.as_ref().unwrap().pubkey(),
            council_token_mint,
            &self.bench.payer.pubkey(),
            Some(set_realm_config_args.community()),
            Some(set_realm_config_args.council()),
            set_realm_config_args
                .realm_config_args
                .min_community_weight_to_create_governance,
            set_realm_config_args
                .realm_config_args
                .community_mint_max_voter_weight_source
                .clone(),
        );

        instruction_override(&mut set_realm_config_ix);

        let default_signers = &[realm_cookie.realm_authority.as_ref().unwrap()];
        let signers = signers_override.unwrap_or(default_signers);

        realm_cookie.account.config.council_mint = council_token_mint;
        realm_cookie
            .account
            .config
            .community_mint_max_voter_weight_source = set_realm_config_args
            .realm_config_args
            .community_mint_max_voter_weight_source
            .clone();

        if set_realm_config_args
            .realm_config_args
            .community_token_config_args
            .use_voter_weight_addin
            || set_realm_config_args
                .realm_config_args
                .community_token_config_args
                .use_max_voter_weight_addin
        {
            realm_cookie.realm_config = Some(RealmConfigCookie {
                address: get_realm_config_address(&self.program_id, &realm_cookie.address),
                account: RealmConfigAccount {
                    account_type: GovernanceAccountType::RealmConfig,
                    realm: realm_cookie.address,
                    community_token_config: set_realm_config_args.community_args(),
                    council_token_config: set_realm_config_args.council_args(),
                    reserved: Default::default(),
                },
            })
        }

        self.bench
            .process_transaction(&[set_realm_config_ix], Some(signers))
            .await
    }

    #[allow(dead_code)]
    pub async fn withdraw_community_tokens(
        &mut self,
        realm_cookie: &RealmCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
    ) -> Result<(), TransportError> {
        self.withdraw_governing_tokens(
            realm_cookie,
            token_owner_record_cookie,
            &realm_cookie.account.community_mint,
            &token_owner_record_cookie.token_owner,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn withdraw_council_tokens(
        &mut self,
        realm_cookie: &RealmCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
    ) -> Result<(), TransportError> {
        self.withdraw_governing_tokens(
            realm_cookie,
            token_owner_record_cookie,
            &realm_cookie.account.config.council_mint.unwrap(),
            &token_owner_record_cookie.token_owner,
        )
        .await
    }

    #[allow(dead_code)]
    async fn withdraw_governing_tokens(
        &mut self,
        realm_cookie: &RealmCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        governing_token_mint: &Pubkey,

        governing_token_owner: &Keypair,
    ) -> Result<(), TransportError> {
        let deposit_governing_tokens_ix = withdraw_governing_tokens(
            &self.program_id,
            &realm_cookie.address,
            &token_owner_record_cookie.token_source,
            &governing_token_owner.pubkey(),
            governing_token_mint,
        );

        self.bench
            .process_transaction(
                &[deposit_governing_tokens_ix],
                Some(&[governing_token_owner]),
            )
            .await
    }

    #[allow(dead_code)]
    pub async fn with_governed_account(&mut self) -> GovernedAccountCookie {
        GovernedAccountCookie {
            address: Pubkey::new_unique(),
        }
    }

    #[allow(dead_code)]
    pub async fn with_governed_mint(&mut self) -> GovernedMintCookie {
        let mint_authority = Keypair::new();

        self.with_governed_mint_impl(&mint_authority, None).await
    }

    #[allow(dead_code)]
    pub async fn with_freezable_governed_mint(&mut self) -> GovernedMintCookie {
        let mint_authority = Keypair::new();

        self.with_governed_mint_impl(&mint_authority, Some(&mint_authority.pubkey()))
            .await
    }

    #[allow(dead_code)]
    pub async fn with_governed_mint_impl(
        &mut self,
        mint_authority: &Keypair,
        freeze_authority: Option<&Pubkey>,
    ) -> GovernedMintCookie {
        let mint_keypair = Keypair::new();

        self.bench
            .create_mint(&mint_keypair, &mint_authority.pubkey(), freeze_authority)
            .await
            .unwrap();

        GovernedMintCookie {
            address: mint_keypair.pubkey(),
            mint_authority: clone_keypair(mint_authority),
            transfer_mint_authority: true,
        }
    }

    #[allow(dead_code)]
    pub async fn with_governed_token(&mut self) -> GovernedTokenCookie {
        let mint_keypair = Keypair::new();
        let mint_authority = Keypair::new();

        self.bench
            .create_mint(&mint_keypair, &mint_authority.pubkey(), None)
            .await
            .unwrap();

        let token_keypair = Keypair::new();
        let token_owner = Keypair::new();

        self.bench
            .create_empty_token_account(
                &token_keypair,
                &mint_keypair.pubkey(),
                &token_owner.pubkey(),
            )
            .await;

        self.bench
            .mint_tokens(
                &mint_keypair.pubkey(),
                &mint_authority,
                &token_keypair.pubkey(),
                100,
            )
            .await
            .unwrap();

        GovernedTokenCookie {
            address: token_keypair.pubkey(),
            token_owner,
            transfer_token_owner: true,
            token_mint: mint_keypair.pubkey(),
        }
    }

    pub fn get_default_governance_config(&mut self) -> GovernanceConfig {
        GovernanceConfig {
            min_community_weight_to_create_proposal: 5,
            min_council_weight_to_create_proposal: 2,
            min_transaction_hold_up_time: 10,
            community_vote_threshold: VoteThreshold::YesVotePercentage(60),
            council_vote_threshold: VoteThreshold::YesVotePercentage(80),
            council_veto_vote_threshold: VoteThreshold::YesVotePercentage(55),
            voting_base_time: 10,
            community_vote_tipping: spl_governance::state::enums::VoteTipping::Strict,
            council_vote_tipping: spl_governance::state::enums::VoteTipping::Strict,
            community_veto_vote_threshold: VoteThreshold::YesVotePercentage(55),
            voting_cool_off_time: 0,
            deposit_exempt_proposal_count: 100,
        }
    }

    #[allow(dead_code)]
    pub async fn with_governance(
        &mut self,
        realm_cookie: &RealmCookie,
        governed_account_cookie: &GovernedAccountCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
    ) -> Result<GovernanceCookie, TransportError> {
        let config = self.get_default_governance_config();
        self.with_governance_using_config(
            realm_cookie,
            governed_account_cookie,
            token_owner_record_cookie,
            &config,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_governance_using_config(
        &mut self,
        realm_cookie: &RealmCookie,
        governed_account_cookie: &GovernedAccountCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        governance_config: &GovernanceConfig,
    ) -> Result<GovernanceCookie, TransportError> {
        let voter_weight_record = token_owner_record_cookie
            .voter_weight_record
            .as_ref()
            .map(|voter_weight_record| voter_weight_record.address);

        self.with_governance_impl(
            realm_cookie,
            governed_account_cookie,
            Some(&token_owner_record_cookie.address),
            &token_owner_record_cookie.token_owner,
            voter_weight_record,
            governance_config,
            None,
        )
        .await
    }

    #[allow(dead_code, clippy::too_many_arguments)]
    pub async fn with_governance_impl(
        &mut self,
        realm_cookie: &RealmCookie,
        governed_account_cookie: &GovernedAccountCookie,
        token_owner_record: Option<&Pubkey>,
        create_authority: &Keypair,
        voter_weight_record: Option<Pubkey>,
        governance_config: &GovernanceConfig,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<GovernanceCookie, TransportError> {
        let mut create_governance_ix = create_governance(
            &self.program_id,
            &realm_cookie.address,
            Some(&governed_account_cookie.address),
            token_owner_record.unwrap_or(&Pubkey::new_unique()),
            &self.bench.payer.pubkey(),
            &create_authority.pubkey(),
            voter_weight_record,
            governance_config.clone(),
        );

        let account = GovernanceV2 {
            account_type: GovernanceAccountType::GovernanceV2,
            realm: realm_cookie.address,
            governed_account: governed_account_cookie.address,
            config: governance_config.clone(),
            required_signatories_count: 0,
            reserved1: Default::default(),
            active_proposal_count: 0,
            reserved_v2: Default::default(),
        };

        let default_signers = &[create_authority];
        let signers = signers_override.unwrap_or(default_signers);

        if signers.is_empty() {
            create_governance_ix.accounts[6].is_signer = false;
        }

        self.bench
            .process_transaction(&[create_governance_ix], Some(signers))
            .await?;

        let governance_address = get_governance_address(
            &self.program_id,
            &realm_cookie.address,
            &governed_account_cookie.address,
        );

        Ok(GovernanceCookie {
            address: governance_address,
            account,
            next_proposal_index: 0,
        })
    }

    #[allow(dead_code)]
    pub async fn with_governed_program(&mut self) -> GovernedProgramCookie {
        let program_keypair = Keypair::new();
        let program_buffer_keypair = Keypair::new();
        let program_upgrade_authority_keypair = Keypair::new();

        let program_data_address = get_program_data_address(&program_keypair.pubkey());

        // Load solana_bpf_rust_upgradeable program taken from solana test programs
        let path_buf = find_file("solana_bpf_rust_upgradeable.so").unwrap();
        let program_data = read_file(path_buf);

        let program_buffer_rent =
            self.bench
                .rent
                .minimum_balance(UpgradeableLoaderState::size_of_programdata(
                    program_data.len(),
                ));

        let mut instructions = bpf_loader_upgradeable::create_buffer(
            &self.bench.payer.pubkey(),
            &program_buffer_keypair.pubkey(),
            &program_upgrade_authority_keypair.pubkey(),
            program_buffer_rent,
            program_data.len(),
        )
        .unwrap();

        let chunk_size = 800;

        for (chunk, i) in program_data.chunks(chunk_size).zip(0..) {
            instructions.push(bpf_loader_upgradeable::write(
                &program_buffer_keypair.pubkey(),
                &program_upgrade_authority_keypair.pubkey(),
                (i * chunk_size) as u32,
                chunk.to_vec(),
            ));
        }

        let program_account_rent = self
            .bench
            .rent
            .minimum_balance(UpgradeableLoaderState::size_of_program());

        let deploy_ixs = bpf_loader_upgradeable::deploy_with_max_program_len(
            &self.bench.payer.pubkey(),
            &program_keypair.pubkey(),
            &program_buffer_keypair.pubkey(),
            &program_upgrade_authority_keypair.pubkey(),
            program_account_rent,
            program_data.len(),
        )
        .unwrap();

        instructions.extend_from_slice(&deploy_ixs);

        self.bench
            .process_transaction(
                &instructions[..],
                Some(&[
                    &program_upgrade_authority_keypair,
                    &program_keypair,
                    &program_buffer_keypair,
                ]),
            )
            .await
            .unwrap();

        GovernedProgramCookie {
            address: program_keypair.pubkey(),
            upgrade_authority: program_upgrade_authority_keypair,
            data_address: program_data_address,
            transfer_upgrade_authority: true,
        }
    }

    #[allow(dead_code)]
    pub async fn with_program_governance(
        &mut self,
        realm_cookie: &RealmCookie,
        governed_program_cookie: &GovernedProgramCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
    ) -> Result<GovernanceCookie, TransportError> {
        self.with_program_governance_using_instruction(
            realm_cookie,
            governed_program_cookie,
            token_owner_record_cookie,
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_program_governance_using_instruction<F: Fn(&mut Instruction)>(
        &mut self,
        realm_cookie: &RealmCookie,
        governed_program_cookie: &GovernedProgramCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<GovernanceCookie, TransportError> {
        let config = self.get_default_governance_config();

        let voter_weight_record = token_owner_record_cookie
            .voter_weight_record
            .as_ref()
            .map(|voter_weight_record| voter_weight_record.address);

        let mut create_program_governance_ix = create_program_governance(
            &self.program_id,
            &realm_cookie.address,
            &governed_program_cookie.address,
            &governed_program_cookie.upgrade_authority.pubkey(),
            &token_owner_record_cookie.address,
            &self.bench.payer.pubkey(),
            &token_owner_record_cookie.token_owner.pubkey(),
            voter_weight_record,
            config.clone(),
            governed_program_cookie.transfer_upgrade_authority,
        );

        instruction_override(&mut create_program_governance_ix);

        let default_signers = &[
            &governed_program_cookie.upgrade_authority,
            &token_owner_record_cookie.token_owner,
        ];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[create_program_governance_ix], Some(signers))
            .await?;

        let account = GovernanceV2 {
            account_type: GovernanceAccountType::ProgramGovernanceV2,
            realm: realm_cookie.address,
            governed_account: governed_program_cookie.address,
            config,
            required_signatories_count: 0,
            reserved1: Default::default(),
            reserved_v2: Default::default(),
            active_proposal_count: 0,
        };

        let program_governance_address = get_program_governance_address(
            &self.program_id,
            &realm_cookie.address,
            &governed_program_cookie.address,
        );

        Ok(GovernanceCookie {
            address: program_governance_address,
            account,
            next_proposal_index: 0,
        })
    }

    #[allow(dead_code)]
    pub async fn with_mint_governance(
        &mut self,
        realm_cookie: &RealmCookie,
        governed_mint_cookie: &GovernedMintCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
    ) -> Result<GovernanceCookie, TransportError> {
        self.with_mint_governance_using_instruction(
            realm_cookie,
            governed_mint_cookie,
            token_owner_record_cookie,
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_mint_governance_using_config(
        &mut self,
        realm_cookie: &RealmCookie,
        governed_mint_cookie: &GovernedMintCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        governance_config: &GovernanceConfig,
    ) -> Result<GovernanceCookie, TransportError> {
        self.with_mint_governance_using_config_and_instruction(
            realm_cookie,
            governed_mint_cookie,
            token_owner_record_cookie,
            governance_config,
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_mint_governance_using_instruction<F: Fn(&mut Instruction)>(
        &mut self,
        realm_cookie: &RealmCookie,
        governed_mint_cookie: &GovernedMintCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<GovernanceCookie, TransportError> {
        let governance_config = self.get_default_governance_config();

        self.with_mint_governance_using_config_and_instruction(
            realm_cookie,
            governed_mint_cookie,
            token_owner_record_cookie,
            &governance_config,
            instruction_override,
            signers_override,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_mint_governance_using_config_and_instruction<F: Fn(&mut Instruction)>(
        &mut self,
        realm_cookie: &RealmCookie,
        governed_mint_cookie: &GovernedMintCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        governance_config: &GovernanceConfig,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<GovernanceCookie, TransportError> {
        let voter_weight_record = token_owner_record_cookie
            .voter_weight_record
            .as_ref()
            .map(|voter_weight_record| voter_weight_record.address);

        let mut create_mint_governance_ix = create_mint_governance(
            &self.program_id,
            &realm_cookie.address,
            &governed_mint_cookie.address,
            &governed_mint_cookie.mint_authority.pubkey(),
            &token_owner_record_cookie.address,
            &self.bench.payer.pubkey(),
            &token_owner_record_cookie.token_owner.pubkey(),
            voter_weight_record,
            governance_config.clone(),
            governed_mint_cookie.transfer_mint_authority,
        );

        instruction_override(&mut create_mint_governance_ix);

        let default_signers = &[
            &governed_mint_cookie.mint_authority,
            &token_owner_record_cookie.token_owner,
        ];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[create_mint_governance_ix], Some(signers))
            .await?;

        let account = GovernanceV2 {
            account_type: GovernanceAccountType::MintGovernanceV2,
            realm: realm_cookie.address,
            governed_account: governed_mint_cookie.address,
            config: governance_config.clone(),
            required_signatories_count: 0,
            reserved1: Default::default(),
            reserved_v2: Default::default(),
            active_proposal_count: 0,
        };

        let mint_governance_address = get_mint_governance_address(
            &self.program_id,
            &realm_cookie.address,
            &governed_mint_cookie.address,
        );

        Ok(GovernanceCookie {
            address: mint_governance_address,
            account,
            next_proposal_index: 0,
        })
    }

    #[allow(dead_code)]
    pub async fn with_token_governance(
        &mut self,
        realm_cookie: &RealmCookie,
        governed_token_cookie: &GovernedTokenCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
    ) -> Result<GovernanceCookie, TransportError> {
        self.with_token_governance_using_instruction(
            realm_cookie,
            governed_token_cookie,
            token_owner_record_cookie,
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_token_governance_using_instruction<F: Fn(&mut Instruction)>(
        &mut self,
        realm_cookie: &RealmCookie,
        governed_token_cookie: &GovernedTokenCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<GovernanceCookie, TransportError> {
        let config = self.get_default_governance_config();

        let voter_weight_record = token_owner_record_cookie
            .voter_weight_record
            .as_ref()
            .map(|voter_weight_record| voter_weight_record.address);

        let mut create_token_governance_ix = create_token_governance(
            &self.program_id,
            &realm_cookie.address,
            &governed_token_cookie.address,
            &governed_token_cookie.token_owner.pubkey(),
            &token_owner_record_cookie.address,
            &self.bench.payer.pubkey(),
            &token_owner_record_cookie.token_owner.pubkey(),
            voter_weight_record,
            config.clone(),
            governed_token_cookie.transfer_token_owner,
        );

        instruction_override(&mut create_token_governance_ix);

        let default_signers = &[
            &governed_token_cookie.token_owner,
            &token_owner_record_cookie.token_owner,
        ];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[create_token_governance_ix], Some(signers))
            .await?;

        let account = GovernanceV2 {
            account_type: GovernanceAccountType::TokenGovernanceV2,
            realm: realm_cookie.address,
            governed_account: governed_token_cookie.address,
            config,
            required_signatories_count: 0,
            reserved1: Default::default(),
            reserved_v2: Default::default(),
            active_proposal_count: 0,
        };

        let token_governance_address = get_token_governance_address(
            &self.program_id,
            &realm_cookie.address,
            &governed_token_cookie.address,
        );

        Ok(GovernanceCookie {
            address: token_governance_address,
            account,
            next_proposal_index: 0,
        })
    }

    #[allow(dead_code)]
    pub async fn with_proposal(
        &mut self,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        governance_cookie: &mut GovernanceCookie,
    ) -> Result<ProposalCookie, TransportError> {
        self.with_proposal_using_instruction(
            token_owner_record_cookie,
            governance_cookie,
            NopOverride,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_multi_option_proposal(
        &mut self,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        governance_cookie: &mut GovernanceCookie,
        options: Vec<String>,
        use_deny_option: bool,
        vote_type: VoteType,
    ) -> Result<ProposalCookie, TransportError> {
        self.with_proposal_using_instruction_impl(
            token_owner_record_cookie,
            governance_cookie,
            options,
            use_deny_option,
            vote_type,
            NopOverride,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_signed_off_proposal(
        &mut self,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        governance_cookie: &mut GovernanceCookie,
    ) -> Result<ProposalCookie, TransportError> {
        let proposal_cookie = self
            .with_proposal(token_owner_record_cookie, governance_cookie)
            .await?;

        let signatory_record_cookie = self
            .with_signatory(&proposal_cookie, token_owner_record_cookie)
            .await?;

        self.sign_off_proposal(&proposal_cookie, &signatory_record_cookie)
            .await?;

        Ok(proposal_cookie)
    }

    #[allow(dead_code)]
    pub async fn with_proposal_using_instruction<F: Fn(&mut Instruction)>(
        &mut self,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        governance_cookie: &mut GovernanceCookie,
        instruction_override: F,
    ) -> Result<ProposalCookie, TransportError> {
        let options = vec!["Yes".to_string()];

        self.with_proposal_using_instruction_impl(
            token_owner_record_cookie,
            governance_cookie,
            options,
            true,
            VoteType::SingleChoice,
            instruction_override,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_proposal_using_instruction_impl<F: Fn(&mut Instruction)>(
        &mut self,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        governance_cookie: &mut GovernanceCookie,
        options: Vec<String>,
        use_deny_option: bool,
        vote_type: VoteType,
        instruction_override: F,
    ) -> Result<ProposalCookie, TransportError> {
        let proposal_index = governance_cookie.next_proposal_index;
        governance_cookie.next_proposal_index += 1;

        let proposal_key = Pubkey::new_unique();

        let name = format!("Proposal #{}", proposal_index);

        let description_link = "Proposal Description".to_string();

        let governance_authority = token_owner_record_cookie.get_governance_authority();

        let voter_weight_record = token_owner_record_cookie
            .voter_weight_record
            .as_ref()
            .map(|voter_weight_record| voter_weight_record.address);

        let mut create_proposal_transaction = create_proposal(
            &self.program_id,
            &governance_cookie.address,
            &token_owner_record_cookie.address,
            &governance_authority.pubkey(),
            &self.bench.payer.pubkey(),
            voter_weight_record,
            &governance_cookie.account.realm,
            name.clone(),
            description_link.clone(),
            &token_owner_record_cookie.account.governing_token_mint,
            vote_type.clone(),
            options.clone(),
            use_deny_option,
            &proposal_key,
        );

        instruction_override(&mut create_proposal_transaction);

        self.bench
            .process_transaction(
                &[create_proposal_transaction],
                Some(&[governance_authority]),
            )
            .await?;

        let clock = self.bench.get_clock().await;

        let proposal_options: Vec<ProposalOption> = options
            .iter()
            .map(|o| ProposalOption {
                label: o.to_string(),
                vote_weight: 0,
                vote_result: OptionVoteResult::None,
                transactions_executed_count: 0,
                transactions_count: 0,
                transactions_next_index: 0,
            })
            .collect();

        let deny_vote_weight = if use_deny_option { Some(0) } else { None };

        let account = ProposalV2 {
            account_type: GovernanceAccountType::ProposalV2,
            description_link,
            name: name.clone(),
            governance: governance_cookie.address,
            governing_token_mint: token_owner_record_cookie.account.governing_token_mint,
            state: ProposalState::Draft,
            signatories_count: 0,

            start_voting_at: None,
            draft_at: clock.unix_timestamp,
            signing_off_at: None,

            voting_at: None,
            voting_at_slot: None,
            voting_completed_at: None,
            executing_at: None,
            closed_at: None,

            token_owner_record: token_owner_record_cookie.address,
            signatories_signed_off_count: 0,

            vote_type,
            options: proposal_options,
            deny_vote_weight,

            veto_vote_weight: 0,
            abstain_vote_weight: None,

            execution_flags: InstructionExecutionFlags::None,
            max_vote_weight: None,
            max_voting_time: None,
            vote_threshold: None,

            reserved: [0; 64],
            reserved1: 0,
        };

        let proposal_address = get_proposal_address(
            &self.program_id,
            &governance_cookie.address,
            &token_owner_record_cookie.account.governing_token_mint,
            &proposal_key,
        );

        Ok(ProposalCookie {
            address: proposal_address,
            account,
            proposal_owner: governance_authority.pubkey(),
            realm: governance_cookie.account.realm,
        })
    }

    #[allow(dead_code)]
    pub async fn with_signatory(
        &mut self,
        proposal_cookie: &ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
    ) -> Result<SignatoryRecordCookie, TransportError> {
        let signatory = Keypair::new();

        let add_signatory_ix = add_signatory(
            &self.program_id,
            &proposal_cookie.account.governance,
            &proposal_cookie.address,
            &spl_governance::instruction::AddSignatoryAuthority::ProposalOwner {
                governance_authority: token_owner_record_cookie.token_owner.pubkey(),
                token_owner_record: token_owner_record_cookie.address,
            },
            // &token_owner_record_cookie.token_owner.pubkey(),
            &self.bench.payer.pubkey(),
            &signatory.pubkey(),
        );

        self.bench
            .process_transaction(
                &[add_signatory_ix],
                Some(&[&token_owner_record_cookie.token_owner]),
            )
            .await?;

        let signatory_record_address = get_signatory_record_address(
            &self.program_id,
            &proposal_cookie.address,
            &signatory.pubkey(),
        );

        let signatory_record_data = SignatoryRecordV2 {
            account_type: GovernanceAccountType::SignatoryRecordV2,
            proposal: proposal_cookie.address,
            signatory: signatory.pubkey(),
            signed_off: false,
            reserved_v2: [0; 8],
        };

        let signatory_record_cookie = SignatoryRecordCookie {
            address: signatory_record_address,
            account: signatory_record_data,
            signatory,
        };

        Ok(signatory_record_cookie)
    }

    #[allow(dead_code)]
    pub async fn sign_off_proposal_by_owner(
        &mut self,
        proposal_cookie: &ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
    ) -> Result<(), TransportError> {
        self.sign_off_proposal_by_owner_using_instruction(
            proposal_cookie,
            token_owner_record_cookie,
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn sign_off_proposal_by_owner_using_instruction<F: Fn(&mut Instruction)>(
        &mut self,
        proposal_cookie: &ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<(), TransportError> {
        let mut sign_off_proposal_ix = sign_off_proposal(
            &self.program_id,
            &proposal_cookie.realm,
            &proposal_cookie.account.governance,
            &proposal_cookie.address,
            &token_owner_record_cookie.account.governing_token_owner,
            Some(&token_owner_record_cookie.address),
        );

        instruction_override(&mut sign_off_proposal_ix);

        let default_signers = &[&token_owner_record_cookie.token_owner];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[sign_off_proposal_ix], Some(signers))
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn sign_off_proposal(
        &mut self,
        proposal_cookie: &ProposalCookie,
        signatory_record_cookie: &SignatoryRecordCookie,
    ) -> Result<(), TransportError> {
        self.sign_off_proposal_using_instruction(
            proposal_cookie,
            signatory_record_cookie,
            NopOverride,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn sign_off_proposal_using_instruction<F: Fn(&mut Instruction)>(
        &mut self,
        proposal_cookie: &ProposalCookie,
        signatory_record_cookie: &SignatoryRecordCookie,
        instruction_override: F,
        signers_override: Option<&[&Keypair]>,
    ) -> Result<(), TransportError> {
        let mut sign_off_proposal_ix = sign_off_proposal(
            &self.program_id,
            &proposal_cookie.realm,
            &proposal_cookie.account.governance,
            &proposal_cookie.address,
            &signatory_record_cookie.signatory.pubkey(),
            None,
        );

        instruction_override(&mut sign_off_proposal_ix);

        let default_signers = &[&signatory_record_cookie.signatory];
        let signers = signers_override.unwrap_or(default_signers);

        self.bench
            .process_transaction(&[sign_off_proposal_ix], Some(signers))
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn finalize_vote(
        &mut self,
        realm_cookie: &RealmCookie,
        proposal_cookie: &ProposalCookie,
        max_voter_weight_record_cookie: Option<MaxVoterWeightRecordCookie>,
    ) -> Result<(), TransportError> {
        let max_voter_weight_record = max_voter_weight_record_cookie.map(|rc| rc.address);

        let finalize_vote_ix = finalize_vote(
            &self.program_id,
            &realm_cookie.address,
            &proposal_cookie.account.governance,
            &proposal_cookie.address,
            &proposal_cookie.account.token_owner_record,
            &proposal_cookie.account.governing_token_mint,
            max_voter_weight_record,
        );

        self.bench
            .process_transaction(&[finalize_vote_ix], None)
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn cancel_proposal(
        &mut self,
        proposal_cookie: &ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
    ) -> Result<(), TransportError> {
        let cancel_proposal_transaction = cancel_proposal(
            &self.program_id,
            &proposal_cookie.realm,
            &proposal_cookie.account.governance,
            &proposal_cookie.address,
            &token_owner_record_cookie.address,
            &token_owner_record_cookie.token_owner.pubkey(),
        );

        self.bench
            .process_transaction(
                &[cancel_proposal_transaction],
                Some(&[&token_owner_record_cookie.token_owner]),
            )
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn with_cast_yes_no_vote(
        &mut self,
        proposal_cookie: &ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        yes_no_vote: YesNoVote,
    ) -> Result<VoteRecordCookie, TransportError> {
        let vote = match yes_no_vote {
            YesNoVote::Yes => Vote::Approve(vec![VoteChoice {
                rank: 0,
                weight_percentage: 100,
            }]),
            YesNoVote::No => Vote::Deny,
        };

        self.with_cast_vote(proposal_cookie, token_owner_record_cookie, vote)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_cast_vote(
        &mut self,
        proposal_cookie: &ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        vote: Vote,
    ) -> Result<VoteRecordCookie, TransportError> {
        let (voter_weight_record, vote_amount) =
            if let Some(voter_weight_record) = &token_owner_record_cookie.voter_weight_record {
                (
                    Some(voter_weight_record.address),
                    voter_weight_record.account.voter_weight,
                )
            } else {
                (
                    None,
                    token_owner_record_cookie
                        .account
                        .governing_token_deposit_amount,
                )
            };

        let max_voter_weight_record = token_owner_record_cookie
            .max_voter_weight_record
            .as_ref()
            .map(|max_voter_weight_record| max_voter_weight_record.address);

        let cast_vote_ix = cast_vote(
            &self.program_id,
            &token_owner_record_cookie.account.realm,
            &proposal_cookie.account.governance,
            &proposal_cookie.address,
            &proposal_cookie.account.token_owner_record,
            &token_owner_record_cookie.address,
            &token_owner_record_cookie.token_owner.pubkey(),
            &token_owner_record_cookie.account.governing_token_mint,
            &self.bench.payer.pubkey(),
            voter_weight_record,
            max_voter_weight_record,
            vote.clone(),
        );

        self.bench
            .process_transaction(
                &[cast_vote_ix],
                Some(&[&token_owner_record_cookie.token_owner]),
            )
            .await?;

        let account = VoteRecordV2 {
            account_type: GovernanceAccountType::VoteRecordV2,
            proposal: proposal_cookie.address,
            governing_token_owner: token_owner_record_cookie.token_owner.pubkey(),
            vote,
            voter_weight: vote_amount,
            is_relinquished: false,
            reserved_v2: [0; 8],
        };

        let vote_record_cookie = VoteRecordCookie {
            address: get_vote_record_address(
                &self.program_id,
                &proposal_cookie.address,
                &token_owner_record_cookie.address,
            ),
            account,
        };

        Ok(vote_record_cookie)
    }

    #[allow(dead_code)]
    pub async fn with_set_governance_config_transaction(
        &mut self,
        proposal_cookie: &mut ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        governance_config: &GovernanceConfig,
    ) -> Result<ProposalTransactionCookie, TransportError> {
        let mut set_governance_config_ix = set_governance_config(
            &self.program_id,
            &proposal_cookie.account.governance,
            governance_config.clone(),
        );

        self.with_proposal_transaction(
            proposal_cookie,
            token_owner_record_cookie,
            0,
            None,
            &mut set_governance_config_ix,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_mint_tokens_transaction(
        &mut self,
        governed_mint_cookie: &GovernedMintCookie,
        proposal_cookie: &mut ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        option_index: u8,
        index: Option<u16>,
        hold_up_time: Option<u32>,
    ) -> Result<ProposalTransactionCookie, TransportError> {
        let token_account_keypair = Keypair::new();
        self.bench
            .create_empty_token_account(
                &token_account_keypair,
                &governed_mint_cookie.address,
                &self.bench.payer.pubkey(),
            )
            .await;

        let mut instruction = spl_token::instruction::mint_to(
            &spl_token::id(),
            &governed_mint_cookie.address,
            &token_account_keypair.pubkey(),
            &proposal_cookie.account.governance,
            &[],
            10,
        )
        .unwrap();

        self.with_proposal_transaction(
            proposal_cookie,
            token_owner_record_cookie,
            option_index,
            index,
            &mut instruction,
            hold_up_time,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_transfer_tokens_transaction(
        &mut self,
        governed_token_cookie: &GovernedTokenCookie,
        proposal_cookie: &mut ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        index: Option<u16>,
    ) -> Result<ProposalTransactionCookie, TransportError> {
        let token_account_keypair = Keypair::new();
        self.bench
            .create_empty_token_account(
                &token_account_keypair,
                &governed_token_cookie.token_mint,
                &self.bench.payer.pubkey(),
            )
            .await;

        let mut instruction = spl_token::instruction::transfer(
            &spl_token::id(),
            &governed_token_cookie.address,
            &token_account_keypair.pubkey(),
            &proposal_cookie.account.governance,
            &[],
            15,
        )
        .unwrap();

        self.with_proposal_transaction(
            proposal_cookie,
            token_owner_record_cookie,
            0,
            index,
            &mut instruction,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_native_transfer_transaction(
        &mut self,
        governance_cookie: &GovernanceCookie,
        proposal_cookie: &mut ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        to_wallet_cookie: &WalletCookie,
        lamports: u64,
    ) -> Result<ProposalTransactionCookie, TransportError> {
        let treasury_address =
            get_native_treasury_address(&self.program_id, &governance_cookie.address);

        let mut transfer_ix =
            system_instruction::transfer(&treasury_address, &to_wallet_cookie.address, lamports);

        self.with_proposal_transaction(
            proposal_cookie,
            token_owner_record_cookie,
            0,
            None,
            &mut transfer_ix,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_upgrade_program_transaction(
        &mut self,
        governance_cookie: &GovernanceCookie,
        proposal_cookie: &mut ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
    ) -> Result<ProposalTransactionCookie, TransportError> {
        let program_buffer_keypair = Keypair::new();
        let buffer_authority_keypair = Keypair::new();

        // Load solana_bpf_rust_upgraded program taken from solana test programs
        let path_buf = find_file("solana_bpf_rust_upgraded.so").unwrap();
        let program_data = read_file(path_buf);

        let program_buffer_rent =
            self.bench
                .rent
                .minimum_balance(UpgradeableLoaderState::size_of_programdata(
                    program_data.len(),
                ));

        let mut instructions = bpf_loader_upgradeable::create_buffer(
            &self.bench.payer.pubkey(),
            &program_buffer_keypair.pubkey(),
            &buffer_authority_keypair.pubkey(),
            program_buffer_rent,
            program_data.len(),
        )
        .unwrap();

        let chunk_size = 800;

        for (chunk, i) in program_data.chunks(chunk_size).zip(0..) {
            instructions.push(bpf_loader_upgradeable::write(
                &program_buffer_keypair.pubkey(),
                &buffer_authority_keypair.pubkey(),
                (i * chunk_size) as u32,
                chunk.to_vec(),
            ));
        }

        instructions.push(bpf_loader_upgradeable::set_buffer_authority(
            &program_buffer_keypair.pubkey(),
            &buffer_authority_keypair.pubkey(),
            &governance_cookie.address,
        ));

        self.bench
            .process_transaction(
                &instructions[..],
                Some(&[&program_buffer_keypair, &buffer_authority_keypair]),
            )
            .await
            .unwrap();

        let mut upgrade_ix = bpf_loader_upgradeable::upgrade(
            &governance_cookie.account.governed_account,
            &program_buffer_keypair.pubkey(),
            &governance_cookie.address,
            &governance_cookie.address,
        );

        self.with_proposal_transaction(
            proposal_cookie,
            token_owner_record_cookie,
            0,
            None,
            &mut upgrade_ix,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_nop_transaction(
        &mut self,
        proposal_cookie: &mut ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        option_index: u8,
        index: Option<u16>,
    ) -> Result<ProposalTransactionCookie, TransportError> {
        // Create NOP instruction as a placeholder
        // Note: The actual instruction is irrelevant because we do not execute it in tests
        let mut instruction = Instruction {
            program_id: Pubkey::new_unique(),
            accounts: vec![],
            data: vec![],
        };

        self.with_proposal_transaction(
            proposal_cookie,
            token_owner_record_cookie,
            option_index,
            index,
            &mut instruction,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_proposal_transaction(
        &mut self,
        proposal_cookie: &mut ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        option_index: u8,
        index: Option<u16>,
        instruction: &mut Instruction,
        hold_up_time: Option<u32>,
    ) -> Result<ProposalTransactionCookie, TransportError> {
        let hold_up_time = hold_up_time.unwrap_or(15);

        let instruction_data: InstructionData = instruction.clone().into();
        let yes_option = &mut proposal_cookie.account.options[0];

        let transaction_index = index.unwrap_or(yes_option.transactions_next_index);

        yes_option.transactions_next_index += 1;

        let insert_transaction_ix = insert_transaction(
            &self.program_id,
            &proposal_cookie.account.governance,
            &proposal_cookie.address,
            &token_owner_record_cookie.address,
            &token_owner_record_cookie.token_owner.pubkey(),
            &self.bench.payer.pubkey(),
            option_index,
            transaction_index,
            hold_up_time,
            vec![instruction_data.clone()],
        );

        self.bench
            .process_transaction(
                &[insert_transaction_ix],
                Some(&[&token_owner_record_cookie.token_owner]),
            )
            .await?;

        let proposal_transaction_address = get_proposal_transaction_address(
            &self.program_id,
            &proposal_cookie.address,
            &option_index.to_le_bytes(),
            &transaction_index.to_le_bytes(),
        );

        let proposal_transaction_data = ProposalTransactionV2 {
            account_type: GovernanceAccountType::ProposalTransactionV2,
            option_index,
            transaction_index,
            hold_up_time,
            instructions: vec![instruction_data],
            executed_at: None,
            execution_status: TransactionExecutionStatus::None,
            proposal: proposal_cookie.address,
            reserved_v2: [0; 8],
        };

        instruction.accounts = instruction
            .accounts
            .iter()
            .map(|a| AccountMeta {
                pubkey: a.pubkey,
                is_signer: false, // Remove signer since the Governance account PDA will be signing the instruction for us
                is_writable: a.is_writable,
            })
            .collect();

        let proposal_transaction_cookie = ProposalTransactionCookie {
            address: proposal_transaction_address,
            account: proposal_transaction_data,
            instruction: instruction.clone(),
        };

        Ok(proposal_transaction_cookie)
    }

    #[allow(dead_code)]
    pub async fn remove_transaction(
        &mut self,
        proposal_cookie: &mut ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        proposal_transaction_cookie: &ProposalTransactionCookie,
    ) -> Result<(), TransportError> {
        let remove_transaction_ix = remove_transaction(
            &self.program_id,
            &proposal_cookie.address,
            &token_owner_record_cookie.address,
            &token_owner_record_cookie.token_owner.pubkey(),
            &proposal_transaction_cookie.address,
            &self.bench.payer.pubkey(),
        );

        self.bench
            .process_transaction(
                &[remove_transaction_ix],
                Some(&[&token_owner_record_cookie.token_owner]),
            )
            .await?;

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn execute_proposal_transaction(
        &mut self,
        proposal_cookie: &ProposalCookie,
        proposal_transaction_cookie: &ProposalTransactionCookie,
    ) -> Result<(), TransportError> {
        let execute_proposal_transaction_ix = execute_transaction(
            &self.program_id,
            &proposal_cookie.account.governance,
            &proposal_cookie.address,
            &proposal_transaction_cookie.address,
            &proposal_transaction_cookie.instruction.program_id,
            &proposal_transaction_cookie.instruction.accounts,
        );

        self.bench
            .process_transaction(&[execute_proposal_transaction_ix], None)
            .await
    }

    #[allow(dead_code)]
    pub async fn flag_transaction_error(
        &mut self,
        proposal_cookie: &ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        proposal_transaction_cookie: &ProposalTransactionCookie,
    ) -> Result<(), TransportError> {
        let governance_authority = token_owner_record_cookie.get_governance_authority();

        let flag_transaction_error_ix = flag_transaction_error(
            &self.program_id,
            &proposal_cookie.address,
            &proposal_cookie.account.token_owner_record,
            &governance_authority.pubkey(),
            &proposal_transaction_cookie.address,
        );

        self.bench
            .process_transaction(&[flag_transaction_error_ix], Some(&[governance_authority]))
            .await
    }

    #[allow(dead_code)]
    pub async fn get_token_owner_record_account(&mut self, address: &Pubkey) -> TokenOwnerRecordV2 {
        self.bench
            .get_borsh_account::<TokenOwnerRecordV2>(address)
            .await
    }

    #[allow(dead_code)]
    pub async fn get_program_metadata_account(&mut self, address: &Pubkey) -> ProgramMetadata {
        self.bench
            .get_borsh_account::<ProgramMetadata>(address)
            .await
    }

    #[allow(dead_code)]
    pub async fn get_native_treasury_account(&mut self, address: &Pubkey) -> NativeTreasury {
        self.bench
            .get_borsh_account::<NativeTreasury>(address)
            .await
    }

    #[allow(dead_code)]
    pub async fn get_realm_account(&mut self, realm_address: &Pubkey) -> RealmV2 {
        self.bench.get_borsh_account::<RealmV2>(realm_address).await
    }

    #[allow(dead_code)]
    pub async fn get_realm_config_data(
        &mut self,
        realm_config_address: &Pubkey,
    ) -> RealmConfigAccount {
        self.bench
            .get_borsh_account::<RealmConfigAccount>(realm_config_address)
            .await
    }

    #[allow(dead_code)]
    pub async fn get_governance_account(&mut self, governance_address: &Pubkey) -> GovernanceV2 {
        self.bench
            .get_borsh_account::<GovernanceV2>(governance_address)
            .await
    }

    #[allow(dead_code)]
    pub async fn get_proposal_account(&mut self, proposal_address: &Pubkey) -> ProposalV2 {
        self.bench
            .get_borsh_account::<ProposalV2>(proposal_address)
            .await
    }

    #[allow(dead_code)]
    pub async fn get_vote_record_account(&mut self, vote_record_address: &Pubkey) -> VoteRecordV2 {
        self.bench
            .get_borsh_account::<VoteRecordV2>(vote_record_address)
            .await
    }

    #[allow(dead_code)]
    pub async fn get_proposal_transaction_account(
        &mut self,
        proposal_transaction_address: &Pubkey,
    ) -> ProposalTransactionV2 {
        self.bench
            .get_borsh_account::<ProposalTransactionV2>(proposal_transaction_address)
            .await
    }

    #[allow(dead_code)]
    pub async fn get_signatory_record_account(
        &mut self,
        proposal_address: &Pubkey,
    ) -> SignatoryRecordV2 {
        self.bench
            .get_borsh_account::<SignatoryRecordV2>(proposal_address)
            .await
    }

    #[allow(dead_code, clippy::await_holding_refcell_ref)]
    async fn get_packed_account<T: Pack + IsInitialized>(&mut self, address: &Pubkey) -> T {
        self.bench
            .context
            .borrow_mut()
            .banks_client
            .get_packed_account_data::<T>(*address)
            .await
            .unwrap()
    }

    #[allow(dead_code)]
    pub async fn get_upgradable_loader_account(
        &mut self,
        address: &Pubkey,
    ) -> UpgradeableLoaderState {
        self.bench.get_bincode_account(address).await
    }

    #[allow(dead_code)]
    pub async fn get_token_account(&mut self, address: &Pubkey) -> spl_token::state::Account {
        self.get_packed_account(address).await
    }

    #[allow(dead_code)]
    pub async fn get_mint_account(&mut self, address: &Pubkey) -> spl_token::state::Mint {
        self.get_packed_account(address).await
    }

    /// ----------- VoterWeight Addin -----------------------------

    #[allow(dead_code)]
    pub async fn with_voter_weight_addin_record(
        &mut self,
        token_owner_record_cookie: &mut TokenOwnerRecordCookie,
    ) -> Result<VoterWeightRecordCookie, TransportError> {
        self.with_voter_weight_addin_record_impl(token_owner_record_cookie, 120, None, None, None)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_voter_weight_addin_record_impl(
        &mut self,
        token_owner_record_cookie: &mut TokenOwnerRecordCookie,
        voter_weight: u64,
        voter_weight_expiry: Option<Slot>,
        weight_action: Option<VoterWeightAction>,
        weight_action_target: Option<Pubkey>,
    ) -> Result<VoterWeightRecordCookie, TransportError> {
        let voter_weight_record_account = Keypair::new();

        let setup_voter_weight_record = setup_voter_weight_record(
            &self.community_voter_weight_addin.unwrap(),
            &token_owner_record_cookie.account.realm,
            &token_owner_record_cookie.account.governing_token_mint,
            &token_owner_record_cookie.account.governing_token_owner,
            &voter_weight_record_account.pubkey(),
            &self.bench.payer.pubkey(),
            voter_weight,
            voter_weight_expiry,
            weight_action.clone(),
            weight_action_target,
        );

        self.bench
            .process_transaction(
                &[setup_voter_weight_record],
                Some(&[&voter_weight_record_account]),
            )
            .await?;

        let voter_weight_record_cookie = VoterWeightRecordCookie {
            address: voter_weight_record_account.pubkey(),
            account: VoterWeightRecord {
                account_discriminator: VoterWeightRecord::ACCOUNT_DISCRIMINATOR,
                realm: token_owner_record_cookie.account.realm,
                governing_token_mint: token_owner_record_cookie.account.governing_token_mint,
                governing_token_owner: token_owner_record_cookie.account.governing_token_owner,
                voter_weight,
                voter_weight_expiry,
                weight_action,
                weight_action_target,
                reserved: [0; 8],
            },
        };

        token_owner_record_cookie.voter_weight_record = Some(voter_weight_record_cookie.clone());

        Ok(voter_weight_record_cookie)
    }

    #[allow(dead_code)]
    pub async fn with_max_voter_weight_addin_record(
        &mut self,
        token_owner_record_cookie: &mut TokenOwnerRecordCookie,
    ) -> Result<MaxVoterWeightRecordCookie, TransportError> {
        self.with_max_voter_weight_addin_record_impl(token_owner_record_cookie, 200, None)
            .await
    }

    #[allow(dead_code)]
    pub async fn with_max_voter_weight_addin_record_impl(
        &mut self,
        token_owner_record_cookie: &mut TokenOwnerRecordCookie,
        max_voter_weight: u64,
        max_voter_weight_expiry: Option<Slot>,
    ) -> Result<MaxVoterWeightRecordCookie, TransportError> {
        let max_voter_weight_record_account = Keypair::new();

        let setup_voter_weight_record = setup_max_voter_weight_record(
            &self.max_community_voter_weight_addin.unwrap(),
            &token_owner_record_cookie.account.realm,
            &token_owner_record_cookie.account.governing_token_mint,
            &max_voter_weight_record_account.pubkey(),
            &self.bench.payer.pubkey(),
            max_voter_weight,
            max_voter_weight_expiry,
        );

        self.bench
            .process_transaction(
                &[setup_voter_weight_record],
                Some(&[&max_voter_weight_record_account]),
            )
            .await?;

        let max_voter_weight_record_cookie = MaxVoterWeightRecordCookie {
            address: max_voter_weight_record_account.pubkey(),
            account: MaxVoterWeightRecord {
                account_discriminator: MaxVoterWeightRecord::ACCOUNT_DISCRIMINATOR,
                realm: token_owner_record_cookie.account.realm,
                governing_token_mint: token_owner_record_cookie.account.governing_token_mint,
                max_voter_weight,
                max_voter_weight_expiry,
                reserved: [0; 8],
            },
        };

        token_owner_record_cookie.max_voter_weight_record =
            Some(max_voter_weight_record_cookie.clone());

        Ok(max_voter_weight_record_cookie)
    }

    #[allow(dead_code)]
    pub async fn advance_clock_past_timestamp(&mut self, unix_timestamp: UnixTimestamp) {
        let mut clock = self.bench.get_clock().await;
        let mut n = 1;

        while clock.unix_timestamp <= unix_timestamp {
            // Since the exact time is not deterministic keep wrapping by arbitrary 400 slots until we pass the requested timestamp
            self.bench
                .context
                .borrow_mut()
                .warp_to_slot(clock.slot + n * 400)
                .unwrap();

            n += 1;
            clock = self.bench.get_clock().await;
        }
    }

    #[allow(dead_code)]
    pub async fn advance_clock_by_min_timespan(&mut self, time_span: u64) {
        let clock = self.bench.get_clock().await;
        self.advance_clock_past_timestamp(clock.unix_timestamp + (time_span as i64))
            .await;
    }

    #[allow(dead_code)]
    pub async fn with_governance_required_signatory_transaction(
        &mut self,
        proposal_cookie: &mut ProposalCookie,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        governance: &GovernanceCookie,
        signatory: &Pubkey,
    ) -> Result<ProposalTransactionCookie, TransportError> {
        let mut gwr_ix = add_required_signatory(
            &self.program_id,
            &governance.address,
            &self.bench.payer.pubkey(),
            signatory,
        );

        self.with_proposal_transaction(
            proposal_cookie,
            token_owner_record_cookie,
            0,
            None,
            &mut gwr_ix,
            None,
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn with_governance_required_signatory(
        &mut self,
        token_owner_record_cookie: &TokenOwnerRecordCookie,
        governance_cookie: &mut GovernanceCookie,
        signatory: &Pubkey,
    ) -> Result<(), TransportError> {
        let mut proposal_cookie = self
            .with_proposal(token_owner_record_cookie, governance_cookie)
            .await
            .unwrap();

        let signatory_record_cookie = self
            .with_signatory(&proposal_cookie, token_owner_record_cookie)
            .await
            .unwrap();

        let proposal_transaction_cookie = self
            .with_governance_required_signatory_transaction(
                &mut proposal_cookie,
                token_owner_record_cookie,
                governance_cookie,
                signatory,
            )
            .await
            .unwrap();

        self.sign_off_proposal(&proposal_cookie, &signatory_record_cookie)
            .await
            .unwrap();

        self.with_cast_yes_no_vote(&proposal_cookie, token_owner_record_cookie, YesNoVote::Yes)
            .await
            .unwrap();

        // Advance timestamp past hold_up_time
        self.advance_clock_by_min_timespan(proposal_transaction_cookie.account.hold_up_time as u64)
            .await;

        self.execute_proposal_transaction(&proposal_cookie, &proposal_transaction_cookie)
            .await
            .unwrap();

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn with_signatory_record_for_required_signatory(
        &mut self,
        proposal_cookie: &ProposalCookie,
        governance: &GovernanceCookie,
        signatory: &Pubkey,
    ) -> Result<SignatoryRecordCookieWithoutKeypair, TransportError> {
        let create_signatory_record_ix = add_signatory(
            &self.program_id,
            &governance.address,
            &proposal_cookie.address,
            &AddSignatoryAuthority::None,
            &self.bench.payer.pubkey(),
            signatory,
        );

        self.bench
            .process_transaction(&[create_signatory_record_ix], Some(&[]))
            .await?;

        let signatory_record_address =
            get_signatory_record_address(&self.program_id, &proposal_cookie.address, signatory);

        let signatory_record_data = SignatoryRecordV2 {
            account_type: GovernanceAccountType::SignatoryRecordV2,
            proposal: proposal_cookie.address,
            signatory: *signatory,
            signed_off: false,
            reserved_v2: [0; 8],
        };

        let signatory_record_cookie = SignatoryRecordCookieWithoutKeypair {
            address: signatory_record_address,
            account: signatory_record_data,
        };

        Ok(signatory_record_cookie)
    }
}
