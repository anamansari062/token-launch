#[cfg(test)]
mod test_unhappy {
    
    use borsh::BorshSerialize;
    use token_launch::{constants::TOKEN_PROGRAM_ID, entrypoint::process_instruction, AssetType, LaunchConfig, LaunchpadInstruction};
    use solana_program_test::{processor, ProgramTest};
    use solana_sdk::{instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::Keypair, signer::Signer, transaction::Transaction};

    // Helper function to create program test environment
    fn create_program_test() -> ProgramTest {
        ProgramTest::new(
            "token_launch",
            token_launch::id(),
            processor!(process_instruction),
        )
    }

    #[tokio::test]
    async fn test_launch_with_missing_accounts() {
        let program_test = create_program_test();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mint = Keypair::new();
        let metadata_account = Pubkey::find_program_address(
            &[b"launched_asset", mint.pubkey().as_ref()], 
            &token_launch::id()
        ).0;

        // Missing token_account in accounts
        let accounts = vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(mint.pubkey(), true),
            // Missing token account
            AccountMeta::new(metadata_account, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false), 
            AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        ];

        let config = LaunchConfig {
            asset_type: AssetType::SplToken2022,
            name: "Wrong Token Program".to_string(),
            symbol: "WTP".to_string(),
            decimals: 6,
            total_supply: 1_000_000,
            metadata_uri: "https://example.com/wrong-metadata.json".to_string(),
            creator: payer.pubkey(),
            is_mutable: false,
        };

        let data = LaunchpadInstruction::LaunchAsset { config }
            .try_to_vec()
            .unwrap();

        let instruction = Instruction {
            program_id: token_launch::id(),
            accounts,
            data,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer, &mint],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(tx).await;
        assert!(result.is_err(), "Missing accounts should fail");
    }

    #[tokio::test]
    async fn test_launch_with_wrong_token_program() {
        let program_test = create_program_test();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mint = Keypair::new();
        let token_account = Keypair::new();
        let metadata_account = Pubkey::find_program_address(
            &[b"launched_asset", mint.pubkey().as_ref()], 
            &token_launch::id()
        ).0;

        // Missing token_account in accounts
        let accounts = vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new(token_account.pubkey(), true),
            AccountMeta::new(metadata_account, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false), // Should be TOKEN_2022_PROGRAM_ID
            AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        ];

        let config = LaunchConfig {
            asset_type: AssetType::SplToken2022,
            name: "Missing Account Token".to_string(),
            symbol: "MISS".to_string(),
            decimals: 6,
            total_supply: 1_000_000,
            metadata_uri: "https://example.com/missing-metadata.json".to_string(),
            creator: payer.pubkey(),
            is_mutable: false,
        };

        let data = LaunchpadInstruction::LaunchAsset { config }
            .try_to_vec()
            .unwrap();

        let instruction = Instruction {
            program_id: token_launch::id(),
            accounts,
            data,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer, &mint, &token_account],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(tx).await;
        assert!(result.is_err(), "Wrong token program passed should fail");
    }

}