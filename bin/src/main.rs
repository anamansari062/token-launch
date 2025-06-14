use clap::{Arg, Command};
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

// Import from our library
use token_launch::{AssetType, LaunchConfig};

fn main() {
    let matches = Command::new("Token Launch CLI")
        .version("1.0")
        .about("CLI tool for launching tokens and NFTs on Solana")
        .subcommand(
            Command::new("get-pda")
                .about("Get the PDA for a launched asset")
                .arg(
                    Arg::new("mint")
                        .short('m')
                        .long("mint")
                        .value_name("MINT_PUBKEY")
                        .help("Mint public key")
                        .required(true),
                )
                .arg(
                    Arg::new("program-id")
                        .short('p')
                        .long("program-id")
                        .value_name("PROGRAM_ID")
                        .help("Launchpad program ID")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("validate")
                .about("Validate a launch configuration")
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .value_name("NAME")
                        .help("Token name")
                        .required(true),
                )
                .arg(
                    Arg::new("symbol")
                        .short('s')
                        .long("symbol")
                        .value_name("SYMBOL")
                        .help("Token symbol")
                        .required(true),
                )
                .arg(
                    Arg::new("decimals")
                        .short('d')
                        .long("decimals")
                        .value_name("DECIMALS")
                        .help("Number of decimals")
                        .default_value("6"),
                )
                .arg(
                    Arg::new("uri")
                        .short('u')
                        .long("uri")
                        .value_name("URI")
                        .help("Metadata URI")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("get-pda", sub_matches)) => {
            if let Err(e) = handle_get_pda(sub_matches) {
                eprintln!("Error getting PDA: {}", e);
                std::process::exit(1);
            }
        }
        Some(("validate", sub_matches)) => {
            if let Err(e) = handle_validate(sub_matches) {
                eprintln!("Error validating config: {}", e);
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("No valid subcommand provided. Use --help for usage information.");
            std::process::exit(1);
        }
    }
}

fn handle_validate(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let name = matches.get_one::<String>("name").unwrap();
    let symbol = matches.get_one::<String>("symbol").unwrap();
    let decimals: u8 = matches.get_one::<String>("decimals").unwrap().parse()?;
    let uri = matches.get_one::<String>("uri").unwrap();

    let config = LaunchConfig {
        asset_type: AssetType::SplTokenLegacy,
        name: name.clone(),
        symbol: symbol.clone(),
        decimals,
        total_supply: 1_000_000,
        metadata_uri: uri.clone(),
        creator: Pubkey::new_unique(),
        is_mutable: true,
    };

    match token_launch::validate_launch_config(&config) {
        Ok(()) => {
            println!("✅ Configuration is valid!");
            println!("Name: {}", name);
            println!("Symbol: {}", symbol);
            println!("Decimals: {}", decimals);
            println!("URI: {}", uri);
        }
        Err(e) => {
            println!("❌ Configuration is invalid: {:?}", e);
            return Err(e.into());
        }
    }

    Ok(())
}

fn handle_get_pda(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let mint_str = matches.get_one::<String>("mint").unwrap();
    let program_id_str = matches.get_one::<String>("program-id").unwrap();

    let mint = Pubkey::from_str(mint_str)?;
    let program_id = Pubkey::from_str(program_id_str)?;

    let (pda, bump) = token_launch::get_launched_asset_pda(&program_id, &mint);

    println!("Mint: {}", mint);
    println!("Program ID: {}", program_id);
    println!("PDA: {}", pda);
    println!("Bump: {}", bump);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_type_parsing() {
        assert!(matches!(
            parse_asset_type("spl-legacy"),
            Ok(AssetType::SplTokenLegacy)
        ));
        assert!(matches!(
            parse_asset_type("spl-2022"),
            Ok(AssetType::SplToken2022)
        ));
        assert!(matches!(
            parse_asset_type("nft"),
            Ok(AssetType::StandardNft)
        ));
        assert!(parse_asset_type("invalid").is_err());
    }

    fn parse_asset_type(s: &str) -> Result<AssetType, &'static str> {
        match s {
            "spl-legacy" => Ok(AssetType::SplTokenLegacy),
            "spl-2022" => Ok(AssetType::SplToken2022),
            "nft" => Ok(AssetType::StandardNft),
            _ => Err("Invalid asset type"),
        }
    }
}
