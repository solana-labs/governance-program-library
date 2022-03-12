pub use registrar::*;
pub mod registrar;

pub use collection_config::*;
pub mod collection_config;

pub use nft_vote_record::*;
pub mod nft_vote_record;

use crate::max_voter_weight_record;
use crate::voter_weight_record;

// Generate a VoterWeightRecord and MaxVoterWeightRecord Anchor wrapper, owned by the current program
// The Records accounts  are unique in that they are defined by the SPL governance program as ABI
// but they are actually owned by this program
voter_weight_record!(crate::ID);
max_voter_weight_record!(crate::ID);
