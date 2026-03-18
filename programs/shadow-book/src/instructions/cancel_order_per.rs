use anchor_lang::prelude::*;
use ephemeral_rollups_sdk::anchor::commit;
use ephemeral_rollups_sdk::ephem::{commit_accounts, commit_and_undelegate_accounts};

use crate::errors::ShadowBookError;
use crate::helpers::{find_order, remove_order};
use crate::state::{MarketState, OrderStatus, Side};

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
pub fn handler(ctx: Context<CancelOrderPer>, order_id: u64, side: u8) -> Result<()> {
    let mut market = ctx.accounts.market.load_mut()?;

    require!(market.is_delegated(), ShadowBookError::MarketNotDelegated);

    let parsed_side = match side {
        0 => Side::Buy,
        1 => Side::Sell,
        _ => return err!(ShadowBookError::InvalidSide),
    };

    let order_index =
        find_order(&market, order_id, parsed_side).ok_or(ShadowBookError::OrderNotFound)?;

    let order = match parsed_side {
        Side::Buy => &market.bids[order_index],
        Side::Sell => &market.asks[order_index],
    };

    require!(
        order.trader_pubkey() == ctx.accounts.trader.key(),
        ShadowBookError::UnauthorizedOrderAccess
    );

    require!(
        order.status == OrderStatus::Open as u8,
        ShadowBookError::OrderNotOpen
    );

    remove_order(&mut market, order_id, parsed_side)?;

    let is_empty = market.bid_count == 0 && market.ask_count == 0;

    if is_empty {
        market.is_delegated = 0;
        market.delegated_at = 0;
    }

    // Drop the mutable borrow before calling CPIs
    drop(market);

    let market_info = ctx.accounts.market.to_account_info();
    let payer_info = ctx.accounts.trader.to_account_info();
    let magic_context = ctx.accounts.magic_context.to_account_info();
    let magic_program = ctx.accounts.magic_program.to_account_info();

    if is_empty {
        commit_and_undelegate_accounts(
            &payer_info,
            vec![&market_info],
            &magic_context,
            &magic_program,
        )?;
        msg!(
            "Order {} cancelled. Book empty, undelegating market.",
            order_id
        );
    } else {
        commit_accounts(
            &payer_info,
            vec![&market_info],
            &magic_context,
            &magic_program,
        )?;
        msg!("Order {} cancelled privately.", order_id);
    }

    Ok(())
}

#[commit]
#[derive(Accounts)]
pub struct CancelOrderPer<'info> {
    #[account(mut)]
    pub trader: Signer<'info>,

    #[account(mut)]
    pub market: AccountLoader<'info, MarketState>,

    pub system_program: Program<'info, System>,
}
