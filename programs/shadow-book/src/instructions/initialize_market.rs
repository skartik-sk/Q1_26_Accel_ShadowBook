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
    _ctx: Context<InitializeMarket>,
    _fee_rate_bps: u16,
    _keeper_reward_bps: u16,
    _oracle_feed_id: [u8; 32],
) -> Result<()> {
    // TODO (Chunk A): Implement
    // 1. Validate fee_rate_bps <= 1000 (max 10%)
    // 2. Populate all MarketState fields
    // 3. Set bump from ctx.bumps
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
