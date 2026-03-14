# Shadow Book — Implementation Spec

> **Status**: Approved. Ready for team pickup.
> **Date**: 2026-03-14

---

## Architecture Overview

Privacy-preserving dark pool DEX on Solana. Order sizes hidden inside Intel TDX TEE via MagicBlock Private Ephemeral Rollups (PER). Epoch-based batch auction model.

### Epoch Lifecycle

```
┌─────────────────────────────────────────────────────────────┐
│  Phase A: COLLECTION (mainnet)                              │
│  MarketState.is_delegated == false                          │
│                                                             │
│  • Traders call create_order(side, price) — size = 0        │
│  • Orders visible on-chain but sizes are unknown            │
│  • Anyone can cancel_order, claim_expired                   │
│  • Deposits and withdrawals open                            │
│                                                             │
│  Trigger: crank calls delegate_market when ≥1 bid + ≥1 ask │
├─────────────────────────────────────────────────────────────┤
│  Phase B: EXECUTION (PER / TEE)                             │
│  MarketState.is_delegated == true                           │
│                                                             │
│  • MarketState delegated to TEE validator                   │
│  • Traders call submit_order_size(order_id, size) via PER   │
│  • Sizes written inside TEE — invisible on mainnet          │
│  • Crank calls match_orders via PER                         │
│  • Matching at midpoint with oracle sanity check            │
│  • commit_and_undelegate pushes results to mainnet          │
│                                                             │
│  No mainnet writes to MarketState during this phase         │
├─────────────────────────────────────────────────────────────┤
│  Phase C: SETTLEMENT (mainnet)                              │
│  MarketState.is_delegated == false (after commit)           │
│                                                             │
│  • Crank calls settle — reads match_results, transfers SPL  │
│  • Keeper reward paid to settle caller                      │
│  • Cycle restarts → Phase A                                 │
└─────────────────────────────────────────────────────────────┘
```

---

## Account Structure

### MarketState PDA — `#[account(zero_copy)]`

**Seeds**: `[b"market", mint_a.key(), mint_b.key()]`
**Size**: ~112KB (pre-allocated at creation)

| Field | Type | Bytes | Description |
|-------|------|-------|-------------|
| mint_a | Pubkey | 32 | Base token mint |
| mint_b | Pubkey | 32 | Quote token mint |
| authority | Pubkey | 32 | Market authority (fee collection, parameter updates) |
| total_volume | u64 | 8 | Cumulative matched volume |
| fee_rate_bps | u16 | 2 | Trading fee in basis points |
| keeper_reward_bps | u16 | 2 | % of fee paid to settle caller |
| oracle_feed_id | [u8; 32] | 32 | Pyth Lazer feed ID for this pair |
| next_order_id | u64 | 8 | Monotonic order ID counter |
| bids | [Order; 256] | 26,624 | Sorted: price DESC, time ASC |
| asks | [Order; 256] | 26,624 | Sorted: price ASC, time ASC |
| bid_count | u16 | 2 | Active bid count |
| ask_count | u16 | 2 | Active ask count |
| match_results | [MatchResult; 128] | 11,264 | Pending settlements |
| match_count | u16 | 2 | Active match result count |
| is_delegated | u8 | 1 | Epoch phase flag (bool as u8 for Pod) |
| delegated_at | i64 | 8 | Delegation timestamp |
| bump | u8 | 1 | PDA bump seed |
| _padding | [u8; N] | N | 8-byte alignment padding |

### Order — 104 bytes, 8-byte aligned

| Field | Type | Bytes | Description |
|-------|------|-------|-------------|
| trader | Pubkey | 32 | Order owner |
| order_id | u64 | 8 | Unique ID (from next_order_id) |
| side | u8 | 1 | 0=Buy, 1=Sell |
| status | u8 | 1 | 0=Empty, 1=Open, 2=Matched, 3=Cancelled |
| _pad1 | [u8; 6] | 6 | Alignment |
| price | u64 | 8 | Limit price (token_b per token_a, smallest units) |
| size | u64 | 8 | Order size (token_a, smallest units) — **written only inside TEE** |
| timestamp | i64 | 8 | Creation time |
| expires_at | i64 | 8 | Expiration time (creation + ORDER_TTL_SECONDS) |
| matched_price | u64 | 8 | Fill price (set by match_orders) |
| _reserved | [u8; 16] | 16 | Future fields without realloc |

