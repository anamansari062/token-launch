use borsh::BorshSerialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::{Keypair, Signer}, transaction::Transaction
};
use token_launch::{constants::TOKEN_PROGRAM_ID, LaunchConfig, LaunchpadInstruction};

use super::LaunchResult;

pub async fn launch_spl_token_legacy(
    program_id: Pubkey,
    rpc_client: RpcClient,
    payer: &Keypair,
    config: LaunchConfig,
) -> Result<LaunchResult, Box<dyn std::error::Error>> {
    let mint = Keypair::new();
    let token_account = Keypair::new();
    let metadata_account = Pubkey::find_program_address(&[b"launched_asset", mint.pubkey().as_ref()], &token_launch::id()).0;

    let accounts = vec![
        AccountMeta::new(payer.pubkey(), true),
        AccountMeta::new(mint.pubkey(), true),
        AccountMeta::new(token_account.pubkey(), true),
        AccountMeta::new(metadata_account, false),
        AccountMeta::new_readonly(solana_sdk::system_program::id(), false), 
        AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
        AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
    ];   

    let data = LaunchpadInstruction::LaunchAsset{ config}.try_to_vec().unwrap();     

    let instruction = Instruction {
        program_id: program_id,
        accounts, 
        data, 
    };

    let recent_blockhash = rpc_client.get_latest_blockhash().await?;

    let tx = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer, &mint, &token_account],
        recent_blockhash,
    );

    let result = rpc_client.send_and_confirm_transaction(&tx).await;
    match result {
        Ok(signature) => {
            println!("✅ SPL Token (legacy) launched successfully!");
            return Ok(
                LaunchResult {
                    mint: mint.pubkey(),
                    token_account: token_account.pubkey(),
                    metadata_account,
                    signature: signature.to_string(),
                }   
            )
        },
        Err(e) => return Err(Box::new(e)),
    }

}