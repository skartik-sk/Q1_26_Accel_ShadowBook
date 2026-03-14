use anchor_lang::prelude::*;

use crate::state::MarketState;

/// Writes the actual order size into the delegated `MarketState` inside the TEE.
///
/// **Chunk C** — PER-only instruction (Phase B of epoch).
///
/// This is the critical privacy moment: the size value only exists inside the
/// TEE enclave memory. It is never visible on mainnet until the match result
/// is committed (and even then, only the matched size is public — unmatched
/// sizes remain private).
///
/// # Arguments
/// * `order_id` — The order to update.
/// * `size` — The actual order size in smallest base-token units.
///
/// # Validation
/// - Signer must be the order's trader.
/// - Order must have status `Open`.
/// - `size > 0`.
/// - Market must be delegated (`is_delegated == true`).
pub fn handler(_ctx: Context<SubmitOrderSize>, _order_id: u64, _size: u64) -> Result<()> {
    // TODO (Chunk C): Implement
    // 1. require!(market.is_delegated)
    // 2. Find order by order_id
    // 3. require!(order.trader == trader.key())
    // 4. require!(order.status == Open)
    // 5. require!(size > 0)
    // 6. order.size = size
    Ok(())
}

#[derive(Accounts)]
pub struct SubmitOrderSize<'info> {
    pub trader: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,
    // NOTE: This instruction runs inside PER only.
    // Uses #[ephemeral] module attribute (Chunk C will add this).
}
