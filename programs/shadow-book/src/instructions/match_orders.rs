use anchor_lang::prelude::*;

use crate::state::MarketState;

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
pub fn handler(_ctx: Context<MatchOrders>) -> Result<()> {
    // TODO (Chunk C): Implement matching algorithm
    // See docs/implementation-spec.md for full pseudocode.
    //
    // After matching:
    // commit_and_undelegate_accounts(
    //     &ctx.accounts.payer,
    //     vec![&ctx.accounts.market.to_account_info()],
    //     &ctx.accounts.magic_context,
    //     &ctx.accounts.magic_program,
    // )?;
    Ok(())
}

#[derive(Accounts)]
pub struct MatchOrders<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,

    // TODO (Chunk C): Add PER accounts
    // - oracle_feed: AccountInfo (Pyth Lazer PDA, read-only)
    // - magic_context: AccountInfo (MAGIC_CONTEXT_ID)
    // - magic_program: Program (MagicProgram)
    //
    // Uses #[commit] macro to auto-inject magic_context + magic_program.
    pub system_program: Program<'info, System>,
}
