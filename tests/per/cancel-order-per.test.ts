/**
 * Tests: cancel_order_per instruction (PER-only).
 * Chunk: C / D
 */

import { assert } from "chai";
import { provider, traderA, globalSetup } from "../setup/test-context";

describe("cancel_order_per (PER)", () => {
  before(async () => {
    await globalSetup();
    // TODO: Deposit, create order, delegate market
  });

  it("cancels order inside TEE and commits", async () => {
    // TODO (Chunk C/D):
    // 1. Cancel order via PER
    // 2. Wait for commit
    // 3. Verify order removed from book on mainnet
  });

  it("undelegates market if book is empty after cancel", async () => {
    // TODO (Chunk C/D):
    // Only order in the book gets cancelled
    // -> market should auto-undelegate (commit_and_undelegate)
    // -> is_delegated == false on mainnet
  });

  it("rejects cancel by non-owner", async () => {
    // TODO (Chunk C/D):
    // UnauthorizedOrderAccess
  });
});
