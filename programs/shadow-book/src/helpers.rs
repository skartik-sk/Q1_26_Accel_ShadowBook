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

    #[test]
    fn test_insert_bid_sorting() {
        let mut market = MarketState::zeroed();
        let o1 = make_order(1, Side::Buy, 100, 10);
        let o2 = make_order(2, Side::Buy, 120, 20);
        let o3 = make_order(3, Side::Buy, 100, 5);

        assert_eq!(insert_bid(&mut market, o1).unwrap(), 0);
        assert_eq!(insert_bid(&mut market, o2).unwrap(), 0);
        assert_eq!(insert_bid(&mut market, o3).unwrap(), 1);

        assert_eq!(market.bids[0].order_id, 2); // 120
        assert_eq!(market.bids[1].order_id, 3); // 100, t=5
        assert_eq!(market.bids[2].order_id, 1); // 100, t=10
        assert_eq!(market.bid_count, 3);
    }

    #[test]
    fn test_insert_ask_sorting() {
        let mut market = MarketState::zeroed();
        let o1 = make_order(1, Side::Sell, 100, 10);
        let o2 = make_order(2, Side::Sell, 80, 20);
        let o3 = make_order(3, Side::Sell, 100, 5);

        assert_eq!(insert_ask(&mut market, o1).unwrap(), 0);
        assert_eq!(insert_ask(&mut market, o2).unwrap(), 0);
        assert_eq!(insert_ask(&mut market, o3).unwrap(), 1);

        assert_eq!(market.asks[0].order_id, 2); // 80
        assert_eq!(market.asks[1].order_id, 3); // 100, t=5
        assert_eq!(market.asks[2].order_id, 1); // 100, t=10
        assert_eq!(market.ask_count, 3);
    }

    #[test]
    fn test_order_book_full() {
        let mut market = MarketState::zeroed();
        market.bid_count = crate::constants::MAX_ORDERS as u16;
        let o = make_order(1, Side::Buy, 100, 10);
        assert_eq!(
            insert_bid(&mut market, o).unwrap_err(),
            crate::errors::ShadowBookError::OrderBookFull.into()
        );
    }

    #[test]
    fn test_find_order() {
        let mut market = MarketState::zeroed();
        insert_bid(&mut market, make_order(1, Side::Buy, 100, 10)).unwrap();
        insert_bid(&mut market, make_order(2, Side::Buy, 120, 20)).unwrap();

        assert_eq!(find_order(&market, 2, Side::Buy), Some(0)); // 120
        assert_eq!(find_order(&market, 1, Side::Buy), Some(1)); // 100
        assert_eq!(find_order(&market, 3, Side::Buy), None);
        assert_eq!(find_order(&market, 2, Side::Sell), None);
    }

    #[test]
    fn test_remove_order() {
        let mut market = MarketState::zeroed();
        insert_ask(&mut market, make_order(1, Side::Sell, 100, 10)).unwrap();
        insert_ask(&mut market, make_order(2, Side::Sell, 120, 20)).unwrap();
        insert_ask(&mut market, make_order(3, Side::Sell, 110, 15)).unwrap(); // order: 1, 3, 2

        let removed = remove_order(&mut market, 3, Side::Sell).unwrap();
        assert_eq!(removed.order_id, 3);
        assert_eq!(market.ask_count, 2);
        assert_eq!(market.asks[0].order_id, 1);
        assert_eq!(market.asks[1].order_id, 2);
    }

    #[test]
    fn test_remove_order_not_found() {
        let mut market = MarketState::zeroed();
        assert_eq!(
            remove_order(&mut market, 1, Side::Buy).unwrap_err(),
            crate::errors::ShadowBookError::OrderNotFound.into()
        );
    }

    #[test]
    fn test_cleanup_expired() {
        let mut market = MarketState::zeroed();
        let mut o1 = make_order(1, Side::Buy, 100, 10);
        o1.expires_at = 100;
        let mut o2 = make_order(2, Side::Buy, 90, 20);
        o2.expires_at = 200;
        let mut o3 = make_order(3, Side::Buy, 80, 30);
        o3.expires_at = 300;

        insert_bid(&mut market, o1).unwrap();
        insert_bid(&mut market, o2).unwrap();
        insert_bid(&mut market, o3).unwrap();

        // Time is 250, o1 and o2 should be expired. But limit max_removals to 1.
        let removed = cleanup_expired(&mut market, 250, 1);
        assert_eq!(removed, 1);
        assert_eq!(market.bid_count, 2);
        assert_eq!(market.bids[0].order_id, 2); // o2 now at index 0

        // Clean again without limit
        let removed2 = cleanup_expired(&mut market, 250, 10);
        assert_eq!(removed2, 1);
        assert_eq!(market.bid_count, 1);
        assert_eq!(market.bids[0].order_id, 3);
    }

    #[test]
    fn test_is_within_oracle_band_edges() {
        // 0 oracle returns true
        assert!(is_within_oracle_band(1000, 0, 500));

        // exact boundary
        assert!(is_within_oracle_band(1050, 1000, 500));
        assert!(!is_within_oracle_band(1051, 1000, 500));
    }

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
