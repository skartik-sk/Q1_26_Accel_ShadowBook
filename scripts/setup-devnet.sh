#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# Shadow Book — Devnet setup script
#
# Sets up a devnet wallet with SOL, creates test mints, and deploys the
# program. Run this once when onboarding a new dev.
# ---------------------------------------------------------------------------

set -euo pipefail

echo "=== Shadow Book Devnet Setup ==="

# 1. Configure Solana CLI for devnet
solana config set --url devnet
echo "[1/5] Configured for devnet"

# 2. Generate keypair if none exists
if [ ! -f ~/.config/solana/id.json ]; then
  solana-keygen new --no-bip39-passphrase -o ~/.config/solana/id.json
  echo "[2/5] Generated new keypair"
else
  echo "[2/5] Using existing keypair: $(solana-keygen pubkey)"
fi

# 3. Airdrop SOL
echo "[3/5] Requesting airdrop..."
solana airdrop 5 || echo "Airdrop may have failed — check balance with: solana balance"

# 4. Build the program
echo "[4/5] Building program..."
anchor build

# 5. Deploy to devnet
echo "[5/5] Deploying to devnet..."
anchor deploy --provider.cluster devnet

echo ""
echo "=== Setup complete ==="
echo "Program ID: $(solana-keygen pubkey target/deploy/shadow_book-keypair.json)"
echo "Wallet:     $(solana-keygen pubkey)"
echo "Balance:    $(solana balance)"
