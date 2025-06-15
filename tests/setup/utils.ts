import { Connection, Keypair, sendAndConfirmTransaction, Transaction, TransactionInstruction } from "@solana/web3.js";
import assert from "assert";
import fs from "fs";
import os from "os";
import path from "path";

// Load payer from local Solana CLI keypair
export function loadKeypair(): Keypair {
  const home = os.homedir();
  const keypairPath = path.join(home, ".config", "solana", "id.json");
  const secret = JSON.parse(fs.readFileSync(keypairPath, "utf-8"));
  return Keypair.fromSecretKey(Uint8Array.from(secret));
}

export async function sendLaunchTransaction(
  connection: Connection,
  signers: Keypair[],
  tx: Transaction,
) {
  // Send transaction
  try {
    tx.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;
    const sig = await sendAndConfirmTransaction(
        connection,
        tx,
        signers
    );
    assert.ok(sig);
      console.log("Transaction signature:", sig);
  }
  catch (e: any) {
      console.error("Transaction failed:", e);
      if (e instanceof Error && "logs" in e) {
        console.error("Program logs:", (e as any).logs);
      }
      throw e;
    }
}
