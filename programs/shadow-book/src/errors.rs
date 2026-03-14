use anchor_lang::prelude::*;

#[error_code]
pub enum ShadowBookError {
    // -- Market errors -------------------------------------------------------
    #[msg("Market is currently delegated to PER; this operation is not allowed")]
    MarketDelegated,

    #[msg("Market is not delegated; this operation requires PER delegation")]
    MarketNotDelegated,

    #[msg("Market must have at least one bid and one ask to delegate")]
    InsufficientOrdersToDelegate,

    // -- Order errors --------------------------------------------------------
    #[msg("Order book is full on this side")]
    OrderBookFull,

    #[msg("Order not found")]
    OrderNotFound,

    #[msg("Order has already been matched or cancelled")]
    OrderNotOpen,

    #[msg("Order has expired")]
    OrderExpired,

    #[msg("Invalid order side")]
    InvalidSide,

    #[msg("Order size must be greater than zero")]
    ZeroSize,

    #[msg("Signer is not the order owner")]
    UnauthorizedOrderAccess,

    // -- Balance errors ------------------------------------------------------
    #[msg("Insufficient balance for this operation")]
    InsufficientBalance,

    #[msg("Cannot withdraw while orders are open; cancel orders first")]
    FundsLocked,

    // -- Matching errors -----------------------------------------------------
    #[msg("No crossing orders found")]
    NoCrossingOrders,

    #[msg("Match results buffer is full; settle existing matches first")]
    MatchResultsFull,

    #[msg("Fill price deviates more than allowed from oracle price")]
    OracleSanityCheckFailed,

    // -- Settlement errors ---------------------------------------------------
    #[msg("Match has already been settled")]
    AlreadySettled,

    #[msg("No unsettled matches to process")]
    NothingToSettle,

    // -- Fee errors ----------------------------------------------------------
    #[msg("Only the market authority can perform this action")]
    UnauthorizedAuthority,

    #[msg("Fee rate exceeds maximum allowed (10%)")]
    FeeRateTooHigh,

    // -- Oracle errors -------------------------------------------------------
    #[msg("Oracle price feed data is invalid or stale")]
    InvalidOracleData,

    // -- Numeric errors ------------------------------------------------------
    #[msg("Arithmetic overflow")]
    MathOverflow,
}
