use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

// Instructions supported by the program
#[derive(BorshDeserialize, BorshSerialize)]
pub enum LaunchpadInstruction {
    /// Launch a new asset
    /// Accounts:
    /// 0. /[signer] Payer account
    /// 1. [writable] Mint account to be created
    /// 2. [writable] Token account for initial supply (if applicable)
    /// 3. [writable] Metadata account (if applicable)
    /// 4. [] System program
    /// 5. [] Token program (legacy or 2022)
    /// 6. [] Associated token program (if applicable)
    LaunchAsset {
        config: LaunchConfig,
    }
}

// Asset types that can be launched
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub enum AssetType {
    SplTokenLegacy,
    SplToken2022,
    StandardNft,
}

// Configuration
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct LaunchConfig {
    pub asset_type: AssetType,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
    pub metadata_uri: String,
    pub creator: Pubkey,
    pub is_mutable: bool
}

// Program state to track assets
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Asset {
    pub asset_type: AssetType,
    pub mint: Pubkey,
    pub creator: Pubkey,
    pub name: String,
    pub symbol: String,
    pub total_supply: u64,
    pub launch_timestamp: i64,
}
