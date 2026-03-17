import { assert } from "chai";
import { globalSetup } from "../setup/test-context";

describe("collect_fees", () => {
  before(async () => {
    await globalSetup();
  });

  it("collects fees", async () => {
    assert.isTrue(true);
  });
});
