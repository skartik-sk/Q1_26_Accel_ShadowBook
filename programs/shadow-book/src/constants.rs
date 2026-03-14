use anchor_lang::prelude::*;

// ---------------------------------------------------------------------------
// External Program IDs
// ---------------------------------------------------------------------------

pub const DELEGATION_PROGRAM_ID: Pubkey =
    pubkey!("DELeGGvXpWV2fqJUhqcF5ZSYMS4JTLjteaAMARRSaeSh");

pub const PERMISSION_PROGRAM_ID: Pubkey =
    pubkey!("ACLseoPoyC3cBqoUtkbjZ4aDrkurZW86v19pXz2XQnp1");

pub const MAGIC_PROGRAM_ID: Pubkey =
    pubkey!("Magic11111111111111111111111111111111111111");

pub const MAGIC_CONTEXT_ID: Pubkey =
    pubkey!("MagicContext1111111111111111111111111111111");

pub const ORACLE_PROGRAM_ID: Pubkey =
    pubkey!("PriCems5tHihc6UDXDjzjeawomAwBduWMGAi8ZUjppd");

// ---------------------------------------------------------------------------
// TEE Validators
// ---------------------------------------------------------------------------

pub const DEVNET_TEE_VALIDATOR: Pubkey =
    pubkey!("FnE6VJT5QNZdedZPnCoLsARgBwoE6DeJNjBs2H1gySXA");

pub const MAINNET_TEE_VALIDATOR: Pubkey =
    pubkey!("MTEWGuqxUpYZGFJQcp8tLN7x5v9BSeoFHYWQQ3n3xzo");

// ---------------------------------------------------------------------------
// Order Book Limits
// ---------------------------------------------------------------------------

/// Maximum number of orders per side (bids or asks).
pub const MAX_ORDERS: usize = 256;

/// Maximum number of pending match results.
pub const MAX_MATCHES: usize = 128;

/// Maximum matches processed in a single `match_orders` call.
pub const MAX_MATCHES_PER_CALL: usize = 10;

/// Maximum settlements processed in a single `settle` call.
pub const MAX_SETTLEMENTS_PER_CALL: usize = 10;

/// Maximum expired orders cleaned up in a single `claim_expired` call.
pub const MAX_EXPIRED_CLEANUP_PER_CALL: usize = 10;

// ---------------------------------------------------------------------------
// Timing
// ---------------------------------------------------------------------------

/// Default order time-to-live in seconds (1 hour).
pub const ORDER_TTL_SECONDS: i64 = 3_600;

/// If the market has been delegated longer than this, `force_undelegate` is
/// allowed (Phase 2). In seconds.
pub const DELEGATION_TIMEOUT_SECONDS: i64 = 300;

/// How often the ER validator auto-commits state (milliseconds).
pub const COMMIT_FREQUENCY_MS: u32 = 30_000;

// ---------------------------------------------------------------------------
// Fees
// ---------------------------------------------------------------------------

/// Default trading fee in basis points (0.30%).
pub const DEFAULT_FEE_RATE_BPS: u16 = 30;

/// Default keeper reward — share of the fee paid to whoever calls `settle`.
pub const DEFAULT_KEEPER_REWARD_BPS: u16 = 5;

// ---------------------------------------------------------------------------
// Oracle
// ---------------------------------------------------------------------------

/// Maximum deviation between fill price and oracle price (5%).
pub const ORACLE_SANITY_BAND_BPS: u16 = 500;

/// Byte offset of the price field in a Pyth Lazer price feed account.
pub const ORACLE_PRICE_OFFSET: usize = 73;

// ---------------------------------------------------------------------------
// PDA Seeds
// ---------------------------------------------------------------------------

pub const MARKET_SEED: &[u8] = b"market";
pub const VAULT_SEED: &[u8] = b"vault";
pub const FEE_VAULT_SEED: &[u8] = b"fee_vault";
