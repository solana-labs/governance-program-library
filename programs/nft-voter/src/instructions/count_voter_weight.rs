use anchor_lang::prelude::*;

use crate::state::Registrar;

#[derive(Accounts)]
#[instruction(proposal: Pubkey)]
pub struct CountVoterWeight<'info> {
    /// The NFT voting registrar
    pub registrar: Account<'info, Registrar>,

    /// The token owner whose voter weight is being counted
    pub governing_token_owner: Signer<'info>,

    /// The account which pays for the transaction
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn count_voter_weight<'a, 'b, 'c, 'info>(
    _ctx: Context<'a, 'b, 'c, 'info, CountVoterWeight<'info>>,
    _proposal: Pubkey,
) -> Result<()> {
    Ok(())
}
