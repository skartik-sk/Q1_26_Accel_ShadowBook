use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::delegate;
use ephemeral_rollups_sdk::cpi::DelegateConfig;

use crate::constants::{COMMIT_FREQUENCY_MS, DEVNET_TEE_VALIDATOR, MARKET_SEED};
use crate::errors::ShadowBookError;
use crate::state::MarketState;

/// Delegates the `MarketState` to the TEE validator, transitioning from
/// Phase A (collection) to Phase B (execution).
///
/// **Chunk C** — mainnet instruction.
///
/// # Flow
/// 1. Validate market has ≥1 bid AND ≥1 ask.
/// 2. Delegate `MarketState` to TEE validator via `delegate_account()`.
/// 3. Set `is_delegated = true`, `delegated_at = clock.unix_timestamp`.
///
/// # Notes
/// - Permissionless — anyone can call (incentivized by keeper reward on settle).
/// - Uses `#[delegate]` macro from `ephemeral-rollups-sdk`.
pub fn handler(ctx: Context<DelegateMarket>) -> Result<()> {
    let mut market = ctx.accounts.market.load_mut()?;

    // 1. require!(!market.is_delegated)
    require!(!market.is_delegated(), ShadowBookError::MarketDelegated);

    // 2. require!(market.bid_count >= 1 && market.ask_count >= 1)
    require!(
        market.bid_count >= 1 && market.ask_count >= 1,
        ShadowBookError::InsufficientOrdersToDelegate
    );

    // Update state to indicate delegation
    // 5. market.is_delegated = 1
    market.is_delegated = 1;
    // 6. market.delegated_at = clock.unix_timestamp
    market.delegated_at = Clock::get()?.unix_timestamp;

    let mint_a = market.mint_a;
    let mint_b = market.mint_b;
    let bump = market.bump;

    // Drop the mutable borrow of the AccountLoader before CPI to avoid RefCell panics
    drop(market);

    let signer_seeds: &[&[u8]] = &[MARKET_SEED, &mint_a, &mint_b, &[bump]];

    // 4. delegate_account() CPI with DelegateConfig
    ctx.accounts.delegate_market(
        &ctx.accounts.payer,
        signer_seeds,
        DelegateConfig {
            validator: Some(DEVNET_TEE_VALIDATOR),
            commit_frequency_ms: COMMIT_FREQUENCY_MS,
            ..Default::default()
        },
    )?;

    msg!("Market delegated to PER (Phase B execution started)");

    Ok(())
}

#[delegate]
#[derive(Accounts)]
pub struct DelegateMarket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, del)]
    pub market: AccountLoader<'info, MarketState>,

    pub system_program: Program<'info, System>,
}
