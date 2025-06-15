use clap::{Arg, Command};
use cli::{handle_get_pda, handle_launch, handle_validate};

mod instruction;
mod helper;
mod cli;

#[tokio::main]
async fn main() {
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
        .subcommand(
            Command::new("launch")
                .about("Mint a new SPL token (legacy), token 22 or NFT")
                .arg(
                    Arg::new("type")
                        .short('t')
                        .long("type")
                        .value_name("ASSET_TYPE")
                        .help("Asset type: spl-legacy, spl-2022, or nft")
                        .required(true),
                )
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .value_name("NAME")
                        .help("Token or NFT name")
                        .default_value("MyCliToken")
                        .required(false),
                )
                .arg(
                    Arg::new("symbol")
                        .short('s')
                        .long("symbol")
                        .value_name("SYMBOL")
                        .help("Token or NFT symbol")
                        .default_value("CLI")
                        .required(false),
                )
                .arg(
                    Arg::new("decimals")
                        .short('d')
                        .long("decimals")
                        .value_name("DECIMALS")
                        .help("Number of decimals (0 for NFT)")
                        .default_value("6")
                        .required(false),
                )
                .arg(
                    Arg::new("supply")
                        .short('S')
                        .long("supply")
                        .value_name("SUPPLY")
                        .help("Total supply (1 for NFT)")
                        .default_value("1000000")
                        .required(false),
                )
                .arg(
                    Arg::new("uri")
                        .short('u')
                        .long("uri")
                        .value_name("URI")
                        .help("Metadata URI")
                        .required(false)
                        .default_value("https://example.com/metadata.json"),
                )
                .arg(
                    Arg::new("program-id")
                        .short('p')
                        .long("program-id")
                        .value_name("PROGRAM_ID")
                        .help("Launchpad program ID")
                        .default_value("4n6ByGTtLj4fTgLApV2aigC3XzWZhCmYkNbcfVheGzd8")
                        .required(false),
                )
                .arg(
                    Arg::new("rpc-url")
                        .long("rpc-url")
                        .value_name("RPC_URL")
                        .help("Solana RPC endpoint")
                        .default_value("https://api.devnet.solana.com")
                        .required(false),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("launch", sub_matches)) => {
            if let Err(e) = handle_launch(sub_matches).await {
                eprintln!("Error launching asset: {}", e);
                std::process::exit(1);    
            }
        }
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


#[cfg(test)]
mod tests {
    use token_launch::AssetType;

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
