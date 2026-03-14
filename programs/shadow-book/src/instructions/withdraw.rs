use anchor_lang::prelude::*;

/// Withdraws SPL tokens from the market vault back to the trader's wallet.
///
/// **Chunk B** — mainnet instruction.
///
/// # Validation
/// - Trader must have sufficient EATA balance.
/// - Funds locked by open orders cannot be withdrawn.
///   (available = eata_balance - sum of trader's open order sizes)
pub fn handler(_ctx: Context<Withdraw>, _amount: u64) -> Result<()> {
    // TODO (Chunk B): Implement
    // - Validate available balance (not locked by open orders)
    // - withdrawSplIx from vault back to trader ATA
    Ok(())
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub trader: Signer<'info>,

    // TODO (Chunk B): Add accounts
    // - market: AccountLoader<MarketState>
    // - vault / vault_ata
    // - ephemeral_ata
    // - trader_token_account
    // - token_program
    // - system_program
    pub system_program: Program<'info, System>,
}
