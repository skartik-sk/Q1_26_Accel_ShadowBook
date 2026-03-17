use anchor_lang::prelude::*;

use crate::constants::ORDER_TTL_SECONDS;
use crate::errors::ShadowBookError;
use crate::helpers::{insert_ask, insert_bid};
use crate::state::{MarketState, Order, OrderStatus, Side};
use bytemuck::Zeroable;

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
pub fn handler(ctx: Context<CreateOrder>, side: u8, price: u64) -> Result<()> {
    let mut market = ctx.accounts.market.load_mut()?;

    require!(!market.is_delegated(), ShadowBookError::MarketDelegated);

    let parsed_side = match side {
        0 => Side::Buy,
        1 => Side::Sell,
        _ => return err!(ShadowBookError::InvalidSide),
    };

    let order_id = market.next_order_id;
    market.next_order_id = market
        .next_order_id
        .checked_add(1)
        .ok_or(ShadowBookError::MathOverflow)?;

    let now = Clock::get()?.unix_timestamp;

    let mut order = Order::zeroed();
    order.trader = ctx.accounts.trader.key().to_bytes();
    order.order_id = order_id;
    order.side = side;
    order.status = OrderStatus::Open as u8;
    order.price = price;
    order.size = 0; // Size is 0 until submitted in TEE
    order.timestamp = now;
    order.expires_at = now.checked_add(ORDER_TTL_SECONDS).unwrap_or(i64::MAX);
    order.matched_price = 0;

    match parsed_side {
        Side::Buy => {
            insert_bid(&mut market, order)?;
        }
        Side::Sell => {
            insert_ask(&mut market, order)?;
        }
    }

    msg!(
        "Order {} created (side: {}, price: {})",
        order_id,
        side,
        price
    );

    Ok(())
}

#[derive(Accounts)]
pub struct CreateOrder<'info> {
    #[account(mut)]
    pub trader: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,

    pub system_program: Program<'info, System>,
}
