#[cfg(test)]
mod test_configs {
    use borsh::BorshSerialize;
    use token_launch::{
        constants::TOKEN_PROGRAM_ID, 
        entrypoint::process_instruction, 
        LaunchConfig, 
        LaunchpadInstruction,
        state::AssetType,
    };
    use solana_program_test::{processor, ProgramTest};
    use solana_sdk::{
        instruction::{AccountMeta, Instruction}, 
        pubkey::Pubkey, 
        signature::Keypair, 
        signer::Signer, 
        transaction::Transaction
    };

    // Helper function to create program test environment
    fn create_program_test() -> ProgramTest {
        ProgramTest::new(
            "token_launch",
            token_launch::id(),
            processor!(process_instruction),
        )
    }

    // Helper function to create launch instruction
    fn create_launch_instruction(
        payer: &Pubkey,
        mint: &Pubkey,
        token_account: &Pubkey,
        metadata_account: &Pubkey,
        token_program: &Pubkey,
        config: LaunchConfig,
    ) -> Instruction {
        let accounts = vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new(*mint, true),
            AccountMeta::new(*token_account, true),
            AccountMeta::new(*metadata_account, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            AccountMeta::new_readonly(*token_program, false),
            AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        ];

        let data = LaunchpadInstruction::LaunchAsset { config }
            .try_to_vec()
            .unwrap();

        Instruction {
            program_id: token_launch::id(),
            accounts,
            data,
        }
    }

