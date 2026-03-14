use anchor_lang::prelude::*;

use crate::state::MarketState;

/// Withdraws accumulated trading fees from the fee vault.
///
/// **Chunk B** — mainnet instruction.
///
/// # Validation
/// - Only the market authority can call this.
pub fn handler(_ctx: Context<CollectFees>) -> Result<()> {
    // TODO (Chunk B): Implement
    // 1. Validate signer == market.authority
    // 2. Transfer fee vault EATA balance to authority's ATA
    Ok(())
}

#[derive(Accounts)]
pub struct CollectFees<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,

    // TODO (Chunk B): Add accounts
    // - fee_vault EATA
    // - authority_token_account
    // - vault / vault_ata
    // - token_program
    pub system_program: Program<'info, System>,
}
