use borsh::BorshSerialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, instruction::{AccountMeta, Instruction}, program::invoke, pubkey::Pubkey};

#[derive(BorshSerialize)]
pub struct SetAuthorityInstructionData {
    pub discriminator: u8,        // 6
    pub authority_type: u8, // 0 for MintTokens, 1 for FreezeAccount
    pub new_authority_option: u8, // 0 for None, 1 for Some
    pub new_authority: Pubkey,
}

/// CPI call to set_authority
pub fn process<'a>(
    mint: &AccountInfo<'a>,
    authority: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
) -> ProgramResult {
    // Discriminator for SetAuthority (usually 6)

    // Serialize args
    let data = SetAuthorityInstructionData {
        discriminator: 6,
        authority_type: 0, // AuthorityType::MintTokens
        new_authority_option: 0, // None Authority
        new_authority: Pubkey::default(), // Default Pubkey for None
    }.try_to_vec()?;

    // Build full account metas
    let accounts = vec![
        AccountMeta::new(*mint.key, true),
        AccountMeta::new_readonly(*authority.key, authority.is_signer),
    ];

    let ix = Instruction {
        program_id: *token_program.key,
        accounts,
        data,
    };

    invoke(
        &ix,
        &[
            mint.clone(),
            authority.clone(),
            token_program.clone(),
        ],
    )?;

    Ok(())
}
