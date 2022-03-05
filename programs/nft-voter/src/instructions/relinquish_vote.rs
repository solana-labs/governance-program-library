use anchor_lang::prelude::*;

use crate::state::Registrar;

#[derive(Accounts)]
#[instruction(realm:Pubkey, governing_token_mint:Pubkey, governing_token_owner: Pubkey)]
pub struct RelinquishVote<'info> {
    #[account(
        seeds = [b"registrar".as_ref(),realm.as_ref(),  governing_token_mint.as_ref()],
        bump,
    )]
    pub registrar: Account<'info, Registrar>,
}

pub fn relinquish_vote(
    _ctx: Context<RelinquishVote>,
    _realm: Pubkey,
    _governing_token_mint: Pubkey,
    _governing_token_owner: Pubkey,
) -> Result<()> {
    // TODO: Validate registrar vs VoterWeightRecord
    // TODO: Validate governing_token_owner

    // TODO: remove proposal/vote record

    Ok(())
}
