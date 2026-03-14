use anchor_lang::prelude::*;

use crate::state::MarketState;

/// Cancels an order from inside the TEE.
///
/// **Chunk C** — PER-only instruction (Phase B of epoch).
///
/// # Flow
/// 1. Validate signer is the order's trader.
/// 2. Remove order from the book.
/// 3. If the book is now empty (no bids AND no asks), commit + undelegate.
/// 4. If the book still has orders, commit only (market stays in PER).
///
/// # Notes
/// - Uses `#[ephemeral]` + `#[commit]` macros.
pub fn handler(_ctx: Context<CancelOrderPer>, _order_id: u64, _side: u8) -> Result<()> {
    // TODO (Chunk C): Implement
    // 1. require!(market.is_delegated)
    // 2. Find and remove order
    // 3. If book empty → commit_and_undelegate_accounts
    // 4. Else → commit_accounts
    Ok(())
}

#[derive(Accounts)]
pub struct CancelOrderPer<'info> {
    pub trader: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,

    // TODO (Chunk C): Add PER accounts
    // - magic_context
    // - magic_program
    pub system_program: Program<'info, System>,
}
