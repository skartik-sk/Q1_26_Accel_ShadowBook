use anchor_lang::prelude::*;

// ---------------------------------------------------------------------------
// Side
// ---------------------------------------------------------------------------

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub enum Side {
    Buy = 0,
    Sell = 1,
}

// ---------------------------------------------------------------------------
// OrderStatus
// ---------------------------------------------------------------------------

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, AnchorSerialize, AnchorDeserialize)]
pub enum OrderStatus {
    /// Slot is unused.
    Empty = 0,
    /// Order is active and awaiting matching.
    Open = 1,
    /// Order has been matched; pending settlement.
    Matched = 2,
    /// Order was cancelled by the trader.
    Cancelled = 3,
}

// ---------------------------------------------------------------------------
// Order (104 bytes, 8-byte aligned)
// ---------------------------------------------------------------------------

/// A single order in the order book. Stored inline in `MarketState`.
///
/// `size` is written **only inside the TEE** via `submit_order_size`.
/// On mainnet, `size` remains 0 until the match result is committed.
#[zero_copy]
#[derive(Debug)]
#[repr(C)]
pub struct Order {
    /// Owner of this order.
    pub trader: [u8; 32],

    /// Unique monotonic ID assigned at creation.
    pub order_id: u64,

    /// 0 = Buy, 1 = Sell. See `Side`.
    pub side: u8,

    /// 0 = Empty, 1 = Open, 2 = Matched, 3 = Cancelled. See `OrderStatus`.
    pub status: u8,

    pub _pad1: [u8; 6],

    /// Limit price in smallest quote-token units per base-token unit.
    pub price: u64,

    /// Order size in smallest base-token units.
    /// Written inside TEE only — private until commit.
    pub size: u64,

    /// Unix timestamp when the order was created.
    pub timestamp: i64,

    /// Unix timestamp when the order expires.
    pub expires_at: i64,

    /// Fill price set by `match_orders` inside TEE.
    pub matched_price: u64,

    /// Reserved for future fields without realloc.
    pub _reserved: [u8; 16],
}

impl Order {
    pub const SIZE: usize = 104;

    pub fn is_empty(&self) -> bool {
        self.status == OrderStatus::Empty as u8
    }

    pub fn is_open(&self) -> bool {
        self.status == OrderStatus::Open as u8
    }

    pub fn is_expired(&self, now: i64) -> bool {
        self.expires_at > 0 && now >= self.expires_at
    }

    pub fn trader_pubkey(&self) -> Pubkey {
        Pubkey::from(self.trader)
    }
}
