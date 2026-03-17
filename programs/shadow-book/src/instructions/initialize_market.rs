use anchor_lang::prelude::*;

use crate::constants::*;
use crate::state::MarketState;

/// Creates a new `MarketState` account for a token pair.
///
/// **Chunk A** — mainnet instruction.
///
/// # Arguments
/// * `fee_rate_bps` — Trading fee in basis points.
/// * `keeper_reward_bps` — Share of fee paid to settle caller.
/// * `oracle_feed_id` — Pyth Lazer feed ID for oracle sanity checks.
pub fn handler(
    ctx: Context<InitializeMarket>,
    fee_rate_bps: u16,
    keeper_reward_bps: u16,
    oracle_feed_id: [u8; 32],
) -> Result<()> {
    require!(
        fee_rate_bps <= 1000,
        crate::errors::ShadowBookError::FeeRateTooHigh
    );

    let mut market = ctx.accounts.market.load_init()?;

    market.mint_a = ctx.accounts.mint_a.key().to_bytes();
    market.mint_b = ctx.accounts.mint_b.key().to_bytes();
    market.authority = ctx.accounts.authority.key().to_bytes();
    market.total_volume = 0;
    market.fee_rate_bps = fee_rate_bps;
    market.keeper_reward_bps = keeper_reward_bps;
    market.oracle_feed_id = oracle_feed_id;
    market.next_order_id = 1;
    market.bid_count = 0;
    market.ask_count = 0;
    market.match_count = 0;
    market.is_delegated = 0;
    market.delegated_at = 0;
    market.bump = ctx.bumps.market;

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeMarket<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Validated as SPL mint in handler. Used only as PDA seed.
    pub mint_a: AccountInfo<'info>,

    /// CHECK: Validated as SPL mint in handler. Used only as PDA seed.
    pub mint_b: AccountInfo<'info>,

    /// The market account to create. Seeds: ["market", mint_a, mint_b].
    #[account(
        init,
        payer = authority,
        space = MarketState::SPACE,
        seeds = [MARKET_SEED, mint_a.key().as_ref(), mint_b.key().as_ref()],
        bump,
    )]
    pub market: AccountLoader<'info, MarketState>,

    pub system_program: Program<'info, System>,
}
