use anchor_lang::prelude::*;

use crate::errors::ShadowBookError;
use crate::helpers::{find_order, remove_order};
use crate::state::{MarketState, OrderStatus, Side};

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
pub fn handler(ctx: Context<CancelOrder>, order_id: u64, side: u8) -> Result<()> {
    let mut market = ctx.accounts.market.load_mut()?;

    require!(!market.is_delegated(), ShadowBookError::MarketDelegated);

    let parsed_side = match side {
        0 => Side::Buy,
        1 => Side::Sell,
        _ => return err!(ShadowBookError::InvalidSide),
    };

    let order_index =
        find_order(&market, order_id, parsed_side).ok_or(ShadowBookError::OrderNotFound)?;

    let order = match parsed_side {
        Side::Buy => &market.bids[order_index],
        Side::Sell => &market.asks[order_index],
    };

    require!(
        order.trader_pubkey() == ctx.accounts.trader.key(),
        ShadowBookError::UnauthorizedOrderAccess
    );

    require!(
        order.status == OrderStatus::Open as u8,
        ShadowBookError::OrderNotOpen
    );

    remove_order(&mut market, order_id, parsed_side)?;

    Ok(())
}

#[derive(Accounts)]
pub struct CancelOrder<'info> {
    pub trader: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,
}
