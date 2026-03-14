/**
 * TypeScript types matching on-chain account structures.
 *
 * These types are used to deserialize account data on the client side.
 * They must stay in sync with the Rust structs in `state/`.
 */

import { PublicKey } from "@solana/web3.js";

// ---------------------------------------------------------------------------
// Enums
// ---------------------------------------------------------------------------

export enum Side {
  Buy = 0,
  Sell = 1,
}

export enum OrderStatus {
  Empty = 0,
  Open = 1,
  Matched = 2,
  Cancelled = 3,
}

// ---------------------------------------------------------------------------
// Account types
// ---------------------------------------------------------------------------

export interface Order {
  trader: PublicKey;
  orderId: bigint;
  side: Side;
  status: OrderStatus;
  price: bigint;
  size: bigint;
  timestamp: bigint;
  expiresAt: bigint;
  matchedPrice: bigint;
}

export interface MatchResult {
  buyer: PublicKey;
  seller: PublicKey;
  price: bigint;
  size: bigint;
  settled: boolean;
}

export interface MarketState {
  mintA: PublicKey;
  mintB: PublicKey;
  authority: PublicKey;
  totalVolume: bigint;
  feeRateBps: number;
  keeperRewardBps: number;
  oracleFeedId: Uint8Array;
  nextOrderId: bigint;
  bids: Order[];
  asks: Order[];
  bidCount: number;
  askCount: number;
  matchResults: MatchResult[];
  matchCount: number;
  isDelegated: boolean;
  delegatedAt: bigint;
  bump: number;
}

// ---------------------------------------------------------------------------
// Deserialization helpers
// ---------------------------------------------------------------------------

// TODO (Chunk D): Implement zero-copy deserialization from raw account data.
// The MarketState account uses #[account(zero_copy)] so it does NOT have
// standard Anchor borsh encoding. Deserialize using DataView/Buffer reads
// at known offsets matching the Rust #[repr(C)] layout.

export function deserializeMarketState(_data: Buffer): MarketState {
  throw new Error("Not implemented — Chunk D");
}

export function deserializeOrder(_data: Buffer, _offset: number): Order {
  throw new Error("Not implemented — Chunk D");
}

export function deserializeMatchResult(_data: Buffer, _offset: number): MatchResult {
  throw new Error("Not implemented — Chunk D");
}
