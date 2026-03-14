/**
 * Tests: cancel_order instruction (mainnet).
 * Chunk: A / D
 */

import { assert } from "chai";
import {
  provider,
  traderA,
  traderB,
  globalSetup,
} from "../setup/test-context";

describe("cancel_order", () => {
  before(async () => {
    await globalSetup();
    // TODO: Deposit and create an order to cancel
  });

  it("cancels an open order and removes it from the book", async () => {
    // TODO (Chunk A/D):
    // 1. Create a buy order
    // 2. Call cancel_order(order_id, side=Buy) as the order owner
    // 3. Fetch MarketState, verify bid_count decremented
    // 4. Verify the order is no longer in the bids array
  });

  it("rejects cancel by a non-owner", async () => {
    // TODO (Chunk A/D):
    // 1. Create order as traderA
    // 2. Call cancel_order as traderB
    // 3. Expect UnauthorizedOrderAccess error
  });

  it("rejects cancel when market is delegated", async () => {
    // TODO (Chunk A/D):
    // Expect MarketDelegated error
  });

  it("rejects cancel of already-matched order", async () => {
    // TODO (Chunk A/D):
    // Expect OrderNotOpen error
  });
});
