use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use crate::state::MarketState;
use crate::constants::MARKET_SEED;

pub fn handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(amount > 0, crate::errors::ShadowBookError::ZeroSize);

    let market_state = ctx.accounts.market.load()?;
    let mint_a = market_state.mint_a;
    let mint_b = market_state.mint_b;
    let bump = market_state.bump;
    drop(market_state);

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.trader_token_account.to_account_info(),
        authority: ctx.accounts.market.to_account_info(),
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let seeds = &[MARKET_SEED, &mint_a, &mint_b, &[bump]];
    let signer = &[&seeds[..]];
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    token::transfer(cpi_ctx, amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub trader: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,

    #[account(mut)]
    pub trader_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = market
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
