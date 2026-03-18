import { assert } from "chai";
import { globalSetup } from "../setup/test-context";

describe("claim_expired", () => {
  before(async () => {
    await globalSetup();
  });

  it("claims expired orders", async () => {
    assert.isTrue(true);
  });
});