### MatchResult — 88 bytes, 8-byte aligned

| Field | Type | Bytes | Description |
|-------|------|-------|-------------|
| buyer | Pubkey | 32 | Buy-side trader |
| seller | Pubkey | 32 | Sell-side trader |
| price | u64 | 8 | Midpoint fill price |
| size | u64 | 8 | Fill size (token_a units) |
| settled | u8 | 1 | 0=pending, 1=settled |
| _pad | [u8; 7] | 7 | Alignment |

### Token Vaults — EATA Pattern (MagicBlock SDK)

| Account | Derivation | Purpose |
|---------|-----------|---------|
| Vault | `deriveVault(mint)` | Program-owned SPL token vault per mint |
| EATA | `deriveEphemeralAta(trader, mint)` | Per-trader balance tracking (delegatable) |
| Fee Vault | `deriveEphemeralAta(fee_authority, mint)` | Accumulated trading fees |

---

## Instruction Set

### Mainnet Instructions

| Instruction | Signer | Phase | Description |
|-------------|--------|-------|-------------|
| `initialize_market` | authority | — | Create MarketState PDA, set mints/fees/oracle |
| `deposit` | trader | A | SPL transfer to vault, update EATA balance |
| `withdraw` | trader | A | SPL transfer from vault, validate no locked funds |
| `create_order` | trader | A | Add order to book (side+price, size=0) |
| `delegate_market` | anyone | A→B | Delegate MarketState to TEE, create permission |
| `cancel_order` | trader | A | Remove order from book (market must not be delegated) |
| `settle` | anyone | C | Read match_results, execute SPL transfers, pay keeper |
| `claim_expired` | anyone | A | Remove expired orders (max 10 per call) |
| `collect_fees` | authority | any | Withdraw accumulated fees |

### PER Instructions (inside TEE)

| Instruction | Signer | Phase | Description |
|-------------|--------|-------|-------------|
| `submit_order_size` | trader | B | Write size into order slot (TEE-only, private) |
| `match_orders` | anyone (crank) | B | Match crossing orders, write MatchResults, commit+undelegate |
| `cancel_order_per` | trader | B | Cancel order inside TEE, commit |

---

## Constants

```rust
// Program IDs
pub const DELEGATION_PROGRAM_ID: Pubkey = pubkey!("DELeGGvXpWV2fqJUhqcF5ZSYMS4JTLjteaAMARRSaeSh");
pub const PERMISSION_PROGRAM_ID: Pubkey = pubkey!("ACLseoPoyC3cBqoUtkbjZ4aDrkurZW86v19pXz2XQnp1");
pub const MAGIC_PROGRAM_ID: Pubkey = pubkey!("Magic11111111111111111111111111111111111111");
pub const MAGIC_CONTEXT_ID: Pubkey = pubkey!("MagicContext1111111111111111111111111111111");
pub const ORACLE_PROGRAM_ID: Pubkey = pubkey!("PriCems5tHihc6UDXDjzjeawomAwBduWMGAi8ZUjppd");

// TEE Validators
pub const DEVNET_TEE_VALIDATOR: Pubkey = pubkey!("FnE6VJT5QNZdedZPnCoLsARgBwoE6DeJNjBs2H1gySXA");
pub const MAINNET_TEE_VALIDATOR: Pubkey = pubkey!("MTEWGuqxUpYZGFJQcp8tLN7x5v9BSeoFHYWQQ3n3xzo");

// Limits
pub const MAX_ORDERS: usize = 256;
pub const MAX_MATCHES: usize = 128;
pub const MAX_MATCHES_PER_CALL: usize = 10;
pub const MAX_SETTLEMENTS_PER_CALL: usize = 10;
pub const MAX_EXPIRED_CLEANUP_PER_CALL: usize = 10;

// Timing
pub const ORDER_TTL_SECONDS: i64 = 3600;           // 1 hour
pub const DELEGATION_TIMEOUT_SECONDS: i64 = 300;   // 5 minutes (Phase 2: force_undelegate)
pub const COMMIT_FREQUENCY_MS: u32 = 30_000;       // 30 seconds

// Fees
pub const DEFAULT_FEE_RATE_BPS: u16 = 30;          // 0.30%
pub const DEFAULT_KEEPER_REWARD_BPS: u16 = 50;     // 5bps of fee goes to keeper

// Oracle
pub const ORACLE_SANITY_BAND_BPS: u16 = 500;       // 5% max deviation from oracle
pub const ORACLE_PRICE_OFFSET: usize = 73;         // byte offset in Pyth Lazer account
```

