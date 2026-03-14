use anchor_lang::prelude::*;

use crate::state::MarketState;

/// Creates a new order in the market's order book.
///
/// **Chunk C** — mainnet instruction (Phase A of epoch).
///
/// Places the order with `size = 0` as a placeholder. The actual size is
/// submitted privately inside the TEE via `submit_order_size`.
///
/// # Arguments
/// * `side` — 0 = Buy, 1 = Sell.
/// * `price` — Limit price in smallest quote-token units per base-token unit.
///
/// # Validation
/// - Market must NOT be delegated (`is_delegated == false`).
/// - Trader must have sufficient EATA balance for worst-case fill.
/// - Order book must not be full on the given side.
pub fn handler(_ctx: Context<CreateOrder>, _side: u8, _price: u64) -> Result<()> {
    // TODO (Chunk C): Implement
    // 1. require!(!market.is_delegated)
    // 2. Build Order { side, price, size: 0, status: Open, timestamp: clock, expires_at, ... }
    // 3. Assign order_id from market.next_order_id++
    // 4. Call insert_bid() or insert_ask() based on side
    Ok(())
}

#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(mut)]
    pub trader: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,

    // TODO (Chunk C): Add accounts
    // - clock: Sysvar<Clock> (or use Clock::get())

    pub system_program: Program<'info, System>,
}
