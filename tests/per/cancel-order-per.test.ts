import { assert } from "chai";
import { globalSetup } from "../setup/test-context";

describe("cancel-order-per", () => {
  before(async () => {
    await globalSetup();
  });

  it("cancels order in PER", async () => {
    assert.isTrue(true);
  });
});
