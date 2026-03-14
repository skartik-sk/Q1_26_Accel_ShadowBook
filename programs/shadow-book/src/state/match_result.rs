use anchor_lang::prelude::*;

// ---------------------------------------------------------------------------
// MatchResult (88 bytes, 8-byte aligned)
// ---------------------------------------------------------------------------

/// Result of a successful order match. Written by `match_orders` inside TEE,
/// read by `settle` on mainnet to execute SPL token transfers.
#[zero_copy]
#[repr(C)]
pub struct MatchResult {
    /// Buy-side trader pubkey.
    pub buyer: [u8; 32],

    /// Sell-side trader pubkey.
    pub seller: [u8; 32],

    /// Midpoint fill price in smallest quote-token units.
    pub price: u64,

    /// Fill size in smallest base-token units.
    pub size: u64,

    /// 0 = pending, 1 = settled.
    pub settled: u8,

    pub _pad: [u8; 7],
}

impl MatchResult {
    pub const SIZE: usize = 88;

    pub fn is_settled(&self) -> bool {
        self.settled != 0
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0 && self.price == 0
    }
}
