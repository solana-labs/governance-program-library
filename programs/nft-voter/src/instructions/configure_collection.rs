use anchor_lang::{
    account,
    prelude::{Context, Signer},
    Accounts,
};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use spl_governance::state::realm;
use mpl_token_metadata::state::{CollectionDetails,Metadata};


use crate::error::NftVoterError;
use crate::state::{max_voter_weight_record::MaxVoterWeightRecord, CollectionConfig, Registrar};

/// Configures NFT voting collection which defines what NFTs can be used for governances
/// and what weight they have
/// The instruction updates MaxVoterWeightRecord which is used by spl-gov to determine max voting power
/// used to calculate voting quorum    
#[derive(Accounts)]
pub struct ConfigureCollection<'info> {
    /// Registrar for which we configure this Collection
    #[account(mut)]
    pub registrar: Account<'info, Registrar>,

    #[account(
       address = registrar.realm @ NftVoterError::InvalidRealmForRegistrar,
       owner = registrar.governance_program_id
    )]
    /// CHECK: Owned by spl-governance instance specified in registrar.governance_program_id
    pub realm: UncheckedAccount<'info>,

    /// Authority of the Realm must sign and match Realm.authority
    pub realm_authority: Signer<'info>,

    // Collection which is going to be used for voting
    pub collection: Account<'info, Mint>,

    #[account(seeds = [b"metadata".as_ref(), mpl_token_metadata::ID.as_ref(), collection.key().as_ref()], bump, seeds::program = mpl_token_metadata::ID, owner = mpl_token_metadata::ID)]
    pub metadata: AccountInfo<'info>,

    #[account(
        mut,
        constraint = max_voter_weight_record.realm == registrar.realm
        @ NftVoterError::InvalidMaxVoterWeightRecordRealm,

        constraint = max_voter_weight_record.governing_token_mint == registrar.governing_token_mint
        @ NftVoterError::InvalidMaxVoterWeightRecordMint,
    )]
    pub max_voter_weight_record: Account<'info, MaxVoterWeightRecord>,
}

pub fn configure_collection(
    ctx: Context<ConfigureCollection>,
    weight: u64,
    mut size: u64,
) -> Result<()> {
    let collection = &ctx.accounts.collection;

    let collection_metadata = Metadata::from_account_info(&ctx.accounts.metadata);
    
    // Set size to the collection details config if available
    let retrieved_size = if let Some(details) = collection_metadata.unwrap().collection_details {
        match details {
            CollectionDetails::V1 { size } => size,
        }
    } else {
        size
    };

    size = retrieved_size;
    
    require!(size > 0, NftVoterError::InvalidCollectionSize);
    
    let registrar = &mut ctx.accounts.registrar;

    let realm = realm::get_realm_data_for_governing_token_mint(
        &registrar.governance_program_id,
        &ctx.accounts.realm,
        &registrar.governing_token_mint,
    )?;

    require!(
        realm.authority.unwrap() == ctx.accounts.realm_authority.key(),
        NftVoterError::InvalidRealmAuthority
    );

    // Changes to the collections config can accidentally tip the scales for outstanding proposals and hence we disallow it
    if realm.voting_proposal_count > 0 {
        return err!(NftVoterError::CannotConfigureCollectionWithVotingProposals);
    }


    let collection_config = CollectionConfig {
        collection: collection.key(),
        weight,
        reserved: [0; 8],
        size,
    };

    let collection_idx = registrar
        .collection_configs
        .iter()
        .position(|cc| cc.collection == collection.key());

    if let Some(collection_idx) = collection_idx {
        registrar.collection_configs[collection_idx] = collection_config;
    } else {
        // Note: In the current runtime version push() would throw an error if we exceed
        // max_collections specified when the Registrar was created
        registrar.collection_configs.push(collection_config);
    }

    // TODO: if weight == 0 then remove the collection from config
    // Currently if weight is set to 0 then the collection won't be removed but it won't have any governance power

    // Update MaxVoterWeightRecord based on max voting power of the collections
    let max_voter_weight_record = &mut ctx.accounts.max_voter_weight_record;

    max_voter_weight_record.max_voter_weight = registrar
        .collection_configs
        .iter()
        .try_fold(0u64, |sum, cc| sum.checked_add(cc.get_max_weight()))
        .unwrap();

    // The weight never expires and only changes when collections are configured
    max_voter_weight_record.max_voter_weight_expiry = None;

    Ok(())
}
