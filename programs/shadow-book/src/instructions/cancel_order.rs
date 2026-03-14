use anchor_lang::prelude::*;

use crate::state::MarketState;

/// Cancels an open order on mainnet (Phase A only).
///
/// **Chunk A** — mainnet instruction.
///
/// # Arguments
/// * `order_id` — The order to cancel.
/// * `side` — 0 = Buy, 1 = Sell.
///
/// # Validation
/// - Market must NOT be delegated.
/// - Signer must be the order's trader.
/// - Order must have status `Open`.
pub fn handler(_ctx: Context<CancelOrder>, _order_id: u64, _side: u8) -> Result<()> {
    // TODO (Chunk A): Implement
    // 1. require!(!market.is_delegated)
    // 2. Find order by order_id and side
    // 3. require!(order.trader == trader.key())
    // 4. require!(order.status == Open)
    // 5. remove_order(market, order_id, side)
    Ok(())
}

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    pub trader: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,
}
