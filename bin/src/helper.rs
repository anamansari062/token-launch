use solana_sdk::signature::{read_keypair_file, Keypair};
use std::env;
use std::path::PathBuf;

/// Loads the payer keypair from ~/.config/solana/id.json
pub fn get_payer_keypair() -> Result<Keypair, Box<dyn std::error::Error>> {
    let home_dir = env::var("HOME")?;
    let mut path = PathBuf::from(home_dir);
    path.push(".config/solana/id.json");
    let keypair = read_keypair_file(path)
        .map_err(|e| format!("Failed to read keypair file: {}", e))?;
    Ok(keypair)
}