# Shadow Book

A privacy-preserving dark pool DEX on Solana. Order sizes are hidden from submission through matching using MagicBlock Private Ephemeral Rollups (PER) backed by Intel TDX hardware enclaves. Only settlement (token transfer amounts and prices) is public.

## Problem

Every existing Solana DEX exposes all order data before execution. A trader submitting a large buy order reveals their wallet, trade size, and direction to the entire network. This enables front-running and sandwich attacks by MEV bots that move the price before the order fills.

Traditional dark pools solve this through a trusted operator. Shadow Book replaces the trusted operator with a TEE-secured Ephemeral Rollup where the matching engine runs inside Intel TDX hardware. Not even the node operator can read the order book state.

## How It Works

Shadow Book uses an epoch-based batch auction model with three phases:

```
Phase A: COLLECTION (Solana mainnet)
  Traders place orders with side and price only. Size is not set.
  Deposits and withdrawals are open.

Phase B: EXECUTION (MagicBlock PER / TEE)
  The market account is delegated to a TEE validator.
  Traders submit their hidden order sizes inside the enclave.
  A crank triggers matching at the midpoint of crossing prices.
  Results are committed back to mainnet.

Phase C: SETTLEMENT (Solana mainnet)
  A crank reads committed match results and executes SPL token
  transfers between trader vaults. The cycle restarts.
```

### Privacy Model

| Data | Visibility |
| --- | --- |
| That a trader deposited collateral | Public (mainnet tx) |
| That an order was submitted (side + price) | Public (mainnet tx) |
| Order size | Private (TEE only, never on mainnet until settled) |
| Counterparty identity before match | Private |
| That a match occurred, final price and size | Public (settlement tx) |
| Token balance changes | Public (SPL transfer) |

## Architecture

```
programs/shadow-book/src/
  lib.rs                     Program entrypoint (12 instructions)
  constants.rs               Program IDs, limits, fees, oracle config
  errors.rs                  Typed error codes
  helpers.rs                 Order book operations, oracle parsing
  state/
    market.rs                MarketState (zero-copy, ~64KB inline order book)
    order.rs                 Order struct (104 bytes) + enums
    match_result.rs          MatchResult struct (88 bytes)
  instructions/
    initialize_market.rs     Create a market for a token pair
    deposit.rs               SPL token deposit via EATA pattern
    withdraw.rs              SPL token withdrawal
    create_order.rs          Place order (side + price, size = 0)
    cancel_order.rs          Cancel on mainnet (Phase A only)
    delegate_market.rs       Delegate to TEE validator
    submit_order_size.rs     Write hidden size inside TEE (PER-only)
    match_orders.rs          Matching engine crank (PER-only)
    cancel_order_per.rs      Cancel inside TEE (PER-only)
    settle.rs                Execute SPL transfers from match results
    claim_expired.rs         Permissionless expired order cleanup
    collect_fees.rs          Authority fee withdrawal

sdk/src/
  client.ts                  Mainnet instruction wrappers
  per-client.ts              TEE auth + PER instruction wrappers
  crank.ts                   Automated epoch lifecycle operator
  types.ts                   TypeScript types matching on-chain structs

tests/
  setup/                     Shared test context and helpers
  market/                    Market init, deposit, withdraw tests
  orders/                    Order creation, cancellation, expiry tests
  per/                       Delegation, size submission, matching tests
  settlement/                Settle, fee collection tests
  e2e/                       Full epoch lifecycle integration tests
```

### Key Design Decisions

**Single MarketState account.** The entire order book (256 bids, 256 asks, 128 pending match results) lives in one zero-copy account. This follows the Phoenix/OpenBook pattern, eliminates the need to enumerate accounts inside a program, and lets Solana's write locks serialize all matching operations automatically.

**Epoch-based batching.** Orders are collected on mainnet, then the market is delegated to PER for private size submission and matching. This mirrors how institutional dark pools operate (periodic batch auctions) and avoids the UX problem of traders being unable to place orders while the market is delegated.

**Midpoint fill pricing with oracle guardrail.** Fill price is the midpoint of the two crossing limit prices. A Pyth Lazer oracle feed (available natively inside the ER at 50-200ms update intervals) provides a 5% sanity band to reject stale or manipulated prices.

**Empty permission list for maximum privacy.** The Permission Program is configured with zero external readers. The program itself reads account data inside the TEE via instruction context. No wallet can query the order book state via RPC while it is delegated.

## Prerequisites

| Tool | Version |
| --- | --- |
| Solana CLI (Agave) | 3.1.11+ |
| Anchor | 0.32.1 |
| Rust | 1.86.0+ |
| Node.js | 20+ |

## Getting Started

```bash
# Clone the repository
git clone https://github.com/solana-turbin3/Q1_26_Accel_ShadowBook.git
cd Q1_26_Accel_ShadowBook

# Install dependencies
yarn install

# Build the program
anchor build

# Run tests (localnet)
anchor test

# Deploy to devnet (sets up wallet, airdrops SOL, builds, and deploys)
./scripts/setup-devnet.sh
```

## Work Chunks

The project is divided into independent work chunks that can be picked up in parallel. Each instruction stub contains `TODO (Chunk X)` comments with implementation steps.

| Chunk | Scope | Complexity |
| --- | --- | --- |
| A | Account structs, market init, cancel, claim_expired, helpers | Medium |
| B | EATA token flow, deposit, withdraw, settle, collect_fees | Medium |
| C | PER delegation, submit_order_size, match_orders, cancel_order_per | High |
| D | TypeScript SDK, crank, E2E integration tests | Medium |
| E | Partial fills, market orders (Phase 2) | Medium |
| F | Advanced fees: maker/taker split, volume tiers (Phase 2) | Small |
| G | Frontend: Next.js, wallet adapter (Phase 2) | High |
| H | Crash recovery, force_undelegate, retry logic (Phase 2) | Medium |

Chunks A, B, and C can start in parallel. D starts scaffolding immediately and integrates as others deliver. Phase 2 chunks (E-H) begin after the Phase 1 E2E test passes.

See [docs/implementation-spec.md](docs/implementation-spec.md) for the full specification including account layouts, matching algorithm pseudocode, and resolved design decisions.

## Testing

Tests are organized by domain. Each file is independently runnable and tagged with its owning chunk.

```bash
# Run all tests
anchor test

# Run a specific test file
yarn run ts-mocha -p ./tsconfig.json -t 1000000 "tests/market/deposit.test.ts"

# Run all PER tests (requires devnet TEE connection)
yarn run ts-mocha -p ./tsconfig.json -t 1000000 "tests/per/**/*.ts"
```

Tests under `tests/per/` and `tests/e2e/` require a connection to MagicBlock's TEE endpoint (`tee.magicblock.app`) and will not pass on localnet.

## Reference Repositories

These MagicBlock repositories contain patterns used in this project:

- [private-payments-demo](https://github.com/magicblock-labs/private-payments-demo) -- EATA pattern, permission setup, delegation flow
- [magicblock-engine-examples](https://github.com/magicblock-labs/magicblock-engine-examples) -- Basic delegation and commit patterns
- [ephemeral-rollups-spl](https://github.com/magicblock-labs/ephemeral-rollups-spl) -- Token escrow reference (older pattern)
- [real-time-pricing-oracle](https://github.com/magicblock-labs/real-time-pricing-oracle) -- Pyth Lazer oracle integration

## License

MIT
