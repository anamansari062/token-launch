use solana_program::{
    account_info::{ next_account_info, AccountInfo }, entrypoint::ProgramResult, msg, program::{invoke, invoke_signed}, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar
};
use spl_token::{
    instruction as token_instruction,
    state::{Account as TokenAccount, Mint},
};
use crate::{
    constants::LAUNCHED_ASSET_SEED,
    state::{ Asset, AssetType, LaunchConfig },
};

use crate::constants::*;

use borsh::BorshSerialize;

pub fn launch_asset(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    config: LaunchConfig
) -> ProgramResult {
    // Validate config
    validate_launch_config(&config)?;

    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let mint_account = next_account_info(accounts_iter)?;
    let token_account = next_account_info(accounts_iter)?;
    let _metadata_account = next_account_info(accounts_iter)?;
    let rent_sysvar = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    // Verify signer
    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let rent = Rent::from_account_info(rent_sysvar)?;

    match config.asset_type {
        AssetType::SplTokenLegacy => {
            launch_spl_token_legacy(
                // program_id,
                payer,
                mint_account,
                token_account,
                // metadata_account,
                &rent,
                system_program,
                token_program,
                &config,
            )?;
        }
        AssetType::SplToken2022 => {
            launch_spl_token_2022(
                // program_id,
                payer,
                mint_account,
                token_account,
                // metadata_account,
                &rent,
                system_program,
                token_program,
                &config,
            )?;
        }
        AssetType::StandardNft => {
            launch_standard_nft(
                // program_id,
                payer,
                mint_account,
                // metadata_account,
                &rent,
                system_program,
                token_program,
                &config,
            )?;
        }
    }

    // Create program data account to track the launched asset
    create_launched_asset_account(
        program_id,
        payer,
        mint_account.key,
        system_program,
        &rent,
        &config
    )?;

    msg!("Successfully launched {} asset: {}", format!("{:?}", config.asset_type), config.name);

    Ok(())
}

fn create_launched_asset_account(
    program_id: &Pubkey,
    payer: &AccountInfo,
    mint: &Pubkey,
    _system_program: &AccountInfo,
    rent: &Rent,
    config: &LaunchConfig
) -> ProgramResult {
    let (launched_asset_pda, bump_seed) = Pubkey::find_program_address(
        &[LAUNCHED_ASSET_SEED, mint.as_ref()],
        program_id
    );

    let launched_asset = Asset {
        asset_type: config.asset_type.clone(),
        mint: *mint,
        creator: config.creator,
        name: config.name.clone(),
        symbol: config.symbol.clone(),
        total_supply: config.total_supply,
        launch_timestamp: 0, // Would use Clock sysvar in real implementation
    };

    let serialized_data = launched_asset.try_to_vec()?;
    let data_len = serialized_data.len();
    let lamports = rent.minimum_balance(data_len);

    // Create the PDA account
    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            &launched_asset_pda,
            lamports,
            data_len as u64,
            program_id
        ),
        &[payer.clone()],
        &[&[LAUNCHED_ASSET_SEED, mint.as_ref(), &[bump_seed]]]
    )?;

    Ok(())
}

/// Launch SPL Token (Legacy)
fn launch_spl_token_legacy<'a>(
    payer: &AccountInfo<'a>,
    mint_account: &AccountInfo<'a>,
    token_account: &AccountInfo<'a>,
    rent: &Rent,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    config: &LaunchConfig,
)  -> ProgramResult {
    msg!("Launching SPL Token (Legacy): {}", config.name);

    // Create mint account
    // let mint_space = Mint::LEN;
    let mint_space = 32; // Add space for the mint authority
    let mint_lamports = rent.minimum_balance(mint_space);

    invoke(
        &system_instruction::create_account(
            payer.key,
            mint_account.key,
            mint_lamports,
            mint_space as u64,
            token_program.key,
        ),
        &[payer.clone(), mint_account.clone(), system_program.clone()],
    )?;

    // Initialize mint
    invoke(
        &token_instruction::initialize_mint(
            token_program.key,
            mint_account.key,
            payer.key,
            Some(payer.key),
            config.decimals,
        )?,
        &[mint_account.clone()],
    )?;

    // Create and mint to token account if supply > 0
    if config.total_supply > 0 {
        create_and_mint_to_token_account(
            payer,
            mint_account,
            token_account,
            rent,
            system_program,
            token_program,
            config.total_supply,
        )?;
    }

    Ok(())
}

/// Launch SPL Token 2022
fn launch_spl_token_2022<'a>(
    payer: &AccountInfo<'a>,
    mint_account: &AccountInfo<'a>,
    token_account: &AccountInfo<'a>,
    rent: &Rent,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    config: &LaunchConfig,
) -> ProgramResult {
    msg!("Launching SPL Token 2022: {}", config.name);
    
    // For now, use same logic as legacy token
    // In a full implementation, you'd use spl-token-2022 specific features
    launch_spl_token_legacy(
        payer,
        mint_account,
        token_account,
        rent,
        system_program,
        token_program,
        config,
    )
}

/// Launch Standard NFT
fn launch_standard_nft<'a>(
    payer: &AccountInfo<'a>,
    mint_account: &AccountInfo<'a>,
    rent: &Rent,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    config: &LaunchConfig,
) -> ProgramResult {
    msg!("Launching Standard NFT: {}", config.name);

    // Create mint account for NFT (0 decimals, supply of 1)
    let mint_space = Mint::LEN;
    let mint_lamports = rent.minimum_balance(mint_space);

    invoke(
        &system_instruction::create_account(
            payer.key,
            mint_account.key,
            mint_lamports,
            mint_space as u64,
            token_program.key,
        ),
        &[payer.clone(), mint_account.clone(), system_program.clone()],
    )?;

    // Initialize mint with 0 decimals for NFT
    invoke(
        &token_instruction::initialize_mint(
            token_program.key,
            mint_account.key,
            payer.key,
            Some(payer.key),
            0, // NFTs have 0 decimals
        )?,
        &[mint_account.clone()],
    )?;

    // Note: In a full implementation, I would create Metaplex metadata here
    // This requires additional accounts and the Metaplex Token Metadata program

    Ok(())
}

/// Helper function to create token account and mint initial supply
fn create_and_mint_to_token_account<'a>(
    payer: &AccountInfo<'a>,
    mint_account: &AccountInfo<'a>,
    token_account: &AccountInfo<'a>,
    rent: &Rent,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    // Create token account
    let account_space = TokenAccount::LEN;
    let account_lamports = rent.minimum_balance(account_space);

    invoke(
        &system_instruction::create_account(
            payer.key,
            token_account.key,
            account_lamports,
            account_space as u64,
            token_program.key,
        ),
        &[payer.clone(), token_account.clone(), system_program.clone()],
    )?;

    // Initialize token account
    invoke(
        &token_instruction::initialize_account(
            token_program.key,
            token_account.key,
            mint_account.key,
            payer.key,
        )?,
        &[token_account.clone(), mint_account.clone(), payer.clone()],
    )?;

    // Mint tokens to the account
    invoke(
        &token_instruction::mint_to(
            token_program.key,
            mint_account.key,
            token_account.key,
            payer.key,
            &[],
            amount,
        )?,
        &[
            mint_account.clone(),
            token_account.clone(),
            payer.clone(),
        ],
    )?;

    Ok(())
}

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