use anchor_lang::prelude::*;
/// Definition of a NFT collection used for the voting
/// 
///
#[account]
#[derive(Default)]
pub struct Collection {
  /// Key of the collection used to verify NFTs on vote
  pub key: Pubkey,
  /// Multiplier for the collection
  pub multiplier: u64,
  
}
