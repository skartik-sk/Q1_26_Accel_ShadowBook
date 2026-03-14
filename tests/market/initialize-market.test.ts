/**
 * Tests: initialize_market instruction.
 * Chunk: A / D
 */

import { assert } from "chai";
import { provider, authority, globalSetup } from "../setup/test-context";

describe("initialize_market", () => {
  before(async () => {
    await globalSetup();
  });

  it("creates a market with correct parameters", async () => {
    // TODO (Chunk A/D):
    // 1. Call initialize_market(fee_rate=30, keeper_reward=5, oracle_feed_id)
    // 2. Fetch MarketState via AccountLoader
    // 3. Assert mint_a, mint_b match expected
    // 4. Assert authority matches signer
    // 5. Assert fee_rate_bps == 30, keeper_reward_bps == 5
    // 6. Assert bid_count == 0, ask_count == 0
    // 7. Assert is_delegated == false
  });

  it("rejects duplicate market for the same token pair", async () => {
    // TODO (Chunk A/D):
    // Calling initialize_market again with same mints should fail
    // with an "already in use" error (PDA already exists).
  });

  it("rejects fee rate above 10%", async () => {
    // TODO (Chunk A/D):
    // Call with fee_rate_bps = 1001 (>10%), expect FeeRateTooHigh error.
  });
});
