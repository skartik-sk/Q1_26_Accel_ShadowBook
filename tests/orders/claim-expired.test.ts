/**
 * Tests: claim_expired instruction.
 * Chunk: A / D
 */

import { assert } from "chai";
import { provider, traderA, cranker, globalSetup } from "../setup/test-context";

describe("claim_expired", () => {
  before(async () => {
    await globalSetup();
  });

  it("removes expired orders from the book", async () => {
    // TODO (Chunk A/D):
    // 1. Create an order with a short TTL (or manipulate clock in localnet)
    // 2. Advance clock past expires_at
    // 3. Call claim_expired as cranker (permissionless)
    // 4. Verify order removed, bid_count/ask_count decremented
  });

  it("leaves non-expired orders untouched", async () => {
    // TODO (Chunk A/D):
    // 1. Create two orders: one expired, one still valid
    // 2. Call claim_expired
    // 3. Verify only the expired one is removed
  });

  it("respects MAX_EXPIRED_CLEANUP_PER_CALL bound", async () => {
    // TODO (Chunk A/D):
    // 1. Create 15 expired orders
    // 2. Call claim_expired
    // 3. Verify only 10 were removed (bounded)
    // 4. Call again to remove remaining 5
  });

  it("is permissionless — anyone can call", async () => {
    // TODO (Chunk A/D):
    // Call as cranker (not the order owner), verify it works
  });
});
