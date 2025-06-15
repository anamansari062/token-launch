import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import * as borsh from "borsh";
import { loadKeypair, sendLaunchTransaction } from "./setup/utils";
import { LaunchConfig, LaunchConfigSchema, AssetType } from "./setup/data";

// Test suite
describe("launch_it", () => {
  describe("Native happy tests", () => {
    it("Launch SPL Token (Legacy)", async function () {
      this.timeout(15000); // ✅ Now `this` is available
      
      const connection = new Connection("https://api.devnet.solana.com", "confirmed");
      const payer = loadKeypair(); // Use CLI wallet
      const programId = new PublicKey("4n6ByGTtLj4fTgLApV2aigC3XzWZhCmYkNbcfVheGzd8");

      // Prepare LaunchConfig
      const config = new LaunchConfig({
        asset_type: AssetType.SplTokenLegacy,
        name: "TestToken",
        symbol: "TTK",
        decimals: 6,
        total_supply: BigInt(1_000_000_000),
        metadata_uri: "https://example.com/metadata.json",
        creator: payer.publicKey.toBytes(),
        is_mutable: 1,
      });

      // Serialize instruction data: [0, ...borsh(LaunchConfig)]
      const variant = Buffer.from([0]); // 0 = LaunchAsset
      const configBuf = Buffer.from(borsh.serialize(LaunchConfigSchema, config));
      const instructionData = Buffer.concat([variant, configBuf]);

      // Accounts required by the instruction (order must match Rust)
      const mintAccount = Keypair.generate();
      const tokenAccount = Keypair.generate();
      const metadataAccount = PublicKey.findProgramAddressSync(
        [
          Buffer.from("launched_asset"),
          Buffer.from(mintAccount.publicKey.toBuffer()),
        ],
        programId
      )[0];

      console.log("SPL Token (Legacy)");
      console.log("Mint Account:", mintAccount.publicKey.toBase58());
      console.log("Token Account:", tokenAccount.publicKey.toBase58());
      console.log("Metadata Account:", metadataAccount.toBase58());

      const keys = [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // payer
        { pubkey: mintAccount.publicKey, isSigner: true, isWritable: true }, // mint
        { pubkey: tokenAccount.publicKey, isSigner: true, isWritable: true }, // token
        { pubkey: metadataAccount, isSigner: false, isWritable: true }, // metadata
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // system program
        { pubkey: new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"), isSigner: false, isWritable: false }, // token program
        { pubkey: new PublicKey("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"), isSigner: false, isWritable: false }, // associated token program
        { pubkey: new PublicKey("SysvarRent111111111111111111111111111111111"), isSigner: false, isWritable: false }, // rent sysvar
      ];

      // Create instruction
      const ix = new TransactionInstruction({
        keys,
        programId,
        data: instructionData,
      });

      await sendLaunchTransaction(
        connection,
        [payer, mintAccount, tokenAccount],
        new Transaction().add(ix)
      );

    });
  });
});
