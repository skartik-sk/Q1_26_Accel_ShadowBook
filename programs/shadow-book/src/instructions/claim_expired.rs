use anchor_lang::prelude::*;

use crate::state::MarketState;

/// Removes expired orders from the order book.
///
/// **Chunk A** — mainnet instruction.
///
/// # Notes
/// - Permissionless — anyone can call (crank target).
/// - Bounded: max `MAX_EXPIRED_CLEANUP_PER_CALL` removals per invocation.
/// - Market must NOT be delegated.
pub fn handler(_ctx: Context<ClaimExpired>) -> Result<()> {
    // TODO (Chunk A): Implement
    // 1. require!(!market.is_delegated)
    // 2. let now = Clock::get()?.unix_timestamp;
    // 3. let removed = cleanup_expired(&mut market, now, MAX_EXPIRED_CLEANUP_PER_CALL);
    // 4. Log removed count
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimExpired<'info> {
    /// Anyone can call this instruction.
    pub signer: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,
}
