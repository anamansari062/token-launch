# Solana Launchpad Program

A Native Solana program that enables users to launch spl (legacy), spl 2022 token and standard nft asset types on Solana with metadata support.

## Getting Started

### Prerequisites

- Rust 1.79+
- Solana CLI 1.18+

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd token-launch
   ```

2. Build the program:
   ```bash
   cargo build-sbf
   ```

3. Run tests:
   ```bash
   cargo test
   ```

4. Deploy to devnet:
   ```bash
   solana program deploy target/deploy/token-launch.so
   ```

### CLI Usage

The project includes a comprehensive CLI tool for interacting with the program.

#### 1. Launch an SPL Token (Legacy)

```bash
cargo run --bin cli launch \
  --type spl-legacy \
  --name "My Token" \
  --symbol "MYTOKEN" \
  --decimals 6 \
  --supply 1000000 \
  --uri "https://example.com/metadata.json" \
  --program-id <PROGRAM_ID> \
  --rpc-url https://api.devnet.solana.com
```

#### 2. Launch an SPL Token 2022

```bash
cargo run --bin cli launch \
  --type spl-2022 \
  --name "Advanced Token" \
  --symbol "ADV" \
  --decimals 9 \
  --supply 1000000000 \
  --uri "https://example.com/advanced-metadata.json" \
  --program-id <PROGRAM_ID> \
  --rpc-url https://api.devnet.solana.com
```

#### 3. Launch an NFT

```bash
cargo run --bin cli launch \
  --type nft \
  --name "My NFT" \
  --symbol "MYNFT" \
  --decimals 0 \
  --supply 1 \
  --uri "https://example.com/nft-metadata.json" \
  --program-id <PROGRAM_ID> \
  --rpc-url https://api.devnet.solana.com
```

#### 4. Get PDA for a Launched Asset

```bash
cargo run --bin cli get-pda \
  --mint <MINT_PUBKEY> \
  --program-id <PROGRAM_ID>
```

#### 5. Validate a Launch Configuration

```bash
cargo run --bin cli validate \
  --name "My Token" \
  --symbol "MYTOKEN" \
  --decimals 6 \
  --uri "https://example.com/metadata.json"
```

**Flags:**

- `--type`           Asset type: `spl-legacy`, `spl-2022`, or `nft`
- `--name`           Name of the token or NFT
- `--symbol`         Symbol for the asset
- `--decimals`       Number of decimals (0 for NFT)
- `--supply`         Total supply (1 for NFT)
- `--uri`            Metadata URI
- `--program-id`     Solana program ID
- `--rpc-url`        (Optional) Solana RPC endpoint
- `--mint`           Mint public key (for `get-pda`)

See `cargo run --bin cli --help` for all options.

## Testing

### Unit Tests
```bash
cargo test --lib
```

Run specific test modules:
```bash
cargo test test_validate_launch_config
cargo test test_get_launched_asset_pda
```