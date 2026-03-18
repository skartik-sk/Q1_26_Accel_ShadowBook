import { assert } from "chai";
import { SystemProgram, Keypair, PublicKey } from "@solana/web3.js";
import {
  program,
  authority,
  globalSetup,
  mintA,
  mintB,
  marketPda,
} from "../setup/test-context";

describe("initialize_market", () => {
  const oracleFeedId = Array(32).fill(0);
  oracleFeedId[0] = 1; // Dummy ID

  before(async () => {
    await globalSetup();
  });

  it("creates a market with correct parameters", async () => {
    const feeRateBps = 30;
    const keeperRewardBps = 5;

    await program.methods
      .initializeMarket(feeRateBps, keeperRewardBps, oracleFeedId)
      .accounts({
        authority: authority.publicKey,
        mintA: mintA,
        mintB: mintB,
        market: marketPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    // Fetch MarketState
    const marketState = await program.account.marketState.fetch(marketPda);

    // Assertions
    assert.deepEqual(marketState.mintA, Array.from(mintA.toBuffer()));
    assert.deepEqual(marketState.mintB, Array.from(mintB.toBuffer()));
    assert.deepEqual(
      marketState.authority,
      Array.from(authority.publicKey.toBuffer()),
    );
    assert.equal(marketState.feeRateBps, feeRateBps);
    assert.equal(marketState.keeperRewardBps, keeperRewardBps);
    assert.deepEqual(marketState.oracleFeedId, oracleFeedId);
    assert.equal(marketState.bidCount, 0);
    assert.equal(marketState.askCount, 0);
    assert.equal(marketState.isDelegated, 0);
    assert.equal(marketState.matchCount, 0);
  });

  it("rejects duplicate market for the same token pair", async () => {
    const feeRateBps = 30;
    const keeperRewardBps = 5;

    try {
      await program.methods
        .initializeMarket(feeRateBps, keeperRewardBps, oracleFeedId)
        .accounts({
          authority: authority.publicKey,
          mintA: mintA,
          mintB: mintB,
          market: marketPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

      assert.fail("Should have thrown an error for duplicate market");
    } catch (err: any) {
      assert.include(err.message, "already in use");
    }
  });

  it("rejects fee rate above 10%", async () => {
    const feeRateBps = 1001; // > 10%
    const keeperRewardBps = 5;

    // Use dummy mints so we don't hit the PDA already in use error
    const dummyMint1 = Keypair.generate().publicKey;
    const dummyMint2 = Keypair.generate().publicKey;
    const [dummyMarketPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("market"), dummyMint1.toBuffer(), dummyMint2.toBuffer()],
      program.programId,
    );

    try {
      await program.methods
        .initializeMarket(feeRateBps, keeperRewardBps, oracleFeedId)
        .accounts({
          authority: authority.publicKey,
          mintA: dummyMint1,
          mintB: dummyMint2,
          market: dummyMarketPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();

      assert.fail("Should have thrown FeeRateTooHigh error");
    } catch (err: any) {
      assert.include(err.message, "FeeRateTooHigh");
    }
  });
});
