use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, instruction::{AccountMeta, Instruction}, msg, program::invoke, rent::Rent, system_instruction};
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize)]
pub struct InitializeMintInstructionData {
    pub discriminator: u8,        // 20
    pub decimals: u8,
    pub mint_authority: Pubkey,
    pub freeze_authority_option: u8, // 0 for None, 1 for Some
    pub freeze_authority: Pubkey,
}

/// CPI call to initializeMint
pub fn process<'a>(
    mint_authority: &AccountInfo<'a>,
    freeze_authority: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    rent: &Rent,
    decimals: u8,
) -> ProgramResult {
    
    // Serialize instruction data
    // Create mint account
    let mint_space = 82; 
    let mint_lamports = rent.minimum_balance(mint_space);

    invoke(
        &system_instruction::create_account(
            mint_authority.key,
            mint.key,
            mint_lamports,
            82,
            token_program.key,
        ),
        &[mint_authority.clone(), mint.clone(), system_program.clone()],
    )?;

    let data = InitializeMintInstructionData {
        discriminator: 20,
        decimals: decimals,
        mint_authority: *mint_authority.key,
        freeze_authority_option: 1,
        freeze_authority: *freeze_authority.key,
    }
    .try_to_vec()?;

    // Prepare instruction
    let ix = Instruction {
        program_id: *token_program.key,
        accounts: vec![
            AccountMeta::new(*mint.key, false),
        ],
        data,
    };

    invoke(
        &ix,
        &[
            mint.clone(),
            token_program.clone(),
        ],
    )?;

    msg!("Mint account initialized: {}", mint.key);

    Ok(())
}
