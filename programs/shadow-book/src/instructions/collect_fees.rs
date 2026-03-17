use anchor_lang::prelude::*;
use anchor_spl::token::{Token};
use crate::state::MarketState;

pub fn handler(_ctx: Context<CollectFees>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct CollectFees<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
