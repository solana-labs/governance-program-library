use anchor_lang::prelude::*;
/// Definition of a NFT collection used for the voting
/// 
///
#[account]
#[derive(Default)]
pub struct Collection {
  /// Name of the collection
  pub name: String,
  /// Multiplier for the collection
  pub multiplier: u64,
  /// Collection creator address which will be used to verify NFTs on vote
  pub collection_creator: Pubkey
  
}
