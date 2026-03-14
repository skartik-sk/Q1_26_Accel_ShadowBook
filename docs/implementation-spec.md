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
| Solana CLI | 2.3.13 |
| Rust | 1.85.0 |
| Node | 24.x |

---

## Work Chunks

See [implementation plan](/home/allen/.claude/plans/modular-stirring-taco.md) for detailed chunk descriptions, deliverables, and dependencies.

| Chunk | What | Complexity | Est. Days |
|-------|------|-----------|-----------|
| **A** | Account structs + market init + helpers | M | 3 |
| **B** | EATA token flow + settlement | M | 3 |
| **C** | PER delegation + TEE instructions + matching | H | 5 |
| **D** | TS client SDK + E2E tests | M | 4 |
| **E** | Partial fills + market orders | M | 3 |
| **F** | Advanced fees (maker/taker, tiers) | S | 2 |
| **G** | Frontend (Next.js) | H | 5 |
| **H** | Crash recovery + robustness | M | 3 |

**Phase 1** (A-D): parallel, ~5 days to first E2E.
**Phase 2** (E-H): independent chunks, pick after Phase 1 E2E passes.

---

## Reference Repos

- **private-payments-demo**: `github.com/magicblock-labs/private-payments-demo` — EATA pattern, permission setup, delegation flow
- **anchor-counter**: `github.com/magicblock-labs/magicblock-engine-examples` — basic delegation + commit pattern
- **ephemeral-rollups-spl**: `github.com/magicblock-labs/ephemeral-rollups-spl` — older TokenEscrow pattern (reference only)
- **real-time-pricing-oracle**: `github.com/magicblock-labs/real-time-pricing-oracle` — Pyth Lazer integration
