use solana_program::pubkey::Pubkey;
use spl_governance::state::{
    enums::MintMaxVoterWeightSource,
    realm::{GoverningTokenConfigAccountArgs, GoverningTokenConfigArgs, RealmConfigArgs},
    realm_config::GoverningTokenConfig,
};

#[derive(Clone, Debug, PartialEq)]
pub struct SetRealmConfigArgs {
    pub realm_config_args: RealmConfigArgs,
    pub community_voter_weight_addin: Option<Pubkey>,
    pub max_community_voter_weight_addin: Option<Pubkey>,
}

impl Default for SetRealmConfigArgs {
    fn default() -> Self {
        let realm_config_args = RealmConfigArgs {
            use_council_mint: true,

            community_mint_max_voter_weight_source: MintMaxVoterWeightSource::SupplyFraction(100),
            min_community_weight_to_create_governance: 10,

            community_token_config_args: GoverningTokenConfigArgs {
                use_voter_weight_addin: false,
                use_max_voter_weight_addin: false,
                token_type: spl_governance::state::realm_config::GoverningTokenType::Liquid,
            },

            council_token_config_args: GoverningTokenConfigArgs {
                use_voter_weight_addin: false,
                use_max_voter_weight_addin: false,
                token_type: spl_governance::state::realm_config::GoverningTokenType::Dormant,
            },
        };

        Self {
            realm_config_args,
            community_voter_weight_addin: None,
            max_community_voter_weight_addin: None,
        }
    }
}

impl SetRealmConfigArgs {
    pub fn community(&self) -> GoverningTokenConfigAccountArgs {
        GoverningTokenConfigAccountArgs {
            voter_weight_addin: self.community_voter_weight_addin,
            max_voter_weight_addin: self.max_community_voter_weight_addin,
            token_type: self
                .realm_config_args
                .community_token_config_args
                .token_type
                .clone(),
        }
    }

    pub fn community_args(&self) -> GoverningTokenConfig {
        GoverningTokenConfig {
            token_type: self
                .realm_config_args
                .community_token_config_args
                .token_type
                .clone(),
            voter_weight_addin: self.community_voter_weight_addin,
            max_voter_weight_addin: self.max_community_voter_weight_addin,
            reserved: Default::default(),
        }
    }

    pub fn council(&self) -> GoverningTokenConfigAccountArgs {
        GoverningTokenConfigAccountArgs {
            voter_weight_addin: None,
            max_voter_weight_addin: None,
            token_type: self
                .realm_config_args
                .council_token_config_args
                .token_type
                .clone(),
        }
    }

    pub fn council_args(&self) -> GoverningTokenConfig {
        GoverningTokenConfig {
            voter_weight_addin: None,
            max_voter_weight_addin: None,
            token_type: self
                .realm_config_args
                .council_token_config_args
                .token_type
                .clone(),
            reserved: Default::default(),
        }
    }
}
