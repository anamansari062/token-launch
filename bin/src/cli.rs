use crate::helper::get_payer_keypair;
use crate::instruction::{launch_spl_token_2022, launch_spl_token_legacy, launch_standard_nft};
use solana_client::nonblocking::rpc_client;
use solana_program::pubkey::Pubkey;
use solana_sdk::signer::Signer;

use std::str::FromStr;

// Import from our library
use token_launch::{AssetType, LaunchConfig};
use token_launch::util::{validate_launch_config, get_launched_asset_pda};


pub fn handle_validate(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
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

    match validate_launch_config(&config) {
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

pub fn handle_get_pda(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let mint_str = matches.get_one::<String>("mint").unwrap();
    let program_id_str = matches.get_one::<String>("program-id").unwrap();

    let mint = Pubkey::from_str(mint_str)?;
    let program_id = Pubkey::from_str(program_id_str)?;

    let (pda, bump) = get_launched_asset_pda(&program_id, &mint);

    println!("Mint: {}", mint);
    println!("Program ID: {}", program_id);
    println!("PDA: {}", pda);
    println!("Bump: {}", bump);

    Ok(())
}

pub async fn handle_launch(matches: &clap::ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let asset_type_str = matches.get_one::<String>("type").unwrap();
    let name = matches.get_one::<String>("name").unwrap();
    let symbol = matches.get_one::<String>("symbol").unwrap();
    let uri = matches.get_one::<String>("uri").unwrap();

    let rpc_client_string = matches.get_one::<String>("rpc-url").unwrap();
    let rpc_client = rpc_client::RpcClient::new(rpc_client_string.clone());

    let payer = get_payer_keypair()?;

    let program_id_string = matches.get_one::<String>("program-id").unwrap();
    let program_id = Pubkey::from_str(program_id_string)?;

    let config = match asset_type_str.as_str() {
        "spl-legacy" => {
            LaunchConfig {
                asset_type: AssetType::SplTokenLegacy,
                name: name.clone(),
                symbol: symbol.clone(),
                decimals: matches.get_one::<String>("decimals").unwrap().parse()?,
                total_supply: matches.get_one::<String>("supply").unwrap().parse()?,
                metadata_uri: uri.clone(),
                creator: Pubkey::new_unique(),
                is_mutable: true,
            }   
        },
        "spl-2022" => {            
            LaunchConfig {
                asset_type: AssetType::SplToken2022,
                name: name.clone(),
                symbol: symbol.clone(),
                decimals: matches.get_one::<String>("decimals").unwrap().parse()?,
                total_supply: matches.get_one::<String>("supply").unwrap().parse()?,
                metadata_uri: uri.clone(),
                creator: Pubkey::new_unique(),
                is_mutable: true,
            }
        },
        "nft" => {
            LaunchConfig {
                asset_type: AssetType::StandardNft,
                name: name.clone(),
                symbol: symbol.clone(),
                decimals: 0,
                total_supply: 1,
                metadata_uri: uri.clone(),
                creator: payer.pubkey(),
                is_mutable: true,
            }
        },
        _ => return Err("Invalid asset type".into()),
    };

    let result = match asset_type_str.as_str() {
        "spl-legacy" => launch_spl_token_legacy::launch_spl_token_legacy(program_id,rpc_client, &payer, config).await,
        "spl-2022" => launch_spl_token_2022::launch_spl_token_2022(program_id, rpc_client, &payer, config).await,
        "nft" => launch_standard_nft::launch_standard_nft(program_id,rpc_client, &payer, config).await,
        _ => return Err("Invalid asset type".into()),
    };

    match result {
        Ok(launch_result) => {
            println!("Mint: {}", launch_result.mint);
            println!("Token Account: {}", launch_result.token_account);
            println!("Metadata Account: {}", launch_result.metadata_account);
            println!("Transaction Signature: {}", launch_result.signature);
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    Ok(())
}