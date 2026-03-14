/**
 * ShadowBookCrank — automated matching operator.
 *
 * Runs the epoch lifecycle:
 * 1. Polls market state.
 * 2. If not delegated + has bids & asks → delegate.
 * 3. If delegated → wait for size submissions, then match.
 * 4. After match + commit → settle to collect keeper reward.
 *
 * Chunk D: Implement the crank loop.
 */

import { PublicKey } from "@solana/web3.js";
import { ShadowBookPERClient } from "./per-client";

export interface CrankConfig {
  /** How often to poll market state (ms). */
  pollIntervalMs: number;

  /** Minimum time to wait after delegation before matching (ms).
   *  Gives traders time to submit sizes. */
  sizeSubmissionWindowMs: number;
}

const DEFAULT_CONFIG: CrankConfig = {
  pollIntervalMs: 5_000,
  sizeSubmissionWindowMs: 30_000,
};

export class ShadowBookCrank {
  private running = false;
  private intervalHandle: NodeJS.Timeout | null = null;

  constructor(
    private readonly client: ShadowBookPERClient,
    private readonly config: CrankConfig = DEFAULT_CONFIG,
  ) {}

  /**
   * Start the crank loop for a given market.
   */
  async start(market: PublicKey): Promise<void> {
    if (this.running) throw new Error("Crank is already running");
    this.running = true;

    // TODO (Chunk D): Implement crank loop
    //
    // this.intervalHandle = setInterval(async () => {
    //   try {
    //     const marketState = await this.readMarketState(market);
    //
    //     if (!marketState.isDelegated) {
    //       if (marketState.bidCount > 0 && marketState.askCount > 0) {
    //         await this.client.delegateMarket(market);
    //         console.log("[crank] Market delegated");
    //       }
    //     } else {
    //       // Check if enough time has passed for size submissions
    //       const elapsed = Date.now() / 1000 - marketState.delegatedAt;
    //       if (elapsed * 1000 >= this.config.sizeSubmissionWindowMs) {
    //         await this.client.matchOrders(market);
    //         console.log("[crank] Orders matched, waiting for undelegation...");
    //         await this.client.waitForUndelegation(market);
    //         await this.client.settle(market);
    //         console.log("[crank] Settlement complete");
    //       }
    //     }
    //   } catch (err) {
    //     console.error("[crank] Error:", err);
    //   }
    // }, this.config.pollIntervalMs);

    console.log("[crank] Not implemented — Chunk D");
  }

  /**
   * Stop the crank loop.
   */
  stop(): void {
    this.running = false;
    if (this.intervalHandle) {
      clearInterval(this.intervalHandle);
      this.intervalHandle = null;
    }
    console.log("[crank] Stopped");
  }
}
