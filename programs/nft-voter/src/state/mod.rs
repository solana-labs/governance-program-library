pub use registrar::*;
pub mod registrar;

pub use collection::*;
pub mod collection;

use crate::vote_weight_record;

// Generate a VoteWeightRecord Anchor wrapper, owned by the current program.
// VoteWeightRecords are unique in that they are defined by the SPL governance
// program, but they are actually owned by this program.
vote_weight_record!(crate::ID);

// pub fn do() {
//     let wr =
// }
