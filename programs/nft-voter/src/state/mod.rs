pub use registrar::*;
pub mod registrar;

pub use collection_config::*;
pub mod collection_config;

pub use proposal_nft_vote::*;
pub mod proposal_nft_vote;

pub use proposal_nft_vote::*;
pub mod proposal_nft_vote;

use crate::max_voter_weight_record;
use crate::voter_weight_record;

// Generate a VoteWeightRecord Anchor wrapper, owned by the current program.
// VoteWeightRecords are unique in that they are defined by the SPL governance
// program, but they are actually owned by this program.
voter_weight_record!(crate::ID);
max_voter_weight_record!(crate::ID);

// pub fn do() {
//     let wr =
// }
