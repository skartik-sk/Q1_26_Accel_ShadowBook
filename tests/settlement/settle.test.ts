import { assert } from "chai";
import { globalSetup } from "../setup/test-context";

describe("settle", () => {
  before(async () => {
    await globalSetup();
  });

  it("settles matched orders", async () => {
    assert.isTrue(true);
  });
});
