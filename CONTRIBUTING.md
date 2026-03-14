# Contributing to Shadow Book

This document covers how to set up your environment, pick work, write code, and submit changes.

## Table of Contents

- [Getting Started](#getting-started)
- [Picking a Chunk](#picking-a-chunk)
- [Branch and Commit Conventions](#branch-and-commit-conventions)
- [Code Standards](#code-standards)
- [Testing Requirements](#testing-requirements)
- [Pull Request Process](#pull-request-process)
- [Architecture Notes](#architecture-notes)

---

## Getting Started

1. Clone the repo and install dependencies:

```bash
git clone https://github.com/solana-turbin3/Q1_26_Accel_ShadowBook.git
cd Q1_26_Accel_ShadowBook
yarn install
```

2. Verify the build passes:

```bash
anchor build
```

3. Read the implementation spec before writing any code:

```
docs/implementation-spec.md
```

This document contains the full account layouts, instruction specifications, matching algorithm, and design decisions. Treat it as the source of truth.

4. Study the reference repositories listed in the README. In particular, if you are working on Chunk C (PER integration), study [private-payments-demo](https://github.com/magicblock-labs/private-payments-demo) thoroughly before starting.

---

## Picking a Chunk

Work is divided into independent chunks labeled A through H. Each instruction stub in the codebase contains `TODO (Chunk X)` comments that describe exactly what to implement.

To find all work items for a chunk:

```bash
grep -rn "TODO (Chunk A)" programs/ tests/
```

Before starting, announce which chunk you are taking so the team avoids duplicate effort. If a chunk is already claimed, coordinate with the owner before making changes to their files.

### Chunk Dependencies

```
A (account structs) ──┐
B (token flow) ───────┤──> D (SDK + E2E tests)
C (PER/TEE) ──────────┘
```

- A, B, and C can start in parallel.
- D depends on all three for final integration, but can scaffold types and stubs immediately.
- Phase 2 chunks (E-H) start after the Phase 1 E2E test passes.

---

## Branch and Commit Conventions

### Branches

Create a branch from `main` for each chunk or feature:

```
chunk-a/account-structs
chunk-b/deposit-withdraw
chunk-c/per-delegation
chunk-d/sdk-client
fix/oracle-band-overflow
```

### Commit Messages

Use the format: `type: short description`

Types:
- `feat` -- new functionality
- `fix` -- bug fix
- `refactor` -- restructuring without behavior change
- `test` -- adding or updating tests
- `docs` -- documentation only
- `chore` -- build config, CI, dependencies

Keep the subject line under 72 characters. Use the body for context when the change is not obvious from the diff.

```
feat: implement insert_bid with price-time priority sort

Binary search for insertion point. Shift elements right to maintain
sorted invariant: price DESC, then timestamp ASC for equal prices.
Returns the insertion index for caller use.
```

Do not include unrelated changes in a commit. One logical change per commit.

---

## Code Standards

### Rust (programs/)

- Follow the existing module structure. Each instruction lives in its own file under `instructions/`.
- Use the error codes defined in `errors.rs`. Do not use generic `msg!()` for recoverable errors.
- Use the helpers in `helpers.rs` for order book operations. Do not duplicate insertion/removal logic.
- All account fields that need alignment padding must be explicit (no implicit `#[repr(C)]` padding). The struct must pass `derive(Pod)` without warnings.
- Validate all inputs at the top of the handler before mutating state.
- Bound all iterations. Every loop that touches the order book must have a maximum iteration count to prevent compute exhaustion.
- Prefix unused parameters with `_` in stub handlers. Remove the prefix when you implement the handler.
- Run `cargo clippy` before submitting. Fix all warnings.

### TypeScript (sdk/, tests/)

- Use strict TypeScript (`strict: true` in tsconfig).
- Use `bigint` for all on-chain numeric values (prices, sizes, amounts). Do not use `number` for values that could exceed `Number.MAX_SAFE_INTEGER`.
- Import from `../setup/test-context` for shared test state. Do not duplicate provider or keypair setup.
- Each test file covers one instruction. Do not combine unrelated instructions in a single test file.

---

## Testing Requirements

### Before Submitting a PR

1. All existing tests must still pass:

```bash
anchor test
```

2. Add tests for every code path you implement. At minimum:
   - One happy-path test per instruction.
   - One test per error condition (e.g., unauthorized signer, insufficient balance).
   - One boundary test if applicable (e.g., full order book, max matches per call).

3. If you implement a Rust helper function, add unit tests in the `#[cfg(test)]` module at the bottom of `helpers.rs`.

### Test Organization

```
tests/
  setup/         Shared context (do not put test cases here)
  market/        Tests for initialize_market, deposit, withdraw
  orders/        Tests for create_order, cancel_order, claim_expired
  per/           Tests for delegate, submit_size, match, cancel (PER)
  settlement/    Tests for settle, collect_fees
  e2e/           Full lifecycle integration tests
```

Place your test in the directory matching the instruction's domain. Name the file `<instruction-name>.test.ts`.

### PER Tests

Tests under `tests/per/` and `tests/e2e/` require a connection to MagicBlock's TEE endpoint on devnet. They will not pass on localnet. Mark these with a descriptive comment at the top of the file noting the requirement.

---

## Pull Request Process

1. Create your branch from `main`. Keep it up to date with `main` by rebasing before submitting.

2. Ensure the build passes (`anchor build`) with zero errors and zero warnings.

3. Ensure all tests pass (`anchor test`).

4. Open a PR with:
   - A title under 72 characters.
   - A summary section explaining what changed and why.
   - A test plan section describing how the changes were verified.

5. Request review from at least one other team member.

6. Do not merge your own PR. Wait for approval.

### PR Size

Keep PRs focused. A good PR implements one instruction end-to-end (handler + accounts struct + tests) or one helper module with tests. If your chunk is large (e.g., Chunk C), break it into multiple PRs:

```
chunk-c/delegate-market     (delegate_market instruction + tests)
chunk-c/submit-order-size   (submit_order_size instruction + tests)
chunk-c/match-orders        (matching engine + oracle check + tests)
```

---

## Architecture Notes

These are decisions that have already been made. Do not deviate from them without team discussion.

- **Single MarketState account** containing the full order book. Do not create per-order PDA accounts.
- **EATA pattern** from MagicBlock SDK for token vaults. Do not implement custom vault balance tracking.
- **Epoch-based batching** for delegation. Orders are placed on mainnet, then the market is delegated for matching. Do not implement continuous delegation.
- **Midpoint fill pricing** with oracle sanity check. Do not use oracle price as the fill price.
- **Empty permission list** for maximum privacy. Do not add wallet pubkeys to the Permission Program members list.
- **Full fills only** in Phase 1. Partial fills are Phase 2 (Chunk E).

See `docs/implementation-spec.md` for the full rationale behind each decision.
