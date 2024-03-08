use anchor_lang::prelude::Pubkey;
// Add the generic voter weight trait to TokenOwnerRecord structs
use crate::generic_voter_weight::GenericVoterWeight;
use spl_governance::state::token_owner_record::TokenOwnerRecordV2;
use spl_governance_addin_api::voter_weight::VoterWeightAction;

impl GenericVoterWeight for TokenOwnerRecordV2 {
    fn get_governing_token_mint(&self) -> Pubkey {
        self.governing_token_mint
    }

    fn get_governing_token_owner(&self) -> Pubkey {
        self.governing_token_owner
    }

    fn get_realm(&self) -> Pubkey {
        self.realm
    }

    fn get_voter_weight(&self) -> u64 {
        self.governing_token_deposit_amount
    }

    fn get_weight_action(&self) -> Option<VoterWeightAction> {
        None
    }

    fn get_weight_action_target(&self) -> Option<Pubkey> {
        None
    }

    fn get_voter_weight_expiry(&self) -> Option<u64> {
        None
    }
}
