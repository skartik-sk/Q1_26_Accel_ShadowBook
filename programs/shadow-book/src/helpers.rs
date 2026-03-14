use bytemuck::Zeroable;

use crate::constants::MAX_ORDERS;
use crate::errors::ShadowBookError;
use crate::state::{MarketState, Order, OrderStatus, Side};
use anchor_lang::prelude::*;

// ---------------------------------------------------------------------------
// Insertion helpers — maintain sorted invariants
// ---------------------------------------------------------------------------

/// Insert a bid into the bids array, maintaining price DESC then time ASC.
///
/// Returns the index at which the order was inserted.
pub fn insert_bid(market: &mut MarketState, order: Order) -> Result<usize> {
    let count = market.bid_count as usize;
    require!(count < MAX_ORDERS, ShadowBookError::OrderBookFull);

    // Find insertion point via linear scan (binary search optimization welcome).
    let mut pos = count;
    for i in 0..count {
        let existing = &market.bids[i];
        // Higher price first; if equal, earlier timestamp first.
        if order.price > existing.price
            || (order.price == existing.price && order.timestamp < existing.timestamp)
        {
            pos = i;
            break;
        }
    }

    // Shift elements right to make room.
    if pos < count {
        market.bids.copy_within(pos..count, pos + 1);
    }
    market.bids[pos] = order;
    market.bid_count = (count + 1) as u16;

    Ok(pos)
}

/// Insert an ask into the asks array, maintaining price ASC then time ASC.
///
/// Returns the index at which the order was inserted.
pub fn insert_ask(market: &mut MarketState, order: Order) -> Result<usize> {
    let count = market.ask_count as usize;
    require!(count < MAX_ORDERS, ShadowBookError::OrderBookFull);

    let mut pos = count;
    for i in 0..count {
        let existing = &market.asks[i];
        // Lower price first; if equal, earlier timestamp first.
        if order.price < existing.price
            || (order.price == existing.price && order.timestamp < existing.timestamp)
        {
            pos = i;
            break;
        }
    }

    if pos < count {
        market.asks.copy_within(pos..count, pos + 1);
    }
    market.asks[pos] = order;
    market.ask_count = (count + 1) as u16;

    Ok(pos)
}

// ---------------------------------------------------------------------------
// Lookup
// ---------------------------------------------------------------------------

/// Find the index of an order by `order_id` on the given side.
pub fn find_order(market: &MarketState, order_id: u64, side: Side) -> Option<usize> {
    let (orders, count) = match side {
        Side::Buy => (&market.bids[..], market.bid_count as usize),
        Side::Sell => (&market.asks[..], market.ask_count as usize),
    };

    orders[..count].iter().position(|o| o.order_id == order_id)
}

// ---------------------------------------------------------------------------
// Removal
// ---------------------------------------------------------------------------

/// Remove an order by index from the given side. Shifts remaining elements
/// left to keep the array contiguous.
///
/// Returns the removed `Order`.
pub fn remove_order_at(market: &mut MarketState, index: usize, side: Side) -> Result<Order> {
    let (orders, count) = match side {
        Side::Buy => (&mut market.bids[..], &mut market.bid_count),
        Side::Sell => (&mut market.asks[..], &mut market.ask_count),
    };

    let c = *count as usize;
    require!(index < c, ShadowBookError::OrderNotFound);

    let removed = orders[index];
    orders.copy_within(index + 1..c, index);
    // Zero out the now-unused last slot.
    orders[c - 1] = Order::zeroed();
    *count = (c - 1) as u16;

    Ok(removed)
}

/// Remove an order by `order_id`. Combines find + remove.
pub fn remove_order(market: &mut MarketState, order_id: u64, side: Side) -> Result<Order> {
    let index = find_order(market, order_id, side).ok_or(ShadowBookError::OrderNotFound)?;
    remove_order_at(market, index, side)
}

// ---------------------------------------------------------------------------
// Expiry cleanup
// ---------------------------------------------------------------------------

