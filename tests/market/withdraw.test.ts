/**
 * Tests: withdraw instruction.
 * Chunk: B / D
 */

import { assert } from "chai";
import { provider, traderA, globalSetup } from "../setup/test-context";

describe("withdraw", () => {
  before(async () => {
    await globalSetup();
    // TODO: Deposit tokens first so there's a balance to withdraw
  });

  it("withdraws available balance to trader ATA", async () => {
    // TODO (Chunk B/D):
    // 1. Call withdraw(amount=500, mint=mintA) as traderA
    // 2. Verify vault ATA balance decreased
    // 3. Verify trader ATA balance increased
    // 4. Verify EATA balance decreased
  });

  it("rejects withdraw exceeding available balance", async () => {
    // TODO (Chunk B/D):
    // InsufficientBalance error
  });

  it("rejects withdraw when funds are locked by open orders", async () => {
    // TODO (Chunk B/D):
    // 1. Create an order locking some balance
    // 2. Try to withdraw full balance
    // 3. Expect FundsLocked error
  });
});
