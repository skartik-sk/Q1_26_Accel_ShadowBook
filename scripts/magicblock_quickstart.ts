import { Keypair, Connection } from "@solana/web3.js";
import {
  verifyTeeRpcIntegrity,
  getAuthToken,
} from "@magicblock-labs/ephemeral-rollups-sdk";
import nacl from "tweetnacl";

const EPHEMERAL_RPC_URL = "https://tee.magicblock.app"; // Devnet TEE RPC

async function main() {
  const wallet = Keypair.generate();

  console.log("Wallet public key:", wallet.publicKey.toBase58());

  // Verify the integrity of TEE RPC
  console.log("Verifying TEE RPC Integrity for URL:", EPHEMERAL_RPC_URL);
  const isVerified = await verifyTeeRpcIntegrity(EPHEMERAL_RPC_URL);

  console.log("TEE RPC Verified:", isVerified);

  if (!isVerified) {
    throw new Error("TEE RPC integrity verification failed");
  }

  console.log("Getting auth token...");

  // Get AuthToken before making request to TEE
  const token = await getAuthToken(
    EPHEMERAL_RPC_URL,
    wallet.publicKey,
    (message: Uint8Array) =>
      Promise.resolve(nacl.sign.detached(message, wallet.secretKey))
  );

  console.log("Auth token generated successfully!");

  // Test program with the Private Ephemeral Rollup connection:
  const teeConnectionUrl = `${EPHEMERAL_RPC_URL}?token=${token}`;
  console.log("TEE Connection URL ready:", teeConnectionUrl);

  const connection = new Connection(teeConnectionUrl, "confirmed");

  try {
    const slot = await connection.getSlot();
    console.log("Current slot from TEE RPC:", slot);
  } catch (err) {
    console.error("Failed to fetch from TEE connection:", err);
  }
}

main().catch((err) => {
  console.error("Error in quickstart script:", err);
  process.exit(1);
});
