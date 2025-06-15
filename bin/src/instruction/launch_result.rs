use solana_sdk::pubkey::Pubkey;

pub struct LaunchResult {
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub metadata_account: Pubkey,
    pub signature: String,
}