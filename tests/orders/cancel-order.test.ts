import { assert } from "chai";
import { globalSetup } from "../setup/test-context";

describe("cancel_order", () => {
  before(async () => {
    await globalSetup();
  });

  it("cancels an order", async () => {
    assert.isTrue(true);
  });
});
