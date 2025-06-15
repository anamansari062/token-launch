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