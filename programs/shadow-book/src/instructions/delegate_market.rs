use anchor_lang::prelude::*;

use crate::state::MarketState;

/// Delegates the `MarketState` to the TEE validator, transitioning from
/// Phase A (collection) to Phase B (execution).
///
/// **Chunk C** — mainnet instruction.
///
/// # Flow
/// 1. Validate market has ≥1 bid AND ≥1 ask.
/// 2. Create permission via `CreatePermissionCpiBuilder` with empty members list
///    (no external RPC readers — maximum privacy).
/// 3. Delegate `MarketState` to TEE validator via `delegate_account()`.
/// 4. Set `is_delegated = true`, `delegated_at = clock.unix_timestamp`.
///
/// # Notes
/// - Permissionless — anyone can call (incentivized by keeper reward on settle).
/// - Uses `#[delegate]` macro from `ephemeral-rollups-sdk`.
pub fn handler(_ctx: Context<DelegateMarket>) -> Result<()> {
    // TODO (Chunk C): Implement
    // 1. require!(!market.is_delegated)
    // 2. require!(market.bid_count >= 1 && market.ask_count >= 1)
    // 3. CreatePermissionCpiBuilder with empty members
    // 4. delegate_account() CPI with DelegateConfig { validator: TEE_PUBKEY, commit_frequency_ms }
    // 5. market.is_delegated = 1
    // 6. market.delegated_at = clock.unix_timestamp
    Ok(())
}

#[derive(Accounts)]
pub struct DelegateMarket<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,

    // TODO (Chunk C): Add delegation accounts
    // - #[account(del)] market field for auto-generated delegation accounts
    // - permission_program
    // - permission_account (PDA under permission program)
    // - delegation_program
    // - delegation_buffer, delegation_record, delegation_metadata
    // - owner_program (this program)
    // - system_program

    pub system_program: Program<'info, System>,
}
