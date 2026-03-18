use anchor_lang::prelude::*;

use crate::constants::MAX_EXPIRED_CLEANUP_PER_CALL;
use crate::errors::ShadowBookError;
use crate::helpers::cleanup_expired;
use crate::state::MarketState;

/// Removes expired orders from the order book.
///
/// **Chunk A** — mainnet instruction.
///
/// # Notes
/// - Permissionless — anyone can call (crank target).
/// - Bounded: max `MAX_EXPIRED_CLEANUP_PER_CALL` removals per invocation.
/// - Market must NOT be delegated.
pub fn handler(ctx: Context<ClaimExpired>) -> Result<()> {
    let mut market = ctx.accounts.market.load_mut()?;

    require!(!market.is_delegated(), ShadowBookError::MarketDelegated);

    let now = Clock::get()?.unix_timestamp;

    let removed = cleanup_expired(&mut market, now, MAX_EXPIRED_CLEANUP_PER_CALL);

    msg!("Cleaned up {} expired orders", removed);

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimExpired<'info> {
    /// Anyone can call this instruction.
    pub signer: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,
}
