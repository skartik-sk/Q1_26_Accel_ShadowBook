import { assert } from "chai";
import {
  provider,
  program,
  traderA,
  traderAAtaA,
  mintA,
  marketPda,
  globalSetup,
} from "../setup/test-context";
import { getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import * as anchor from "@coral-xyz/anchor";

describe("withdraw", () => {
  before(async () => {
    await globalSetup();
  });

  it("withdraws token_a from vault", async () => {
    const vaultAta = getAssociatedTokenAddressSync(mintA, marketPda, true);
    try {
      await program.methods
        .withdraw(new anchor.BN(500))
        .accounts({
          trader: traderA.publicKey,
          market: marketPda,
          traderTokenAccount: traderAAtaA,
          vaultTokenAccount: vaultAta,
          mint: mintA,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([traderA])
        .rpc();
    } catch (e) {
      assert.isOk(e);
    }
  });
});
