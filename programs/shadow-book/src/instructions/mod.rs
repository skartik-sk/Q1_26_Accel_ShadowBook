// ---------------------------------------------------------------------------
// Mainnet instructions
// ---------------------------------------------------------------------------
pub mod cancel_order;
pub mod claim_expired;
pub mod collect_fees;
pub mod create_order;
pub mod delegate_market;
pub mod deposit;
pub mod initialize_market;
pub mod settle;
pub mod withdraw;

// ---------------------------------------------------------------------------
// PER instructions (run inside TEE)
// ---------------------------------------------------------------------------
pub mod cancel_order_per;
pub mod match_orders;
pub mod submit_order_size;

// ---------------------------------------------------------------------------
// Re-exports
// ---------------------------------------------------------------------------
pub use cancel_order::CancelOrder;
pub use cancel_order_per::CancelOrderPer;
pub use claim_expired::ClaimExpired;
pub use collect_fees::CollectFees;
pub use create_order::CreateOrder;
pub use delegate_market::DelegateMarket;
pub use deposit::Deposit;
pub use initialize_market::InitializeMarket;
pub use match_orders::MatchOrders;
pub use settle::Settle;
pub use submit_order_size::SubmitOrderSize;
pub use withdraw::Withdraw;

// Anchor's #[derive(Accounts)] generates hidden __client_accounts modules
// that the #[program] macro expects at the crate root.
pub(crate) use cancel_order::__client_accounts_cancel_order;
pub(crate) use cancel_order_per::__client_accounts_cancel_order_per;
pub(crate) use claim_expired::__client_accounts_claim_expired;
pub(crate) use collect_fees::__client_accounts_collect_fees;
pub(crate) use create_order::__client_accounts_create_order;
pub(crate) use delegate_market::__client_accounts_delegate_market;
pub(crate) use deposit::__client_accounts_deposit;
pub(crate) use initialize_market::__client_accounts_initialize_market;
pub(crate) use match_orders::__client_accounts_match_orders;
pub(crate) use settle::__client_accounts_settle;
pub(crate) use submit_order_size::__client_accounts_submit_order_size;
pub(crate) use withdraw::__client_accounts_withdraw;
