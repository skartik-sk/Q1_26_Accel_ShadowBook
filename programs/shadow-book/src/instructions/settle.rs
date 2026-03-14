use anchor_lang::prelude::*;

use crate::state::MarketState;

/// Settles matched trades by executing SPL token transfers.
///
/// **Chunk B** — mainnet instruction (Phase C of epoch).
///
/// # Flow
/// For each unsettled `MatchResult`:
/// 1. Transfer `size` of token_a: buyer's EATA → seller's EATA.
/// 2. Transfer `size * price` of token_b: seller's EATA → buyer's EATA.
/// 3. Deduct `fee_rate_bps` from both sides, credit fee vault.
/// 4. If `keeper_reward_bps > 0`, credit keeper (tx signer).
/// 5. Mark `settled = true`.
///
/// # Notes
/// - Permissionless — anyone can call (incentivized by keeper reward).
/// - Bounded: max `MAX_SETTLEMENTS_PER_CALL` per invocation.
/// - Market must NOT be delegated (match results must be committed first).
pub fn handler(_ctx: Context<Settle>) -> Result<()> {
    // TODO (Chunk B): Implement
    // 1. require!(!market.is_delegated)
    // 2. Iterate match_results[0..match_count]
    // 3. For each where settled == 0: execute transfers, deduct fees
    // 4. Mark settled = 1
    Ok(())
}

#[derive(Accounts)]
pub struct Settle<'info> {
    #[account(mut)]
    pub keeper: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,

    // TODO (Chunk B): Add accounts
    // - buyer/seller EATAs (remaining accounts, dynamically resolved)
    // - vault ATAs for both mints
    // - fee vault EATA
    // - token_program
    // - system_program

    pub system_program: Program<'info, System>,
}
