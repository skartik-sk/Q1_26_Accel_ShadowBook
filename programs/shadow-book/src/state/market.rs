use anchor_lang::prelude::*;

use super::match_result::MatchResult;
use super::order::Order;
use crate::constants::{MAX_MATCHES, MAX_ORDERS};

// ---------------------------------------------------------------------------
// MarketState — the core account
// ---------------------------------------------------------------------------

/// Full order book for a single trading pair. Zero-copy for performance.
///
/// Delegated to MagicBlock PER during the execution epoch; lives on mainnet
/// otherwise. Contains bids, asks, and pending match results inline.
///
/// All fields are explicitly aligned to avoid Pod padding violations.
#[account(zero_copy)]
#[repr(C)]
pub struct MarketState {
    // -- Token pair (96 bytes) -----------------------------------------------
    /// Base token mint.
    pub mint_a: [u8; 32],
    /// Quote token mint.
    pub mint_b: [u8; 32],
    /// Market authority — can update fees, collect fee revenue.
    pub authority: [u8; 32],

    // -- Stats (8 bytes) -----------------------------------------------------
    /// Cumulative matched volume (base-token units).
    pub total_volume: u64,

    // -- Fee config (8 bytes: 2 + 2 + 4 pad) ---------------------------------
    /// Trading fee in basis points (e.g. 30 = 0.30%).
    pub fee_rate_bps: u16,
    /// Percentage of the fee paid to whoever calls `settle` (keeper reward).
    pub keeper_reward_bps: u16,
    pub _pad_fees: [u8; 4],

    // -- Oracle (32 bytes) ---------------------------------------------------
    /// Pyth Lazer feed ID for this trading pair.
    pub oracle_feed_id: [u8; 32],

    // -- Order ID counter (8 bytes) ------------------------------------------
    /// Monotonically increasing counter for order IDs.
    pub next_order_id: u64,

    // -- Order book (256 * 104 * 2 = 53,248 bytes) ---------------------------
    /// Buy orders, sorted by price DESC then timestamp ASC.
    pub bids: [Order; MAX_ORDERS],
    /// Sell orders, sorted by price ASC then timestamp ASC.
    pub asks: [Order; MAX_ORDERS],

    // -- Order counts (8 bytes: 2 + 2 + 4 pad) -------------------------------
    /// Number of active bids.
    pub bid_count: u16,
    /// Number of active asks.
    pub ask_count: u16,
    pub _pad_counts: [u8; 4],

    // -- Match results (128 * 88 = 11,264 bytes) -----------------------------
    /// Pending settlement records written by `match_orders`.
    pub match_results: [MatchResult; MAX_MATCHES],

    // -- Match count (8 bytes: 2 + 6 pad) ------------------------------------
    /// Number of active (unsettled + settled) match results.
    pub match_count: u16,
    pub _pad_match: [u8; 6],

    // -- Delegation state (16 bytes: 1 + 7 pad + 8) --------------------------
    /// Whether this market is currently delegated to PER.
    pub is_delegated: u8,
    pub _pad_delegated: [u8; 7],
    /// Unix timestamp when delegation started (for timeout detection).
    pub delegated_at: i64,

    // -- PDA + reserved (8 bytes: 1 bump + 7 reserved) -------------------------
    /// PDA bump seed.
    pub bump: u8,
    /// Reserved/padding.
    pub _reserved: [u8; 7],
}

impl MarketState {
    /// Total on-chain size of this account (for allocation).
    /// 8 bytes discriminator + struct size.
    pub const SPACE: usize = 8 + std::mem::size_of::<Self>();

    pub fn is_delegated(&self) -> bool {
        self.is_delegated != 0
    }

    pub fn mint_a_pubkey(&self) -> Pubkey {
        Pubkey::from(self.mint_a)
    }

    pub fn mint_b_pubkey(&self) -> Pubkey {
        Pubkey::from(self.mint_b)
    }

    pub fn authority_pubkey(&self) -> Pubkey {
        Pubkey::from(self.authority)
    }
}
