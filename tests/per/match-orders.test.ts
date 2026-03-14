/**
 * Tests: match_orders instruction (PER-only crank).
 * Chunk: C / D
 *
 * Requires PER connection. Tests the matching engine running inside the TEE.
 */

import { assert } from "chai";
import { provider, traderA, traderB, cranker, globalSetup } from "../setup/test-context";

describe("match_orders (PER)", () => {
  before(async () => {
    await globalSetup();
    // TODO: Full setup — deposit, create orders, delegate, submit sizes
  });

  it("matches crossing orders at midpoint price", async () => {
    // TODO (Chunk C/D):
    // Setup: bid@100, ask@95, both size=50
    // 1. Call matchOrders via PER
    // 2. Wait for undelegation (poll mainnet)
    // 3. Fetch MarketState on mainnet
    // 4. Assert match_count == 1
    // 5. Assert match_results[0].price == 97 (midpoint: (100+95)/2 = 97)
    // 6. Assert match_results[0].size == 50
    // 7. Assert match_results[0].buyer == traderA
    // 8. Assert match_results[0].seller == traderB
    // 9. Assert both orders have status == Matched
  });

  it("skips non-crossing orders", async () => {
    // TODO (Chunk C/D):
    // bid@90, ask@100 -> no match, match_count stays 0
  });

  it("skips orders with unsubmitted sizes (size == 0)", async () => {
    // TODO (Chunk C/D):
    // Create order but don't submit size -> not matched
  });

  it("respects MAX_MATCHES_PER_CALL bound", async () => {
    // TODO (Chunk C/D):
    // Create 15 crossing pairs, verify only 10 matched per call
  });

  it("rejects fill price outside oracle sanity band", async () => {
    // TODO (Chunk C/D):
    // Set up orders with prices far from oracle price
    // Expect OracleSanityCheckFailed (or match skipped)
  });

  it("sets is_delegated to false after commit", async () => {
    // TODO (Chunk C/D):
    // After match + commit + undelegate, verify market is back on mainnet
  });
});
