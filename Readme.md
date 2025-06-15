# Solana Launchpad Program

A native Solana program and CLI to launch SPL (legacy), SPL 2022 tokens, and standard NFT assets with metadata support.

## Prerequisites

- [Rust 1.79+](https://www.rust-lang.org/tools/install)
- [Solana CLI 1.18+](https://solana.com/docs/intro/installation)

## Quickstart

**Clone & Build**
```bash
git clone <repository-url>
cd token-launch
cd program && cargo build-sbf
cd ../bin && cargo build
```

**Deploy to Devnet**
```bash
cd program
cargo build-sbf
solana program deploy target-program/deploy/token-launch.so
```

**Run Tests**
```bash
cargo test
```

## CLI Usage

All commands are run from the `bin` directory:

### Launch Tokens & NFTs

- **Launch Assets using default metadata values**
  ``` bash
    cd bin
    cargo run --bin cli launch --type spl-legacy
    cargo run --bin cli launch --type spl-2022
    cargo run --bin cli launch --type nft
  ```

- **SPL Legacy Token**
  ```bash
  cargo run --bin cli launch --type spl-legacy --name "My Token" --symbol "MYTOKEN" --decimals 6 --supply 1000000 --uri "https://example.com/metadata.json" --program-id <PROGRAM_ID> --rpc-url https://api.devnet.solana.com
  ```

- **SPL Token 2022**
  ```bash
  cargo run --bin cli launch --type spl-2022 --name "Advanced Token" --symbol "ADV" --decimals 9 --supply 1000000000 --uri "https://example.com/advanced-metadata.json" --program-id <PROGRAM_ID> --rpc-url https://api.devnet.solana.com
  ```

- **NFT**
  ```bash
  cargo run --bin cli launch --type nft --name "My NFT" --symbol "MYNFT" --uri "https://example.com/nft-metadata.json" --program-id <PROGRAM_ID> --rpc-url https://api.devnet.solana.com
  ```

> Note: To pass custom `program-id` and `rpc-url`, make sure to deploy the program accordingly. The CLI defaults to an already deployed devnet program. Also, It requires a funded wallet to be present at `~/.config/solana/id.json`

### Other Commands

- **Get PDA**
  ```bash
  cargo run --bin cli get-pda --mint <MINT_PUBKEY> --program-id <PROGRAM_ID>
  ```

- **Validate Config**
  ```bash
  cargo run --bin cli validate --name "My Token" --symbol "MYTOKEN" --decimals 6 --uri "https://example.com/metadata.json"
  ```

### Flags
Subcommands for `cargo run --bin cli launch`

- `--type` Asset type: `spl-legacy`, `spl-2022`, `nft`
- `--name` Name of token/NFT
- `--symbol` Symbol
- `--decimals` Number of decimals (0 for NFT)
- `--supply` Total supply (1 for NFT)
- `--uri` Metadata URI
- `--program-id` Token Launch program ID
- `--rpc-url` Solana RPC endpoint

Run `cargo run --bin cli launch --help` for all options.

## Testing

Rust Unit and Integration Tests
```bash
cargo test
# Or run specific integration tests:
cargo test bin/test/test_happy
cargo test bin/test/test_unhappy
cargo test bin/test/test_configs
```

Typescript Tests
``` bash
npm install && npm run test
```
