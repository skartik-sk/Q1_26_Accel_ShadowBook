/**
 * Tests: delegate_market instruction.
 * Chunk: C / D
 */

import { assert } from "chai";
import { provider, cranker, traderA, traderB, globalSetup } from "../setup/test-context";

describe("delegate_market", () => {
  before(async () => {
    await globalSetup();
    // TODO: Deposit and place at least 1 bid + 1 ask
  });

  it("delegates market to TEE validator", async () => {
    // TODO (Chunk C/D):
    // 1. Call delegate_market as cranker
    // 2. Fetch MarketState, assert is_delegated == true
    // 3. Assert delegated_at is recent timestamp
    // 4. Verify account owner changed to DELEGATION_PROGRAM_ID
  });

  it("rejects delegation with no bids", async () => {
    // TODO (Chunk C/D):
    // Market with only asks -> InsufficientOrdersToDelegate
  });

  it("rejects delegation with no asks", async () => {
    // TODO (Chunk C/D):
    // Market with only bids -> InsufficientOrdersToDelegate
  });

  it("rejects double delegation", async () => {
    // TODO (Chunk C/D):
    // Already delegated -> MarketDelegated
  });
});
