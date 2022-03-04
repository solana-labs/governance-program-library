use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Invalid authority provided")]
  InvalidAuthority,
  #[msg("Msg")]
  RatesFull,
  #[msg("Invalid RA")]
  InvalidRealmAuthority,
}
