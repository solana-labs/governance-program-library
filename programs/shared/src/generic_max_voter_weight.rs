use crate::mint::MintMaxVoterWeight;
use anchor_lang::prelude::*;
use enum_dispatch::enum_dispatch;
use spl_governance_addin_api::max_voter_weight::MaxVoterWeightRecord;

/// A generic trait representing a max voter weight,
/// that can be passed as an input into the plugin
#[enum_dispatch]
pub trait GenericMaxVoterWeight {
    fn get_governing_token_mint(&self) -> Pubkey;
    // fn get_governing_token_owner(&self) -> Pubkey;
    // fn get_realm(&self) -> Pubkey;
    fn get_max_voter_weight(&self) -> u64;
    // fn get_weight_action(&self) -> Option<VoterWeightAction>;
    // fn get_weight_action_target(&self) -> Option<Pubkey>;
    fn get_max_voter_weight_expiry(&self) -> Option<u64>;
}

#[enum_dispatch(GenericMaxVoterWeight)]
pub enum GenericMaxVoterWeightEnum {
    MaxVoterWeightRecord(MaxVoterWeightRecord),
    Mint(MintMaxVoterWeight),
}

// the "official" on-chain max voter weight record has a discriminator field
// when a predecessor voter weight is provided, it uses this struct
// We add the GenericMaxVoterWeight trait here to hide this from the rest of the code.
impl GenericMaxVoterWeight for MaxVoterWeightRecord {
    fn get_governing_token_mint(&self) -> Pubkey {
        self.governing_token_mint
    }
    //
    // fn get_governing_token_owner(&self) -> Pubkey {
    //     self.governing_token_owner
    // }
    //
    // fn get_realm(&self) -> Pubkey {
    //     self.realm
    // }

    fn get_max_voter_weight(&self) -> u64 {
        self.max_voter_weight
    }

    // The GenericVoterWeight interface expects a crate-defined VoterWeightAction.
    // This is identical to spl_governance_addin_api::voter_weight::VoterWeightAction, but added here
    // so that Anchor will create the mapping correctly in the IDL.
    // This function converts the spl_governance_addin_api::voter_weight::VoterWeightAction to the
    // crate-defined VoterWeightAction by mapping the enum values by integer.
    // Note - it is imperative that the two enums stay in sync to avoid errors here.
    // fn get_weight_action(&self) -> Option<VoterWeightAction> {
    //     self.weight_action.clone()
    // .clone()
    // .map(|x| FromPrimitive::from_u32(x as u32).unwrap())
    // }

    // fn get_weight_action_target(&self) -> Option<Pubkey> {
    //     self.weight_action_target
    // }
    //
    fn get_max_voter_weight_expiry(&self) -> Option<u64> {
        self.max_voter_weight_expiry
    }
}
