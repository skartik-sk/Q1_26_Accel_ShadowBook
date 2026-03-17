/**
 * Shared test context — provider, program, keypairs, mints.
 *
 * Every test file imports from here instead of duplicating setup logic.
 */

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { ShadowBook } from "../../target/types/shadow_book";

// ---------------------------------------------------------------------------
// Provider
// ---------------------------------------------------------------------------

export const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

export const program = anchor.workspace.ShadowBook as Program<ShadowBook>;
export const programId = program.programId;

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

export let traderAAtaA: PublicKey;
export let traderAAtaB: PublicKey;
export let traderBAtaA: PublicKey;
export let traderBAtaB: PublicKey;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

export async function airdrop(address: PublicKey, lamports = 10_000_000_000) {
  const sig = await provider.connection.requestAirdrop(address, lamports);
  const latestBlockhash = await provider.connection.getLatestBlockhash();
  await provider.connection.confirmTransaction(
    {
      signature: sig,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    },
    "confirmed",
  );
}

let setupDone = false;

/**
 * Call in a top-level `before()` to bootstrap test accounts.
 * Idempotent — safe to call multiple times.
 */
export async function globalSetup() {
  if (setupDone) return;

  // 1. Airdrop SOL
  await airdrop(authority.publicKey);
  await airdrop(traderA.publicKey);
  await airdrop(traderB.publicKey);
  await airdrop(cranker.publicKey);

  // 2. Create test SPL mints
  mintA = await createMint(
    provider.connection,
    authority,
    authority.publicKey,
    null,
    6,
  );

  mintB = await createMint(
    provider.connection,
    authority,
    authority.publicKey,
    null,
    6,
  );

  // Ensure consistent sorting of mints for PDA derivation (like standard DEXes)
  if (mintA.toBuffer().compare(mintB.toBuffer()) > 0) {
    const temp = mintA;
    mintA = mintB;
    mintB = temp;
  }

  // 3. Create ATAs for each trader
  traderAAtaA = (
    await getOrCreateAssociatedTokenAccount(
      provider.connection,
      traderA,
      mintA,
      traderA.publicKey,
    )
  ).address;
  traderAAtaB = (
    await getOrCreateAssociatedTokenAccount(
      provider.connection,
      traderA,
      mintB,
      traderA.publicKey,
    )
  ).address;

  traderBAtaA = (
    await getOrCreateAssociatedTokenAccount(
      provider.connection,
      traderB,
      mintA,
      traderB.publicKey,
    )
  ).address;
  traderBAtaB = (
    await getOrCreateAssociatedTokenAccount(
      provider.connection,
      traderB,
      mintB,
      traderB.publicKey,
    )
  ).address;

  // 4. Mint tokens to traders
  const mintAmount = 1_000_000 * 1_000_000; // 1 million tokens (6 decimals)
  await mintTo(
    provider.connection,
    authority,
    mintA,
    traderAAtaA,
    authority,
    mintAmount,
  );
  await mintTo(
    provider.connection,
    authority,
    mintB,
    traderAAtaB,
    authority,
    mintAmount,
  );
  await mintTo(
    provider.connection,
    authority,
    mintA,
    traderBAtaA,
    authority,
    mintAmount,
  );
  await mintTo(
    provider.connection,
    authority,
    mintB,
    traderBAtaB,
    authority,
    mintAmount,
  );

  // 5. Derive market PDA
  [marketPda, marketBump] = PublicKey.findProgramAddressSync(
    [Buffer.from("market"), mintA.toBuffer(), mintB.toBuffer()],
    programId,
  );

  setupDone = true;
}