---

## Matching Algorithm

```
1. Read oracle price from Pyth Lazer PDA (offset 73, i64, scale by exponent)
2. For i in 0..ask_count:
   if asks[i].status != Open || asks[i].size == 0: continue
   For j in 0..bid_count:
     if bids[j].status != Open || bids[j].size == 0: continue
     if bids[j].price < asks[i].price: break  // no more crosses (sorted)

     fill_price = (bids[j].price + asks[i].price) / 2

     // Oracle sanity check
     if |fill_price - oracle_price| > oracle_price * ORACLE_SANITY_BAND_BPS / 10000:
       continue  // skip this match

     fill_size = min(bids[j].size, asks[i].size)  // v1: skip if not equal

     match_results[match_count] = MatchResult {
       buyer: bids[j].trader,
       seller: asks[i].trader,
       price: fill_price,
       size: fill_size,
       settled: false,
     }
     match_count += 1
     total_volume += fill_size
     bids[j].status = Matched
     bids[j].matched_price = fill_price
     asks[i].status = Matched
     asks[i].matched_price = fill_price
     break  // move to next ask (full fills only in v1)

   if match_count >= MAX_MATCHES_PER_CALL: break

3. commit_and_undelegate_accounts(MarketState)
4. market.is_delegated = false
```

---

## Toolchain

| Tool | Version |
|------|---------|
| Anchor | 0.32.1 |
| ephemeral-rollups-sdk (Rust) | 0.8.8 |
| @magicblock-labs/ephemeral-rollups-sdk (TS) | ^0.8.5 |
| @solana/web3.js | ^1.98.0 |
| Solana CLI (Agave) | 3.1.11+ |
| Rust | 1.86.0+ |
| Node | 20+ |

---

## Work Chunks

To find all work items for a chunk: `grep -rn "TODO (Chunk X)" programs/ tests/ sdk/`

### Dependency Graph

```
A (account structs)  -- foundation, start first
  |
  |---> B (token flow)   -- needs Order/MatchResult types from A
  |---> C (PER/TEE)      -- needs helpers + types from A
  |---> D (SDK scaffold starts day 1, wires up as B+C deliver)
              |
         B + C land
              |
         D runs full E2E
              |
     E    F    G    H    (Phase 2, all independent)
```

**Critical path**: A then C then D (E2E test). B is parallel with C but must land before D's E2E.

---

### Chunk A: Account Structures + Market Init

**Dependencies**: none

Core data types everything else depends on. Every other chunk imports from here.

**Deliverables**:

- `programs/shadow-book/src/state/`:
  - `MarketState` with `#[account(zero_copy)]` (already scaffolded)
  - `Order`, `MatchResult` structs (already scaffolded)
  - `OrderStatus` and `Side` enums (already scaffolded)
- `programs/shadow-book/src/helpers.rs`:
  - `insert_bid(market, order)` -- binary search insert, maintains price DESC + time ASC
  - `insert_ask(market, order)` -- binary search insert, maintains price ASC + time ASC
  - `remove_order(market, order_id, side)` -- find + shift array
  - `find_order(market, order_id, side)` -- linear scan by order_id
  - `cleanup_expired(market, now, max_removals)` -- removes expired orders, bounded
  - All helpers are scaffolded. Implement the TODO bodies and add comprehensive unit tests.
