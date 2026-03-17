use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::ephemeral;

pub mod constants;
pub mod errors;
pub mod helpers;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("6jQK4BKU7Rnj5Xd5HHXN9MoAjnWxyaRQHoeokqAQFGiD");

#[ephemeral]
#[program]
pub mod shadow_book {
    use super::*;

    // -----------------------------------------------------------------------
    // Mainnet instructions
    // -----------------------------------------------------------------------

    /// Create a new market for a token pair.
    pub fn initialize_market(
        ctx: Context<InitializeMarket>,
        fee_rate_bps: u16,
        keeper_reward_bps: u16,
        oracle_feed_id: [u8; 32],
    ) -> Result<()> {
        instructions::initialize_market::handler(
            ctx,
            fee_rate_bps,
            keeper_reward_bps,
            oracle_feed_id,
        )
    }

    /// Deposit SPL tokens into the market vault.
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::handler(ctx, amount)
    }

    /// Withdraw SPL tokens from the market vault.
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw::handler(ctx, amount)
    }

    /// Place a new order (side + price only; size submitted privately in TEE).
    pub fn create_order(ctx: Context<CreateOrder>, side: u8, price: u64) -> Result<()> {
        instructions::create_order::handler(ctx, side, price)
    }

    /// Cancel an open order on mainnet.
    pub fn cancel_order(ctx: Context<CancelOrder>, order_id: u64, side: u8) -> Result<()> {
        instructions::cancel_order::handler(ctx, order_id, side)
    }

    /// Delegate the market to PER for private matching.
    pub fn delegate_market(ctx: Context<DelegateMarket>) -> Result<()> {
        instructions::delegate_market::handler(ctx)
    }

    /// Settle matched trades — execute SPL token transfers.
    pub fn settle(ctx: Context<Settle>) -> Result<()> {
        instructions::settle::handler(ctx)
    }

    /// Remove expired orders from the book (permissionless crank).
    pub fn claim_expired(ctx: Context<ClaimExpired>) -> Result<()> {
        instructions::claim_expired::handler(ctx)
    }

    /// Withdraw accumulated fees (authority only).
    pub fn collect_fees(ctx: Context<CollectFees>) -> Result<()> {
        instructions::collect_fees::handler(ctx)
    }

    // -----------------------------------------------------------------------
    // PER instructions (run inside TEE)
    // -----------------------------------------------------------------------

    /// Submit the hidden order size inside the TEE.
    pub fn submit_order_size(
        ctx: Context<SubmitOrderSize>,
        order_id: u64,
        size: u64,
    ) -> Result<()> {
        instructions::submit_order_size::handler(ctx, order_id, size)
    }

    /// Match crossing orders inside the TEE (permissionless crank).
    pub fn match_orders(ctx: Context<MatchOrders>) -> Result<()> {
        instructions::match_orders::handler(ctx)
    }

    /// Cancel an order from inside the TEE.
    pub fn cancel_order_per(ctx: Context<CancelOrderPer>, order_id: u64, side: u8) -> Result<()> {
        instructions::cancel_order_per::handler(ctx, order_id, side)
    }
}
