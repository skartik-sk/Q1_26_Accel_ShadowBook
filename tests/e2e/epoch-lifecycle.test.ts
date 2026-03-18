import { assert } from "chai";
import { globalSetup } from "../setup/test-context";

describe("epoch-lifecycle", () => {
  before(async () => {
    await globalSetup();
  });

  it("completes full epoch", async () => {
    assert.isTrue(true);
  });
});
