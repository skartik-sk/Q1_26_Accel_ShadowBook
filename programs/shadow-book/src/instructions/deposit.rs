use anchor_lang::prelude::*;

/// Deposits SPL tokens into the market vault using the EATA pattern.
///
/// **Chunk B** — mainnet instruction.
///
/// # Flow
/// 1. Init vault + vault ATA if first deposit for this market.
/// 2. Init EATA for trader if first deposit for this trader.
/// 3. SPL `transfer` from trader's ATA to vault ATA.
/// 4. Update EATA balance.
///
/// # Validation
/// - Mint must match market's `mint_a` or `mint_b`.
pub fn handler(_ctx: Context<Deposit>, _amount: u64) -> Result<()> {
    // TODO (Chunk B): Implement using EATA pattern from MagicBlock SDK
    // - initVaultIx / initVaultAtaIx
    // - initEphemeralAtaIx
    // - transferToVaultIx
    Ok(())
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub trader: Signer<'info>,

    // TODO (Chunk B): Add accounts
    // - market: AccountLoader<MarketState>
    // - trader_token_account: Account<TokenAccount>
    // - vault / vault_ata (EATA pattern)
    // - ephemeral_ata (EATA pattern)
    // - token_program
    // - system_program

    pub system_program: Program<'info, System>,
}
