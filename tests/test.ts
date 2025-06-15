import {
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import * as borsh from "borsh";
import { loadKeypair, sendLaunchTransaction } from "./setup/utils";
import { LaunchConfig, LaunchConfigSchema, AssetType } from "./setup/data";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";

// Test suite
describe("launch_it", () => {
  describe("Native happy tests", () => {
    
    const connection = new Connection("https://api.devnet.solana.com", "confirmed");
    const payer = loadKeypair(); // Use CLI wallet
    const programId = new PublicKey("4n6ByGTtLj4fTgLApV2aigC3XzWZhCmYkNbcfVheGzd8");
    
    it("Launch SPL Token (Legacy)", async function () {
      this.timeout(15000);
      
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
        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // token program
        { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // associated token program
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false }, // rent sysvar
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

    it("Launch SPL Token (2022)", async function () {
      this.timeout(15000);

      // Prepare LaunchConfig
      const config = new LaunchConfig({
        asset_type: AssetType.SplToken2022,
        name: "TestToken2022",
        symbol: "TTK2",
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

      console.log("SPL Token 2022");
      console.log("Mint Account:", mintAccount.publicKey.toBase58());
      console.log("Token Account:", tokenAccount.publicKey.toBase58());
      console.log("Metadata Account:", metadataAccount.toBase58());

      const keys = [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // payer
        { pubkey: mintAccount.publicKey, isSigner: true, isWritable: true }, // mint
        { pubkey: tokenAccount.publicKey, isSigner: true, isWritable: true }, // token
        { pubkey: metadataAccount, isSigner: false, isWritable: true }, // metadata
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // system program
        { pubkey: TOKEN_2022_PROGRAM_ID, isSigner: false, isWritable: false }, // token program
        { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // associated token program
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false }, // rent sysvar
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

    it("Launch Standard NFT", async function () {
      this.timeout(15000);

      // Prepare LaunchConfig
      const config = new LaunchConfig({
        asset_type: AssetType.StandardNft,
        name: "TestNFT",
        symbol: "TN",
        decimals: 0,
        total_supply: BigInt(1),
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

      console.log("Standard NFT");
      console.log("NFT Mint Account:", mintAccount.publicKey.toBase58());
      console.log("Token Account:", tokenAccount.publicKey.toBase58());
      console.log("Metadata Account:", metadataAccount.toBase58());

      const keys = [
        { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // payer
        { pubkey: mintAccount.publicKey, isSigner: true, isWritable: true }, // mint
        { pubkey: tokenAccount.publicKey, isSigner: true, isWritable: true }, // token
        { pubkey: metadataAccount, isSigner: false, isWritable: true }, // metadata
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false }, // system program
        { pubkey: TOKEN_2022_PROGRAM_ID, isSigner: false, isWritable: false }, // token program
        { pubkey: ASSOCIATED_TOKEN_PROGRAM_ID, isSigner: false, isWritable: false }, // associated token program
        { pubkey: SYSVAR_RENT_PUBKEY, isSigner: false, isWritable: false }, // rent sysvar
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