/// Remove expired orders from both sides. Returns the total number removed.
/// Bounded by `max_removals` to cap compute.
pub fn cleanup_expired(market: &mut MarketState, now: i64, max_removals: usize) -> u16 {
    let mut removed: u16 = 0;

    // Clean bids.
    let mut i = 0usize;
    while i < market.bid_count as usize && (removed as usize) < max_removals {
        if market.bids[i].is_open() && market.bids[i].is_expired(now) {
            // Mark cancelled before removal for auditability.
            market.bids[i].status = OrderStatus::Cancelled as u8;
            let _ = remove_order_at(market, i, Side::Buy);
            removed += 1;
            // Don't increment i — the next element shifted into this slot.
        } else {
            i += 1;
        }
    }

    // Clean asks.
    let mut i = 0usize;
    while i < market.ask_count as usize && (removed as usize) < max_removals {
        if market.asks[i].is_open() && market.asks[i].is_expired(now) {
            market.asks[i].status = OrderStatus::Cancelled as u8;
            let _ = remove_order_at(market, i, Side::Sell);
            removed += 1;
        } else {
            i += 1;
        }
    }

    removed
}

// ---------------------------------------------------------------------------
// Oracle helpers
// ---------------------------------------------------------------------------

/// Read the price from a Pyth Lazer price feed account.
///
/// The price is stored as an i64 at byte offset 73.
/// Returns the raw price value (caller must apply exponent scaling).
pub fn read_oracle_price(data: &[u8]) -> Result<i64> {
    require!(data.len() >= 81, ShadowBookError::InvalidOracleData); // 73 + 8
    let price_bytes: [u8; 8] = data[73..81]
        .try_into()
        .map_err(|_| ShadowBookError::InvalidOracleData)?;
    Ok(i64::from_le_bytes(price_bytes))
}

/// Check whether `fill_price` is within the sanity band of `oracle_price`.
///
/// Both prices must be in the same unit and scale.
/// `band_bps` is the maximum allowed deviation in basis points.
pub fn is_within_oracle_band(fill_price: u64, oracle_price: u64, band_bps: u16) -> bool {
    if oracle_price == 0 {
        return true; // Cannot validate against a zero oracle price.
    }

    let band = (oracle_price as u128)
        .checked_mul(band_bps as u128)
        .unwrap_or(u128::MAX)
        / 10_000;

    let lower = oracle_price.saturating_sub(band as u64);
    let upper = oracle_price.saturating_add(band as u64);

    fill_price >= lower && fill_price <= upper
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use bytemuck::Zeroable;

    fn make_order(order_id: u64, side: Side, price: u64, timestamp: i64) -> Order {
        let mut o = Order::zeroed();
        o.order_id = order_id;
        o.side = side as u8;
        o.price = price;
        o.timestamp = timestamp;
        o.status = OrderStatus::Open as u8;
        o.expires_at = timestamp + 3600;
        o
    }

    // TODO (Chunk A): Add comprehensive tests for:
    // - insert_bid maintains price DESC, time ASC invariant
    // - insert_ask maintains price ASC, time ASC invariant
    // - insert into full book returns OrderBookFull
    // - find_order returns correct index / None
    // - remove_order shifts array correctly
    // - remove_order on missing ID returns OrderNotFound
    // - cleanup_expired removes only expired, respects max_removals bound
    // - is_within_oracle_band edge cases (0 oracle, exact boundary, overflow)

    #[test]
    fn test_order_size() {
        assert_eq!(std::mem::size_of::<Order>(), Order::SIZE);
    }

    #[test]
    fn test_oracle_band() {
        // 5% band around 100
        assert!(is_within_oracle_band(100, 100, 500));
        assert!(is_within_oracle_band(105, 100, 500));
        assert!(is_within_oracle_band(95, 100, 500));
        assert!(!is_within_oracle_band(106, 100, 500));
        assert!(!is_within_oracle_band(94, 100, 500));
    }
}