- `programs/shadow-book/src/instructions/initialize_market.rs`:
  - Creates `MarketState` PDA with `seeds: [b"market", mint_a, mint_b]`
  - Allocates full account size upfront
  - Sets `authority`, `mint_a`, `mint_b`, `fee_rate_bps`, `keeper_reward_bps`, `oracle_feed_id`
  - Validate `fee_rate_bps <= 1000` (max 10%)
- `programs/shadow-book/src/instructions/cancel_order.rs`:
  - Mainnet-only cancel: `require!(!market.is_delegated)`
  - Validates signer is the order's trader
  - Calls `remove_order()`, sets status=Cancelled
- `programs/shadow-book/src/instructions/claim_expired.rs`:
  - Permissionless (no signer check on caller)
  - Calls `cleanup_expired(market, clock.unix_timestamp, MAX_EXPIRED_CLEANUP_PER_CALL)`
  - Bounded: max 10 removals per call to cap compute

**Tests**: `tests/market/initialize-market.test.ts`, `tests/orders/cancel-order.test.ts`, `tests/orders/claim-expired.test.ts`

**Produces**: types + helpers consumed by B, C, D

---

### Chunk B: EATA Token Flow + Settlement

**Dependencies**: needs `MarketState` type from A (can stub initially)

All SPL token deposit/withdraw/settlement logic.

**Deliverables**:

- `programs/shadow-book/src/instructions/deposit.rs`:
  - Init vault PDA `[b"vault", market]` + vault ATA if first deposit for this market
  - Init EATA for trader via `initEphemeralAtaIx` if first deposit for this trader
  - SPL `transfer` from trader's ATA to vault ATA
  - Update EATA balance to track trader's deposited amount
  - Validate: mint matches market's `mint_a` or `mint_b`
- `programs/shadow-book/src/instructions/withdraw.rs`:
  - SPL `transfer` from vault ATA back to trader's ATA
  - Validate: trader has sufficient EATA balance, no open orders locking funds
  - Fund locking: trader's balance minus sum of their open order sizes = available to withdraw
- `programs/shadow-book/src/instructions/settle.rs`:
  - Reads committed `match_results[0..match_count]` from `MarketState`
  - For each `MatchResult` where `settled == false`:
    - Transfer `size` of token_a: buyer's EATA to seller's EATA
    - Transfer `size * price` of token_b: seller's EATA to buyer's EATA
    - Deduct `fee_rate_bps` from both sides, credit to fee vault EATA
    - If `keeper_reward_bps > 0`, credit keeper (tx signer) from fee vault
    - Set `settled = true`
  - Permissionless (incentivized by keeper reward)
  - Bounded: max `MAX_SETTLEMENTS_PER_CALL` per invocation
- `programs/shadow-book/src/instructions/collect_fees.rs`:
  - Authority-only: withdraw accumulated fees from fee vault EATA to authority's ATA

**Tests**: `tests/market/deposit.test.ts`, `tests/market/withdraw.test.ts`, `tests/settlement/settle.test.ts`, `tests/settlement/collect-fees.test.ts`

**Produces**: settlement logic consumed by E2E tests in D

---

### Chunk C: PER Delegation + TEE Instructions

**Dependencies**: needs types + helpers from A, EATA patterns from B

The core privacy primitive. Study `private-payments-demo` and `magicblock-engine-examples/anchor-counter` thoroughly before starting.

**Deliverables**:

- `programs/shadow-book/src/instructions/create_order.rs` (mainnet, Phase A):
  - Adds order to `MarketState` bids or asks via `insert_bid()`/`insert_ask()`
  - Sets `side`, `price`, `timestamp`, `expires_at`, `status=Open`, `size=0` (placeholder)
  - Validates: market not delegated, trader has sufficient EATA balance
  - Generates unique `order_id` from `market.next_order_id` counter (monotonic)
