use solana_program::pubkey::Pubkey;
use spl_governance_addin_api::voter_weight::VoterWeightAction;

/// A generic trait representing a voter weight,
/// that can be passed as an input into the plugin
pub trait GenericVoterWeight {
    fn get_governing_token_mint(&self) -> Pubkey;
    fn get_governing_token_owner(&self) -> Pubkey;
    fn get_realm(&self) -> Pubkey;
    fn get_voter_weight(&self) -> u64;
    fn get_weight_action(&self) -> Option<VoterWeightAction>;
    fn get_weight_action_target(&self) -> Option<Pubkey>;
    fn get_vote_expiry(&self) -> Option<u64>;
}
