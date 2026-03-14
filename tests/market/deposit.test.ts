/**
 * Tests: deposit instruction.
 * Chunk: B / D
 */

import { assert } from "chai";
import {
  provider,
  traderA,
  traderB,
  globalSetup,
} from "../setup/test-context";

describe("deposit", () => {
  before(async () => {
    await globalSetup();
  });

  it("deposits token_a into vault and updates EATA balance", async () => {
    // TODO (Chunk B/D):
    // 1. Call deposit(amount=1000, mint=mintA) as traderA
    // 2. Verify vault ATA balance increased by 1000
    // 3. Verify trader's EATA balance == 1000
    // 4. Verify trader's ATA balance decreased by 1000
  });

  it("deposits token_b into vault", async () => {
    // TODO (Chunk B/D):
    // Same flow with mintB
  });

  it("handles multiple deposits from same trader (additive)", async () => {
    // TODO (Chunk B/D):
    // 1. Deposit 500 token_a
    // 2. Deposit 300 token_a
    // 3. Verify EATA balance == 1800 (1000 + 500 + 300)
  });

  it("rejects deposit with zero amount", async () => {
    // TODO (Chunk B/D):
    // Expect an error when amount == 0
  });
});
