use solana_program::{entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey};

use crate::{constants::{LAUNCHED_ASSET_SEED, MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH}, LaunchConfig};

/// Validate launch configuration
pub fn validate_launch_config(config: &LaunchConfig) -> ProgramResult {
    if config.name.len() > MAX_NAME_LENGTH {
        msg!("Name too long: {} > {}", config.name.len(), MAX_NAME_LENGTH);
        return Err(ProgramError::InvalidArgument);
    }

    if config.symbol.len() > MAX_SYMBOL_LENGTH {
        msg!(
            "Symbol too long: {} > {}",
            config.symbol.len(),
            MAX_SYMBOL_LENGTH
        );
        return Err(ProgramError::InvalidArgument);
    }

    if config.metadata_uri.len() > MAX_URI_LENGTH {
        msg!(
            "URI too long: {} > {}",
            config.metadata_uri.len(),
            MAX_URI_LENGTH
        );
        return Err(ProgramError::InvalidArgument);
    }

    if config.decimals > 9 {
        msg!("Decimals too high: {} > 9", config.decimals);
        return Err(ProgramError::InvalidArgument);
    }

    Ok(())
}

/// Helper function to get launched asset PDA
pub fn get_launched_asset_pda(program_id: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[LAUNCHED_ASSET_SEED, mint.as_ref()], program_id)
}

#[cfg(test)]
mod validation_tests {
    use super::*;
    use crate::{LaunchConfig, state::AssetType};
    use solana_program::{program_error::ProgramError, pubkey::Pubkey};

    // Helper function to create a valid config for testing
    fn create_valid_config() -> LaunchConfig {
        LaunchConfig {
            asset_type: AssetType::SplTokenLegacy,
            name: "Test Token".to_string(),
            symbol: "TEST".to_string(),
            decimals: 6,
            total_supply: 1_000_000,
            metadata_uri: "https://example.com/metadata.json".to_string(),
            creator: Pubkey::new_unique(),
            is_mutable: false,
        }
    }

    #[test]
    fn test_validate_launch_config_success() {
        let config = create_valid_config();
        let result = validate_launch_config(&config);
        assert!(result.is_ok(), "Valid config should pass validation");
    }

    #[test]
    fn test_validate_launch_config_name_too_long() {
        let mut config = create_valid_config();
        config.name = "A".repeat(MAX_NAME_LENGTH + 1); // Exceed max length
        
        let result = validate_launch_config(&config);
        assert_eq!(result, Err(ProgramError::InvalidArgument));
    }

    #[test]
    fn test_validate_launch_config_symbol_too_long() {
        let mut config = create_valid_config();
        config.symbol = "B".repeat(MAX_SYMBOL_LENGTH + 1); // Exceed max length
        
        let result = validate_launch_config(&config);
        assert_eq!(result, Err(ProgramError::InvalidArgument));
    }

    #[test]
    fn test_validate_launch_config_uri_too_long() {
        let mut config = create_valid_config();
        config.metadata_uri = "https://example.com/".to_string() + &"x".repeat(MAX_URI_LENGTH);
        
        let result = validate_launch_config(&config);
        assert_eq!(result, Err(ProgramError::InvalidArgument));
    }

    #[test]
    fn test_validate_launch_config_decimals_too_high() {
        let mut config = create_valid_config();
        config.decimals = 10; // Should be <= 9
        
        let result = validate_launch_config(&config);
        assert_eq!(result, Err(ProgramError::InvalidArgument));
    }

    #[test]
    fn test_get_launched_asset_pda_deterministic() {
        let program_id = Pubkey::new_unique();
        let mint = Pubkey::new_unique();
        
        // Call the function twice with same inputs
        let (pda1, bump1) = get_launched_asset_pda(&program_id, &mint);
        let (pda2, bump2) = get_launched_asset_pda(&program_id, &mint);
        
        // Results should be identical
        assert_eq!(pda1, pda2, "PDA should be deterministic");
        assert_eq!(bump1, bump2, "Bump should be deterministic");
    }

    #[test]
    fn test_get_launched_asset_pda_different_mints() {
        let program_id = Pubkey::new_unique();
        let mint1 = Pubkey::new_unique();
        let mint2 = Pubkey::new_unique();
        
        let (pda1, _) = get_launched_asset_pda(&program_id, &mint1);
        let (pda2, _) = get_launched_asset_pda(&program_id, &mint2);
        
        // Different mints should produce different PDAs
        assert_ne!(pda1, pda2, "Different mints should produce different PDAs");
    }
}