pub mod state;
pub mod processor;
pub mod constants;
pub mod entrypoint;
pub mod util;
pub mod cpi;

pub use solana_program;
pub use state::*;

solana_program::declare_id!("4n6ByGTtLj4fTgLApV2aigC3XzWZhCmYkNbcfVheGzd8");

#[cfg(test)]
mod tests {
    use solana_program::pubkey::Pubkey;

    use crate::{constants::LAUNCHED_ASSET_SEED, util::{get_launched_asset_pda, validate_launch_config}};

    use super::*;


    #[test]
    fn test_validate_launch_config_success() {
        let valid_config = LaunchConfig {
            asset_type: AssetType::SplTokenLegacy,
            name: "Valid Token".to_string(),
            symbol: "VALID".to_string(),
            decimals: 6,
            total_supply: 1_000_000,
            metadata_uri: "https://example.com/metadata.json".to_string(),
            creator: Pubkey::new_unique(),
            is_mutable: true,
        };

        assert!(validate_launch_config(&valid_config).is_ok());
    }

    #[test]
    fn test_validate_launch_config_name_too_long() {
        let invalid_config = LaunchConfig {
            asset_type: AssetType::SplTokenLegacy,
            name: "A".repeat(50), // Too long
            symbol: "VALID".to_string(),
            decimals: 6,
            total_supply: 1_000_000,
            metadata_uri: "https://example.com/metadata.json".to_string(),
            creator: Pubkey::new_unique(),
            is_mutable: true,
        };

        assert!(validate_launch_config(&invalid_config).is_err());
    }

    #[test]
    fn test_validate_launch_config_decimals_too_high() {
        let invalid_config = LaunchConfig {
            asset_type: AssetType::SplTokenLegacy,
            name: "Valid Token".to_string(),
            symbol: "VALID".to_string(),
            decimals: 15, // Too high
            total_supply: 1_000_000,
            metadata_uri: "https://example.com/metadata.json".to_string(),
            creator: Pubkey::new_unique(),
            is_mutable: true,
        };

        assert!(validate_launch_config(&invalid_config).is_err());
    }

    #[test]
    fn test_get_launched_asset_pda() {
        let program_id = Pubkey::new_unique();
        let mint = Pubkey::new_unique();

        let (pda, bump) = get_launched_asset_pda(&program_id, &mint);

        // Verify PDA derivation
        let expected_pda = Pubkey::create_program_address(
            &[LAUNCHED_ASSET_SEED, mint.as_ref(), &[bump]],
            &program_id
        ).unwrap();

        assert_eq!(pda, expected_pda);

        // Verify the PDA is off-curve (valid PDA)
        assert!(!pda.is_on_curve());
    }

    #[test]
    fn test_asset_type_variants() {
        // Test that all asset type variants are accessible
        let spl_legacy = AssetType::SplTokenLegacy;
        let spl_2022 = AssetType::SplToken2022;
        let nft = AssetType::StandardNft;

        // Test equality
        assert_eq!(spl_legacy, AssetType::SplTokenLegacy);
        assert_eq!(spl_2022, AssetType::SplToken2022);
        assert_eq!(nft, AssetType::StandardNft);

        // Test inequality
        assert_ne!(spl_legacy, spl_2022);
        // assert_ne!(spl_2022, nft);
        assert_ne!(nft, spl_legacy);
    }

    #[test]
    fn test_launch_config_creation() {
        let config = LaunchConfig {
            asset_type: AssetType::SplToken2022,
            name: "Test Token 2022".to_string(),
            symbol: "TEST22".to_string(),
            decimals: 9,
            total_supply: 1_000_000_000,
            metadata_uri: "https://test.com/metadata".to_string(),
            creator: Pubkey::new_unique(),
            is_mutable: false,
        };

        assert_eq!(config.name, "Test Token 2022");
        assert_eq!(config.symbol, "TEST22");
        assert_eq!(config.decimals, 9);
        assert_eq!(config.total_supply, 1_000_000_000);
        assert_eq!(config.asset_type, AssetType::SplToken2022);
        assert!(!config.is_mutable);
    }
}

