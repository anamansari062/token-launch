use solana_program::{
    account_info::{ next_account_info, AccountInfo }, entrypoint::ProgramResult,  msg, program::invoke_signed, program_error::ProgramError, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar
};
use crate::{
    constants::LAUNCHED_ASSET_SEED, cpi::{initialize_mint, initialize_token_account, mint_to}, state::{ Asset, AssetType, LaunchConfig }, util::validate_launch_config
};

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
    let metadata_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let _associated_token_account = next_account_info(accounts_iter)?;
    let rent= &Rent::get()?;

    // Verify signer
    if !payer.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    match config.asset_type {
        AssetType::SplTokenLegacy => {
            launch_spl_token_legacy(
                payer,
                mint_account,
                token_account,
                system_program,
                token_program,
                rent,
                &config,
            )?;
        }
        AssetType::SplToken2022 => {
            // launch_spl_token_2022(
            //     // program_id,
            //     payer,
            //     mint_account,
            //     token_account,
            //     // metadata_account,
            //     &rent,
            //     system_program,
            //     token_program,
            //     &config,
            // )?;
        }
        AssetType::StandardNft => {
            // launch_standard_nft(
            //     // program_id,
            //     payer,
            //     mint_account,
            //     // metadata_account,
            //     &rent,
            //     system_program,
            //     token_program,
            //     &config,
            // )?;
        }
    }

    // Create program data account to track the launched asset
    create_launched_asset_account(
        program_id,
        payer,
        mint_account.key,
        metadata_account,
        system_program,
        &rent,
        &config
    )?;

    msg!("Successfully launched {} asset: {}", format!("{:?}", config.asset_type), config.name);

    Ok(())
}

/// Launch SPL Token (Legacy)
fn launch_spl_token_legacy<'a>(
    payer: &AccountInfo<'a>,
    mint_account: &AccountInfo<'a>,
    token_account: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    rent: &Rent,
    config: &LaunchConfig,
)  -> ProgramResult {
    msg!("Launching SPL Token (Legacy): {}", config.name);

    // Create and initialize mint account
    initialize_mint::process(
        payer,
        payer,
        mint_account,
        system_program,
        token_program,
        rent
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
fn _launch_spl_token_2022<'a>(
    _payer: &AccountInfo<'a>,
    _mint_account: &AccountInfo<'a>,
    _token_account: &AccountInfo<'a>,
    _rent: &Rent,
    _system_program: &AccountInfo<'a>,
    _token_program: &AccountInfo<'a>,
    config: &LaunchConfig,
) -> ProgramResult {
    msg!("Launching SPL Token 2022: {}", config.name);
    
    // For now, use same logic as legacy token
    // In a full implementation, you'd use spl-token-2022 specific features
    // launch_spl_token_legacy(
    //     payer,
    //     mint_account,
    //     token_account,
    //     rent,
    //     system_program,
    //     token_program,
    //     config,
    // )
    Ok(())
}

/// Launch Standard NFT
fn _launch_standard_nft<'a>(
    _payer: &AccountInfo<'a>,
    _mint_account: &AccountInfo<'a>,
    _rent: &Rent,
    _system_program: &AccountInfo<'a>,
    _token_program: &AccountInfo<'a>,
    config: &LaunchConfig,
) -> ProgramResult {
    msg!("Launching Standard NFT: {}", config.name);

    // Create mint account for NFT (0 decimals, supply of 1)
    // let mint_space = Mint::LEN;
    // let mint_lamports = rent.minimum_balance(mint_space);

    // invoke(
    //     &system_instruction::create_account(
    //         payer.key,
    //         mint_account.key,
    //         mint_lamports,
    //         mint_space as u64,
    //         token_program.key,
    //     ),
    //     &[payer.clone(), mint_account.clone(), system_program.clone()],
    // )?;

    // Initialize mint with 0 decimals for NFT
    // invoke(
    //     &token_instruction::initialize_mint(
    //         token_program.key,
    //         mint_account.key,
    //         payer.key,
    //         Some(payer.key),
    //         0, // NFTs have 0 decimals
    //     )?,
    //     &[mint_account.clone()],
    // )?;

    // Note: In a full implementation, I would create Metaplex metadata here
    // This requires additional accounts and the Metaplex Token Metadata program

    Ok(())
}

/// Helper function to create metadata account for asset
fn create_launched_asset_account<'a>(
    program_id: &Pubkey,
    payer: &AccountInfo<'a>,
    mint: &Pubkey,
    metadata_account: &AccountInfo<'a>,
    _system_program: &AccountInfo<'a>,
    rent: &Rent,
    config: &LaunchConfig,
) -> ProgramResult {
    let (launched_asset_pda, bump_seed) = Pubkey::find_program_address(
        &[LAUNCHED_ASSET_SEED, mint.as_ref()],
        program_id,
    );

    // Ensure the PDA is not already initialized
    if !metadata_account.key.eq(&launched_asset_pda) {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    msg!("Creating launched asset account: {}", launched_asset_pda);

    let launched_asset = Asset {
        asset_type: config.asset_type.clone(),
        mint: *mint,
        creator: config.creator,
        name: config.name.clone(),
        symbol: config.symbol.clone(),
        total_supply: config.total_supply,
        launch_timestamp: 0, // Use Clock sysvar in full impl
    };

    let serialized_data = launched_asset.try_to_vec()?;
    let data_len = serialized_data.len();
    let lamports = rent.minimum_balance(data_len);

    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            &launched_asset_pda,
            lamports,
            data_len as u64,
            program_id,
        ),
        &[
            payer.clone(),
            metadata_account.clone(),
        ],
        &[&[LAUNCHED_ASSET_SEED, mint.as_ref(), &[bump_seed]]],
    )?;

    Ok(())
}

/// Helper function to create token account and mint initial supply
fn create_and_mint_to_token_account<'a>(
    payer: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    token_account: &AccountInfo<'a>,
    rent: &Rent,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {

    initialize_token_account::process(
        payer,
        token_account,
        mint,
        system_program,
        token_program,
        rent
    )?;

    msg!("Token account initialized: {}", token_account.key);

    mint_to::process(
        token_program,
        mint,
        token_account,
        payer,
        amount,
        &[],
    )?;

    msg!("Minted {}", amount);

    Ok(())
}

