/**
 * ShadowBookClient — typed wrapper for all mainnet instructions.
 *
 * Chunk D: Implement each method to build, sign, send, and confirm txs.
 */

import { Connection, PublicKey, Keypair, TransactionSignature } from "@solana/web3.js";
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor";

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

export const MARKET_SEED = Buffer.from("market");
export const VAULT_SEED = Buffer.from("vault");
export const FEE_VAULT_SEED = Buffer.from("fee_vault");

// External program IDs
export const DELEGATION_PROGRAM_ID = new PublicKey("DELeGGvXpWV2fqJUhqcF5ZSYMS4JTLjteaAMARRSaeSh");
export const PERMISSION_PROGRAM_ID = new PublicKey("ACLseoPoyC3cBqoUtkbjZ4aDrkurZW86v19pXz2XQnp1");
export const ORACLE_PROGRAM_ID = new PublicKey("PriCems5tHihc6UDXDjzjeawomAwBduWMGAi8ZUjppd");
export const DEVNET_TEE_VALIDATOR = new PublicKey("FnE6VJT5QNZdedZPnCoLsARgBwoE6DeJNjBs2H1gySXA");
export const MAINNET_TEE_VALIDATOR = new PublicKey("MTEWGuqxUpYZGFJQcp8tLN7x5v9BSeoFHYWQQ3n3xzo");

// ---------------------------------------------------------------------------
// PDA derivation
// ---------------------------------------------------------------------------

export function deriveMarketAddress(
  programId: PublicKey,
  mintA: PublicKey,
  mintB: PublicKey,
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [MARKET_SEED, mintA.toBuffer(), mintB.toBuffer()],
    programId,
  );
}

export function deriveOracleFeedAddress(feedId: Buffer): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("price_feed"), Buffer.from("pyth-lazer"), feedId],
    ORACLE_PROGRAM_ID,
  );
}

// ---------------------------------------------------------------------------
// Client
// ---------------------------------------------------------------------------

export class ShadowBookClient {
  constructor(
    public readonly connection: Connection,
    public readonly programId: PublicKey,
    public readonly wallet: Wallet,
  ) {}

  // TODO (Chunk D): Implement each method

  async initializeMarket(
    mintA: PublicKey,
    mintB: PublicKey,
    feeRateBps: number,
    keeperRewardBps: number,
    oracleFeedId: Buffer,
  ): Promise<TransactionSignature> {
    throw new Error("Not implemented — Chunk D");
  }

  async deposit(
    market: PublicKey,
    mint: PublicKey,
    amount: bigint,
  ): Promise<TransactionSignature> {
    throw new Error("Not implemented — Chunk D");
  }

  async withdraw(
    market: PublicKey,
    mint: PublicKey,
    amount: bigint,
  ): Promise<TransactionSignature> {
    throw new Error("Not implemented — Chunk D");
  }

  async createOrder(
    market: PublicKey,
    side: number,
    price: bigint,
  ): Promise<TransactionSignature> {
    throw new Error("Not implemented — Chunk D");
  }

  async cancelOrder(
    market: PublicKey,
    orderId: bigint,
    side: number,
  ): Promise<TransactionSignature> {
    throw new Error("Not implemented — Chunk D");
  }

  async delegateMarket(market: PublicKey): Promise<TransactionSignature> {
    throw new Error("Not implemented — Chunk D");
  }

  async settle(market: PublicKey): Promise<TransactionSignature> {
    throw new Error("Not implemented — Chunk D");
  }

  async claimExpired(market: PublicKey): Promise<TransactionSignature> {
    throw new Error("Not implemented — Chunk D");
  }

  async collectFees(market: PublicKey): Promise<TransactionSignature> {
    throw new Error("Not implemented — Chunk D");
  }
}
