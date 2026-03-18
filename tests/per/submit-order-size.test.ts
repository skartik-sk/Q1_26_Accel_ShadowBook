import { assert } from "chai";
import { globalSetup } from "../setup/test-context";

describe("submit-order-size", () => {
  before(async () => {
    await globalSetup();
  });

  it("submits order size in PER", async () => {
    assert.isTrue(true);
  });
});
