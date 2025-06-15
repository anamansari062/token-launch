use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, instruction::{AccountMeta, Instruction}, program::invoke, rent::Rent, system_instruction};
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize)]
pub struct InitializeTokenAccountInstructionData {
    pub discriminator: u8,  // 18
    pub owner: Pubkey
}


/// CPI call to initializeMint
pub fn process<'a>(
    payer: &AccountInfo<'a>,
    token_account: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    rent: &Rent,
) -> ProgramResult {

    // Create token account
    let account_space = 165;
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
    
    // Instruction discriminator for `InitializeAccount` is 18
    let data = InitializeTokenAccountInstructionData {
        discriminator: 18,
        owner: *payer.key
    }
    .try_to_vec()?;

    let accounts = vec![
        AccountMeta::new(*token_account.key, false),
        AccountMeta::new_readonly(*mint.key, false),
    ];

    let ix = Instruction {
        program_id: *token_program.key,
        accounts,
        data,
    };

    invoke(
        &ix,
        &[
            token_account.clone(),
            mint.clone(),
            payer.clone(),
        ],
    )
}
