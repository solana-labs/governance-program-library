use anchor_lang::prelude::*;

use crate::state::VoterWeightRecord;

#[derive(Accounts)]
pub struct CreateVoterWeightRecord<'info> {
    // #[account(
    //     init,
    //     seeds = [ b"voter-weight-record".as_ref()],
    //     bump,
    //     payer = payer
    // )]
    // pub voter_weight_record: Account<'info, VoterWeightRecord>,
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}
