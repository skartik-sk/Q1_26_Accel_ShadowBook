/**
 * Shared test context — provider, program, keypairs, mints.
 *
 * Every test file imports from here instead of duplicating setup logic.
 * Chunk D: Wire up program loading and account creation.
 */

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair, Connection } from "@solana/web3.js";

// TODO (Chunk D): Import generated IDL type after first `anchor build`
// import { ShadowBook } from "../../target/types/shadow_book";

// ---------------------------------------------------------------------------
// Provider
// ---------------------------------------------------------------------------

export const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

// TODO (Chunk D): Load program from workspace
// export const program = anchor.workspace.ShadowBook as Program<ShadowBook>;
// export const programId = program.programId;

// ---------------------------------------------------------------------------
// Test Keypairs
// ---------------------------------------------------------------------------

export const authority = Keypair.generate();
export const traderA = Keypair.generate();
export const traderB = Keypair.generate();
export const cranker = Keypair.generate();

// ---------------------------------------------------------------------------
// Test Mints & Market (populated by globalSetup)
// ---------------------------------------------------------------------------

export let mintA: PublicKey;
export let mintB: PublicKey;
export let marketPda: PublicKey;
export let marketBump: number;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

export async function airdrop(address: PublicKey, lamports = 10_000_000_000) {
  const sig = await provider.connection.requestAirdrop(address, lamports);
  await provider.connection.confirmTransaction(sig, "confirmed");
}

/**
 * Call in a top-level `before()` to bootstrap test accounts.
 * Idempotent — safe to call multiple times.
 */
export async function globalSetup() {
  // TODO (Chunk D):
  // 1. Airdrop SOL to authority, traderA, traderB, cranker
  // 2. Create test SPL mints (mintA, mintB)
  // 3. Create ATAs for each trader + authority
  // 4. Mint tokens to traders (e.g. 1_000_000 each)
  // 5. Derive market PDA:
  //    [marketPda, marketBump] = PublicKey.findProgramAddressSync(
  //      [Buffer.from("market"), mintA.toBuffer(), mintB.toBuffer()],
  //      programId,
  //    );
}