    #[tokio::test]
    async fn test_launch_with_maximum_supply() {
        let program_test = create_program_test();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mint = Keypair::new();
        let token_account = Keypair::new();
        let metadata_account = Pubkey::find_program_address(
            &[b"launched_asset", mint.pubkey().as_ref()], 
            &token_launch::id()
        ).0;

        let config = LaunchConfig {
            asset_type: AssetType::SplTokenLegacy,
            name: "Max Supply Token".to_string(),
            symbol: "MAX".to_string(),
            decimals: 9,
            total_supply: u64::MAX, // Maximum possible supply
            metadata_uri: "https://example.com/max-metadata.json".to_string(),
            creator: payer.pubkey(),
            is_mutable: true,
        };

        let instruction = create_launch_instruction(
            &payer.pubkey(),
            &mint.pubkey(),
            &token_account.pubkey(),
            &metadata_account,
            &TOKEN_PROGRAM_ID,
            config,
        );

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer, &mint, &token_account],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(tx).await;
        assert!(result.is_ok(), "Max supply token launch should succeed");
    }

    #[tokio::test]
    async fn test_launch_with_invalid_decimals() {
        let program_test = create_program_test();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mint = Keypair::new();
        let token_account = Keypair::new();
        let metadata_account = Pubkey::find_program_address(
            &[b"launched_asset", mint.pubkey().as_ref()], 
            &token_launch::id()
        ).0;

        let config = LaunchConfig {
            asset_type: AssetType::SplTokenLegacy,
            name: "Invalid Decimals Token".to_string(),
            symbol: "INV".to_string(),
            decimals: 255, // Invalid decimals (should be <= 9 for SPL tokens)
            total_supply: 1_000_000,
            metadata_uri: "https://example.com/invalid-metadata.json".to_string(),
            creator: payer.pubkey(),
            is_mutable: false,
        };

        let instruction = create_launch_instruction(
            &payer.pubkey(),
            &mint.pubkey(),
            &token_account.pubkey(),
            &metadata_account,
            &TOKEN_PROGRAM_ID,
            config,
        );

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer, &mint, &token_account],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(tx).await;
        assert!(result.is_err(), "Invalid decimals should fail");
    }

    #[tokio::test]
    async fn test_launch_with_long_name_should_fail() {
        let program_test = create_program_test();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mint = Keypair::new();
        let token_account = Keypair::new();
        let metadata_account = Pubkey::find_program_address(
            &[b"launched_asset", mint.pubkey().as_ref()], 
            &token_launch::id()
        ).0;

        let long_name = "A".repeat(100); // Very long name, Max allowed is 32
        let config = LaunchConfig {
            asset_type: AssetType::SplTokenLegacy,
            name: long_name,
            symbol: "LONG".to_string(),
            decimals: 6,
            total_supply: 1_000_000,
            metadata_uri: "https://example.com/long-metadata.json".to_string(),
            creator: payer.pubkey(),
            is_mutable: false,
        };

        let instruction = create_launch_instruction(
            &payer.pubkey(),
            &mint.pubkey(),
            &token_account.pubkey(),
            &metadata_account,
            &TOKEN_PROGRAM_ID,
            config,
        );

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer, &mint, &token_account],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(tx).await;
        // This might succeed or fail depending on your validation logic
        assert!(result.is_err(), "Name longer than 32 characters should fail");
    }

    #[tokio::test]
    async fn test_launch_with_invalid_metadata_uri() {
        let program_test = create_program_test();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mint = Keypair::new();
        let token_account = Keypair::new();
        let metadata_account = Pubkey::find_program_address(
            &[b"launched_asset", mint.pubkey().as_ref()], 
            &token_launch::id()
        ).0;

        let config = LaunchConfig {
            asset_type: AssetType::SplTokenLegacy,
            name: "Invalid URI Token".to_string(),
            symbol: "IURI".to_string(),
            decimals: 6,
            total_supply: 1_000_000,
            metadata_uri: "not-a-valid-uri".to_string(), // Invalid URI
            creator: payer.pubkey(),
            is_mutable: false,
        };

        let instruction = create_launch_instruction(
            &payer.pubkey(),
            &mint.pubkey(),
            &token_account.pubkey(),
            &metadata_account,
            &TOKEN_PROGRAM_ID,
            config,
        );

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer, &mint, &token_account],
            recent_blockhash,
        );

        let result = banks_client.process_transaction(tx).await;
        // Depending on your validation, this might succeed or fail
        println!("Invalid URI result: {:?}", result);
    }

    #[tokio::test]
    async fn test_duplicate_mint_launch_should_fail() {
        let program_test = create_program_test();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mint = Keypair::new();
        let token_account1 = Keypair::new();
        let token_account2 = Keypair::new();
        let metadata_account = Pubkey::find_program_address(
            &[b"launched_asset", mint.pubkey().as_ref()], 
            &token_launch::id()
        ).0;

        let config = LaunchConfig {
            asset_type: AssetType::SplTokenLegacy,
            name: "Duplicate Mint Token".to_string(),
            symbol: "DUP".to_string(),
            decimals: 6,
            total_supply: 1_000_000,
            metadata_uri: "https://example.com/dup-metadata.json".to_string(),
            creator: payer.pubkey(),
            is_mutable: false,
        };

        // First launch
        let instruction1 = create_launch_instruction(
            &payer.pubkey(),
            &mint.pubkey(),
            &token_account1.pubkey(),
            &metadata_account,
            &TOKEN_PROGRAM_ID,
            config.clone(),
        );

        let tx1 = Transaction::new_signed_with_payer(
            &[instruction1],
            Some(&payer.pubkey()),
            &[&payer, &mint, &token_account1],
            recent_blockhash,
        );

        let result1 = banks_client.process_transaction(tx1).await;
        
        // Second launch with same mint should fail
        let instruction2 = create_launch_instruction(
            &payer.pubkey(),
            &mint.pubkey(),
            &token_account2.pubkey(),
            &metadata_account,
            &TOKEN_PROGRAM_ID,
            config,
        );

        let tx2 = Transaction::new_signed_with_payer(
            &[instruction2],
            Some(&payer.pubkey()),
            &[&payer, &mint, &token_account2],
            recent_blockhash,
        );

        let result2 = banks_client.process_transaction(tx2).await;
        
        // First should succeed, second should fail
        assert!(result1.is_ok(), "First launch should succeed");
        assert!(result2.is_err(), "Duplicate mint launch should fail");
    }

}