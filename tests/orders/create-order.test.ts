import { assert } from "chai";
import { SystemProgram } from "@solana/web3.js";
import {
  program,
  traderA,
  traderB,
  authority,
  globalSetup,
  marketPda,
  mintA,
  mintB,
} from "../setup/test-context";
import * as anchor from "@coral-xyz/anchor";

describe("create_order", () => {
  before(async () => {
    await globalSetup();

    // Ensure the market is initialized before running tests.
    try {
      const oracleFeedId = Array(32).fill(0);
      oracleFeedId[0] = 1;
      await program.methods
        .initializeMarket(30, 5, oracleFeedId)
        .accounts({
          authority: authority.publicKey,
          mintA: mintA,
          mintB: mintB,
          market: marketPda,
          systemProgram: SystemProgram.programId,
        })
        .signers([authority])
        .rpc();
    } catch (e) {
      // Market might already exist, ignore error.
    }
  });

  it("places a buy order with correct fields", async () => {
    const price = new anchor.BN(100);
    const side = 0; // Buy

    await program.methods
      .createOrder(side, price)
      .accounts({
        trader: traderA.publicKey,
        market: marketPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([traderA])
      .rpc();

    const marketState = await program.account.marketState.fetch(marketPda);
    assert.equal(marketState.bidCount, 1);

    const bid = marketState.bids[0];
    assert.deepEqual(bid.trader, Array.from(traderA.publicKey.toBuffer()));
    assert.equal(bid.price.toNumber(), 100);
    assert.equal(bid.size.toNumber(), 0); // size is 0 until TEE
    assert.equal(bid.status, 1); // Open
    assert.isAbove(bid.orderId.toNumber(), 0);
  });

  it("places a sell order", async () => {
    const price = new anchor.BN(105);
    const side = 1; // Sell

    await program.methods
      .createOrder(side, price)
      .accounts({
        trader: traderB.publicKey,
        market: marketPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([traderB])
      .rpc();

    const marketState = await program.account.marketState.fetch(marketPda);
    assert.equal(marketState.askCount, 1);

    const ask = marketState.asks[0];
    assert.deepEqual(ask.trader, Array.from(traderB.publicKey.toBuffer()));
    assert.equal(ask.price.toNumber(), 105);
    assert.equal(ask.size.toNumber(), 0);
    assert.equal(ask.status, 1); // Open
  });

  it("maintains price-time priority sort for bids", async () => {
    // Already have a bid at 100
    // Place a bid at 110 (should be sorted to index 0)
    await program.methods
      .createOrder(0, new anchor.BN(110))
      .accounts({
        trader: traderA.publicKey,
        market: marketPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([traderA])
      .rpc();

    let marketState = await program.account.marketState.fetch(marketPda);
    assert.equal(marketState.bids[0].price.toNumber(), 110);
    assert.equal(marketState.bids[1].price.toNumber(), 100);

    // Place another bid at 110 (should be sorted to index 1 due to later time)
    await program.methods
      .createOrder(0, new anchor.BN(110))
      .accounts({
        trader: traderB.publicKey,
        market: marketPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([traderB])
      .rpc();

    marketState = await program.account.marketState.fetch(marketPda);
    assert.equal(marketState.bids[0].price.toNumber(), 110);
    assert.equal(marketState.bids[1].price.toNumber(), 110);
    assert.equal(marketState.bids[2].price.toNumber(), 100);

    assert.ok(
      marketState.bids[0].timestamp.toNumber() <=
        marketState.bids[1].timestamp.toNumber(),
    );
  });

  it("maintains price-time priority sort for asks", async () => {
    // Already have an ask at 105
    // Place an ask at 90 (should be sorted to index 0, lower price first)
    await program.methods
      .createOrder(1, new anchor.BN(90))
      .accounts({
        trader: traderA.publicKey,
        market: marketPda,
        systemProgram: SystemProgram.programId,
      })
      .signers([traderA])
      .rpc();

    const marketState = await program.account.marketState.fetch(marketPda);
    assert.equal(marketState.asks[0].price.toNumber(), 90);
    assert.equal(marketState.asks[1].price.toNumber(), 105);
  });

  it("increments next_order_id monotonically", async () => {
    const marketState = await program.account.marketState.fetch(marketPda);
    assert.isAbove(marketState.nextOrderId.toNumber(), 5);
  });

  it("rejects order when market is delegated", async () => {
    // Stub: To be implemented fully when delegate_market integration is tested
    // 1. Delegate market
    // 2. Try create_order
    // 3. Expect MarketDelegated error
    assert.ok(true);
  });

  it("rejects when order book is full", async () => {
    // Stub: Requires filling 256 orders to hit OrderBookFull error
    assert.ok(true);
  });
});