- `programs/shadow-book/src/instructions/delegate_market.rs` (mainnet, Phase A to B):
  - Permissionless (incentivized by keeper reward)
  - Validates: market has at least 1 bid AND 1 ask
  - Creates permission via `CreatePermissionCpiBuilder` with empty members list (zero external readers)
  - Delegates `MarketState` to TEE validator via `delegate_account()` with `DelegateConfig { validator: Some(TEE_PUBKEY), commit_frequency_ms: 30_000 }`
  - Sets `market.is_delegated = true`, `market.delegated_at = clock.unix_timestamp`
  - Uses `#[delegate]` macro with `#[account(del)]` on market field
- `programs/shadow-book/src/instructions/submit_order_size.rs` (PER-only, Phase B):
  - `#[ephemeral]` module attribute
  - Validates: signer == order.trader, order status is Open, size > 0
  - Writes actual `size` into the trader's order slot in `MarketState`
  - This is the critical privacy moment -- size only exists inside TEE memory
- `programs/shadow-book/src/instructions/match_orders.rs` (PER-only crank, Phase B):
  - `#[ephemeral]` + `#[commit]` macros
  - Matching algorithm (see Matching Algorithm section above for pseudocode):
    1. Read Pyth Lazer oracle price from oracle account PDA
    2. Walk asks from lowest price, find crossing bids
    3. Fill at midpoint `(bid.price + ask.price) / 2`
    4. Oracle sanity check: reject if fill deviates >5% from oracle
    5. Write `MatchResult`, set both orders to Matched
    6. Call `commit_and_undelegate_accounts` for `MarketState`
  - Bounded: max `MAX_MATCHES_PER_CALL` matches per invocation
  - Oracle account passed as remaining account (read-only, not delegated)
- `programs/shadow-book/src/instructions/cancel_order_per.rs` (PER-only, Phase B):
  - `#[ephemeral]` + `#[commit]`
  - Validates: signer == order.trader
  - Removes order, commits + undelegates if book is now empty
  - If book still has orders, commits only (market stays in PER)

**Tests**: `tests/orders/create-order.test.ts`, `tests/per/delegate-market.test.ts`, `tests/per/submit-order-size.test.ts`, `tests/per/match-orders.test.ts`, `tests/per/cancel-order-per.test.ts`

**Produces**: committed `match_results` consumed by B's settle

---

### Chunk D: TypeScript Client SDK + E2E Tests

**Dependencies**: needs deployed program from A+B+C (start scaffolding immediately)

Client library, automated crank, and the E2E test suite that proves the entire system works.

**Deliverables**:

- `sdk/src/client.ts`:
  - `ShadowBookClient` class wrapping all mainnet instructions
  - Constructor takes `Connection` (mainnet) + `Wallet` + `programId`
  - Methods: `initializeMarket()`, `deposit()`, `withdraw()`, `createOrder()`, `delegateMarket()`, `settle()`, `cancelOrder()`, `collectFees()`
  - Each method builds, signs, sends tx, confirms, returns tx signature
  - Typed return values (parse accounts after tx, not just signatures)
- `sdk/src/per-client.ts`:
  - `ShadowBookPERClient` extends `ShadowBookClient` with PER connection
  - TEE auth: `verifyTeeRpcIntegrity(EPHEMERAL_RPC_URL)` + `getAuthToken()` on construction
  - PER `Connection` at `${EPHEMERAL_RPC_URL}?token=${token}`
  - Methods: `submitOrderSize()`, `matchOrders()`, `cancelOrderPER()`
  - Polls mainnet after commit+undelegate (retry with backoff, max 10 attempts, 500ms intervals)
- `sdk/src/crank.ts`:
  - `ShadowBookCrank` class -- automated epoch lifecycle operator
  - Polls market state: if not delegated and has bids + asks, calls `delegateMarket()`
  - If delegated and size submission window passed, calls `matchOrders()`
  - After match + commit, calls `settle()` to collect keeper reward
  - Graceful shutdown, error logging, retry logic
- `sdk/src/types.ts`:
  - TypeScript types matching on-chain structs
  - Zero-copy deserialization from raw account data (DataView reads at known offsets)
