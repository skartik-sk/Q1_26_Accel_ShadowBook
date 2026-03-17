use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::commit_and_undelegate_accounts;

use crate::constants::{MAX_MATCHES, MAX_MATCHES_PER_CALL, ORACLE_SANITY_BAND_BPS};
use crate::errors::ShadowBookError;
use crate::helpers::{is_within_oracle_band, read_oracle_price};
use crate::state::{MarketState, OrderStatus};

/// Matching engine — finds crossing orders and fills them at midpoint price.
///
/// **Chunk C** — PER-only crank instruction (Phase B of epoch).
///
/// # Algorithm
/// 1. Read oracle price from Pyth Lazer PDA.
/// 2. Walk asks (lowest price first).
/// 3. For each open ask with size > 0, find best crossing bid.
/// 4. Fill at midpoint: `(bid.price + ask.price) / 2`.
/// 5. Oracle sanity check: reject if fill deviates >5% from oracle.
/// 6. Write `MatchResult`, set both orders to `Matched`.
/// 7. After scanning, `commit_and_undelegate_accounts`.
///
/// # Notes
/// - Permissionless crank. Program logic reads the book inside TEE — the
///   crank caller never sees order data.
/// - Bounded: max `MAX_MATCHES_PER_CALL` matches per invocation.
/// - Uses `#[ephemeral]` + `#[commit]` macros.
/// - v1: Full fills only (skip if sizes don't match).
pub fn handler(ctx: Context<MatchOrders>) -> Result<()> {
    let mut market = ctx.accounts.market.load_mut()?;

    require!(market.is_delegated(), ShadowBookError::MarketNotDelegated);

    let oracle_data = ctx.accounts.oracle_feed.try_borrow_data()?;
    let oracle_price = read_oracle_price(&oracle_data)? as u64;
    drop(oracle_data);

    let mut matches = 0;

    let mut i = 0; // ask index
    while i < market.ask_count as usize && matches < MAX_MATCHES_PER_CALL {
        let mut ask = market.asks[i];

        if ask.status == OrderStatus::Open as u8 && ask.size > 0 {
            // Find best crossing bid (bids are sorted price DESC, so we start at 0)
            for j in 0..market.bid_count as usize {
                let mut bid = market.bids[j];

                if bid.status == OrderStatus::Open as u8
                    && bid.size == ask.size
                    && bid.price >= ask.price
                {
                    let fill_price = (bid.price + ask.price) / 2;

                    if is_within_oracle_band(fill_price, oracle_price, ORACLE_SANITY_BAND_BPS) {
                        let match_idx = market.match_count as usize;
                        if match_idx >= MAX_MATCHES {
                            break; // Match results buffer is full
                        }

                        // Write MatchResult
                        let mut match_result: crate::state::MatchResult =
                            bytemuck::Zeroable::zeroed();
                        match_result.buyer = bid.trader;
                        match_result.seller = ask.trader;
                        match_result.price = fill_price;
                        match_result.size = ask.size;
                        match_result.settled = 0;

                        market.match_results[match_idx] = match_result;
                        market.match_count += 1;
                        matches += 1;

                        // Mark orders as matched
                        bid.status = OrderStatus::Matched as u8;
                        bid.matched_price = fill_price;
                        market.bids[j] = bid;

                        ask.status = OrderStatus::Matched as u8;
                        ask.matched_price = fill_price;
                        market.asks[i] = ask;

                        break; // Move to next ask
                    }
                }
            }
        }

        i += 1;
    }

    // Set delegation state back to false
    market.is_delegated = 0;
    market.delegated_at = 0;

    // Drop the mutable borrow of the AccountLoader before CPI
    drop(market);

    let market_info = ctx.accounts.market.to_account_info();
    let payer_info = ctx.accounts.payer.to_account_info();
    let magic_context = ctx.accounts.magic_context.to_account_info();
    let magic_program = ctx.accounts.magic_program.to_account_info();

    commit_and_undelegate_accounts(
        &payer_info,
        vec![&market_info],
        &magic_context,
        &magic_program,
    )?;

    msg!("Matched {} orders and undelegated market", matches);

    Ok(())
}

#[commit]
#[derive(Accounts)]
pub struct MatchOrders<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,

    /// CHECK: Read-only oracle feed for Pyth Lazer PDA
    pub oracle_feed: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
