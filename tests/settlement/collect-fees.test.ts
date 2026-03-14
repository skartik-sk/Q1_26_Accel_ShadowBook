/**
 * Tests: collect_fees instruction.
 * Chunk: B / D
 */

import { assert } from "chai";
import { provider, authority, traderA, globalSetup } from "../setup/test-context";

describe("collect_fees", () => {
  before(async () => {
    await globalSetup();
    // TODO: Full cycle through settle so fees are accumulated
  });

  it("authority collects accumulated fees", async () => {
    // TODO (Chunk B/D):
    // 1. Call collect_fees as authority
    // 2. Verify fee vault EATA balance decreased
    // 3. Verify authority ATA balance increased
  });

  it("rejects non-authority caller", async () => {
    // TODO (Chunk B/D):
    // Call as traderA -> UnauthorizedAuthority
  });
});
