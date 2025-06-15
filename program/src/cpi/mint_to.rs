use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke
};
use borsh::BorshSerialize;

#[derive(BorshSerialize)]
pub struct MintToInstructionData {
    pub discriminator: u8, // 7
    pub amount: u64
}

/// CPI call to mint tokens to a token account
pub fn process<'a>(
    token_program: &AccountInfo<'a>,
    mint: &AccountInfo<'a>,
    token_account: &AccountInfo<'a>,
    mint_authority: &AccountInfo<'a>,
    amount: u64,
    remaining_accounts: &[AccountInfo<'a>], // optional multisig
) -> ProgramResult {
    
    // Instruction discriminator for `MintTo` is 7
    let data = MintToInstructionData {
        discriminator: 7,
        amount
    }
    .try_to_vec()?;

    let accounts = vec![
        AccountMeta::new(*mint.key, true),
        AccountMeta::new(*token_account.key, true),
        AccountMeta::new_readonly(*mint_authority.key, true),
    ];

    let ix = Instruction {
        program_id: *token_program.key,
        accounts,
        data,
    };

    let mut cpi_accounts = vec![
        mint.clone(),
        token_account.clone(),
        mint_authority.clone(),
    ];
    cpi_accounts.extend_from_slice(remaining_accounts);

    invoke(
        &ix,
        &cpi_accounts,
    )
}
