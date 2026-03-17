use anchor_lang::prelude::*;

use crate::errors::ShadowBookError;
use crate::helpers::find_order;
use crate::state::{MarketState, OrderStatus, Side};

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
pub fn handler(ctx: Context<SubmitOrderSize>, order_id: u64, size: u64) -> Result<()> {
    let mut market = ctx.accounts.market.load_mut()?;

    require!(market.is_delegated(), ShadowBookError::MarketNotDelegated);

    let mut order_idx = None;
    let mut order_side = Side::Buy;

    if let Some(idx) = find_order(&market, order_id, Side::Buy) {
        order_idx = Some(idx);
        order_side = Side::Buy;
    } else if let Some(idx) = find_order(&market, order_id, Side::Sell) {
        order_idx = Some(idx);
        order_side = Side::Sell;
    }

    let idx = order_idx.ok_or(ShadowBookError::OrderNotFound)?;

    let order = match order_side {
        Side::Buy => &mut market.bids[idx],
        Side::Sell => &mut market.asks[idx],
    };

    require!(
        order.trader_pubkey() == ctx.accounts.trader.key(),
        ShadowBookError::UnauthorizedOrderAccess
    );

    require!(
        order.status == OrderStatus::Open as u8,
        ShadowBookError::OrderNotOpen
    );

    require!(size > 0, ShadowBookError::ZeroSize);

    order.size = size;

    msg!("Order {} size submitted privately", order_id);

    Ok(())
}

#[derive(Accounts)]
pub struct SubmitOrderSize<'info> {
    pub trader: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,
}
