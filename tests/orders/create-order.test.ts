/**
 * Tests: create_order instruction.
 * Chunk: C / D
 */

import { assert } from "chai";
import {
  provider,
  traderA,
  traderB,
  globalSetup,
} from "../setup/test-context";

describe("create_order", () => {
  before(async () => {
    await globalSetup();
    // TODO: Deposit tokens for both traders
  });

  it("places a buy order with correct fields", async () => {
    // TODO (Chunk C/D):
    // 1. Call create_order(side=Buy, price=100) as traderA
    // 2. Fetch MarketState
    // 3. Assert bid_count == 1
    // 4. Assert bids[0].trader == traderA.publicKey
    // 5. Assert bids[0].price == 100
    // 6. Assert bids[0].size == 0 (placeholder — size written in TEE)
    // 7. Assert bids[0].status == Open
    // 8. Assert bids[0].order_id == 0 (first order)
  });

  it("places a sell order", async () => {
    // TODO (Chunk C/D):
    // Similar assertions on asks array
  });

  it("maintains price-time priority sort for bids", async () => {
    // TODO (Chunk C/D):
    // 1. Place bid at price=100, then bid at price=110
    // 2. Verify bids[0].price == 110 (higher price first)
    // 3. Place another bid at price=110
    // 4. Verify bids[0] has earlier timestamp than bids[1]
  });

  it("maintains price-time priority sort for asks", async () => {
    // TODO (Chunk C/D):
    // 1. Place ask at price=100, then ask at price=90
    // 2. Verify asks[0].price == 90 (lower price first)
  });

  it("increments next_order_id monotonically", async () => {
    // TODO (Chunk C/D):
    // Place 3 orders, verify order_ids are 0, 1, 2
  });

  it("rejects order when market is delegated", async () => {
    // TODO (Chunk C/D):
    // 1. Delegate market
    // 2. Try create_order
    // 3. Expect MarketDelegated error
  });

  it("rejects when order book is full", async () => {
    // TODO (Chunk C/D):
    // Fill 256 bids, try to place one more
    // Expect OrderBookFull error
  });
});