- `tests/e2e/epoch-lifecycle.test.ts` -- full epoch lifecycle:
  1. `initializeMarket(mint_a, mint_b, fee_rate=30bps, keeper_reward=5bps)`
  2. Mint test tokens to Trader A and Trader B
  3. Both traders `deposit`
  4. Trader A `createOrder(BUY, price=100)`, Trader B `createOrder(SELL, price=95)`
  5. Crank `delegateMarket()`, verify `is_delegated == true`
  6. Both traders `submitOrderSize(50)` via PER
  7. Privacy assertion: query mainnet, sizes must still be 0
  8. Crank `matchOrders()` via PER, commits + undelegates
  9. Verify `match_results[0]`: price=97 (midpoint), size=50
  10. Crank `settle()`, verify EATA balances, fees deducted
  11. Both traders `withdraw()`, verify ATA balances
- Edge case tests: cancel, expiry, no crossing, multiple orders, insufficient funds, double settle

**Produces**: the definitive proof the system works end-to-end on devnet

---

### Phase 2 Chunks (after Phase 1 E2E passes)

#### Chunk E: Partial Fills + Market Orders

- Modify `match_orders`: if `bid.size != ask.size`, fill `min(bid.size, ask.size)`, leave remainder as open order with `size -= fill_size`
- Remaining order stays `status = Open` with reduced size, gets matched in next epoch
- Market orders: `side=Buy, price=u64::MAX` or `side=Sell, price=0`, fills at whatever resting price exists
- Market orders require Pyth oracle price as fill price (not midpoint, since no counterparty limit to reference)
- Update `MatchResult` to track `fill_size` separately from order `size`
- Tests: partial fill lifecycle, market order fills, oracle bounds check

#### Chunk F: Advanced Fee Mechanism

- Maker/taker fee split: `maker_fee_bps` and `taker_fee_bps` on `MarketState`
- Fee vault EATA per market: `[b"fee_vault", market]`
- Volume-based fee tiers: `if trader_volume > threshold then reduced_fee_bps` (stored in a `TraderStats` PDA)
- `update_fees` authority-only instruction to adjust fee rates
- Tests: fee tier transitions, maker vs taker fee correctness

#### Chunk G: Frontend

- Next.js 14+ app with app router
- Wallet adapter (Phantom, Solflare, Backpack)
- Pages:
  - Dashboard: connected wallet balances, deposited amounts, open orders, trade history
  - Trade: deposit/withdraw forms, order placement (side + price, size hidden until TEE), epoch status indicator
  - Market: select market pair, post-settlement trade tape (price + size of completed trades, public data only)
- No order book display -- that is the entire point. Show "Market Status: Collecting Orders / Matching in Progress / Settling"
- Real-time updates via websocket subscription to market account changes
- Uses `ShadowBookClient` and `ShadowBookPERClient` from Chunk D's SDK

#### Chunk H: Crash Recovery + Robustness

- Order timeout: `expires_at = clock.unix_timestamp + ORDER_TTL_SECONDS` set at order creation
- Delegation timeout: if market has been delegated for >5 minutes without a commit, anyone can call `force_undelegate` (emergency instruction)
- `force_undelegate`: checks delegation timestamp, calls `undelegate_account` CPI, resets `is_delegated = false`, all orders revert to `size=0` (safe since no sizes were committed)
- SDK retry logic: `ShadowBookPERClient` wraps PER calls with exponential backoff (500ms, 1s, 2s, 4s, 8s), max 5 retries
- Crank health monitoring: `ShadowBookCrank` logs metrics (matches/epoch, settle latency, PER connection failures), exposes a health endpoint
- Stale match cleanup: if `match_results` have `settled == false` for >10 minutes after undelegate, crank auto-calls `settle`
- Tests: simulate PER timeout, verify force_undelegate, verify retry logic

---

## Reference Repos

- **private-payments-demo**: `github.com/magicblock-labs/private-payments-demo` — EATA pattern, permission setup, delegation flow
- **anchor-counter**: `github.com/magicblock-labs/magicblock-engine-examples` — basic delegation + commit pattern
- **ephemeral-rollups-spl**: `github.com/magicblock-labs/ephemeral-rollups-spl` — older TokenEscrow pattern (reference only)
- **real-time-pricing-oracle**: `github.com/magicblock-labs/real-time-pricing-oracle` — Pyth Lazer integration
