/**
 * Tests: submit_order_size instruction (PER-only).
 * Chunk: C / D
 *
 * These tests require a PER connection to tee.magicblock.app (devnet).
 * They will not pass on localnet without a TEE validator.
 */

import { assert } from "chai";
import { provider, traderA, traderB, globalSetup } from "../setup/test-context";

describe("submit_order_size (PER)", () => {
  before(async () => {
    await globalSetup();
    // TODO: Deposit, create orders, delegate market
    // TODO: Connect to PER via ShadowBookPERClient
  });

  it("writes size into order inside TEE", async () => {
    // TODO (Chunk C/D):
    // 1. Call submitOrderSize(orderId=0, size=50) via PER connection
    // 2. Query MarketState via PER — verify size == 50
  });

  it("size is NOT visible on mainnet (privacy test)", async () => {
    // TODO (Chunk C/D):
    // 1. After submitting size via PER
    // 2. Query MarketState via MAINNET connection
    // 3. Assert order size is still 0 (pre-delegation state)
    // This is the core privacy assertion.
  });

  it("rejects size submission by non-owner", async () => {
    // TODO (Chunk C/D):
    // Sign as traderB for traderA's order -> UnauthorizedOrderAccess
  });

  it("rejects zero size", async () => {
    // TODO (Chunk C/D):
    // size=0 -> ZeroSize error
  });

  it("rejects when market is not delegated", async () => {
    // TODO (Chunk C/D):
    // MarketNotDelegated error
  });
});
