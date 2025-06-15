#[cfg(test)]
mod test_happy {
    use borsh::BorshSerialize;
    use token_launch::{constants::{TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID}, entrypoint::process_instruction, AssetType, LaunchConfig, LaunchpadInstruction};
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
    async fn test_spl_token_legacy_launch() {

        let program_test = create_program_test();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mint = Keypair::new();
        let token_account = Keypair::new();
        let metadata_account = Pubkey::find_program_address(&[b"launched_asset", mint.pubkey().as_ref()], &token_launch::id()).0;

        let accounts = vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new(token_account.pubkey(), true),
            AccountMeta::new(metadata_account, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false), // System program
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false), // Program ID
            AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false), // Rent sysvar
        ];   

        let data = LaunchpadInstruction::LaunchAsset {
            config: LaunchConfig {
                asset_type: token_launch::state::AssetType::SplTokenLegacy,
                name: "Test Token".to_string(),
                symbol: "TTK".to_string(),
                decimals: 6,
                total_supply: 1_000_000_000, // 1 million tokens
                metadata_uri: "https://example.com/metadata.json".to_string(),
                creator: payer.pubkey(),
                is_mutable: false,
            },
        }.try_to_vec().unwrap();     

        // Create a dummy instruction (no accounts, no data)
        let instruction = Instruction {
            program_id: token_launch::id(),
            accounts, 
            data, 
        };

        // Create and send a transaction with the instruction
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer, &mint, &token_account],
            recent_blockhash,
        );

        // Process the transaction
        let result = banks_client.process_transaction(tx).await;

        match result {
            Ok(_) => assert!(true, "Transaction processed successfully"),
            Err(e) => panic!("Transaction failed: {:?}", e),
        }
    }

    #[tokio::test]
    async fn test_spl_token_2022_launch() {

        let program_test = create_program_test();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mint = Keypair::new();
        let token_account = Keypair::new();
        let metadata_account = Pubkey::find_program_address(&[b"launched_asset", mint.pubkey().as_ref()], &token_launch::id()).0;

        let accounts = vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new(token_account.pubkey(), true),
            AccountMeta::new(metadata_account, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false),
            AccountMeta::new_readonly(TOKEN_2022_PROGRAM_ID, false),
            AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false),
        ];   

        let data = LaunchpadInstruction::LaunchAsset {
            config: LaunchConfig {
                asset_type: token_launch::state::AssetType::SplToken2022,
                name: "Test Token 2022".to_string(),
                symbol: "TTK2".to_string(),
                decimals: 6,
                total_supply: 500_000_000, // 500k tokens
                metadata_uri: "https://example.com/metadata.json".to_string(),
                creator: payer.pubkey(),
                is_mutable: false,
            },
        }.try_to_vec().unwrap();     

        // Create a dummy instruction (no accounts, no data)
        let instruction = Instruction {
            program_id: token_launch::id(),
            accounts, // No accounts needed for this simple test
            data,     // No instruction data either
        };

        // Create and send a transaction with the instruction
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer, &mint, &token_account],
            recent_blockhash,
        );

        // Process the transaction
        let result = banks_client.process_transaction(tx).await;

        match result {
            Ok(_) => assert!(true, "Transaction processed successfully"),
            Err(e) => panic!("Transaction failed: {:?}", e),
        }


    }

    #[tokio::test]
    async fn test_standard_nft_launch() {

        // Create a test environment with your program
        let program_test = ProgramTest::new(
            "token_launch", // The name of your crate (matches `Cargo.toml`)
            token_launch::id(), // The program ID from `declare_id!`      ,
            processor!(process_instruction), // Wraps your entrypoint
        );

        // Start the test environment
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        let mint = Keypair::new();
        let token_account = Keypair::new();
        let metadata_account = Pubkey::find_program_address(&[b"launched_asset", mint.pubkey().as_ref()], &token_launch::id()).0;

        let accounts = vec![
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new(mint.pubkey(), true),
            AccountMeta::new(token_account.pubkey(), true),
            AccountMeta::new(metadata_account, false),
            AccountMeta::new_readonly(solana_sdk::system_program::id(), false), // System program
            AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false), // Program ID
            AccountMeta::new_readonly(solana_sdk::sysvar::rent::id(), false), // Rent sysvar
        ];   

        let data = LaunchpadInstruction::LaunchAsset {
            config: LaunchConfig {
                asset_type: token_launch::state::AssetType::StandardNft,
                name: "Test NFT".to_string(),
                symbol: "TN".to_string(),
                decimals: 0,
                total_supply: 1,
                metadata_uri: "https://example.com/metadata.json".to_string(),
                creator: payer.pubkey(),
                is_mutable: false,
            },
        }.try_to_vec().unwrap();     

        // Create a dummy instruction (no accounts, no data)
        let instruction = Instruction {
            program_id: token_launch::id(),
            accounts, // No accounts needed for this simple test
            data,     // No instruction data either
        };

        // Create and send a transaction with the instruction
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&payer.pubkey()),
            &[&payer, &mint, &token_account],
            recent_blockhash,
        );

        // Process the transaction
        let result = banks_client.process_transaction(tx).await;

        match result {
            Ok(_) => assert!(true, "Transaction processed successfully"),
            Err(e) => panic!("Transaction failed: {:?}", e),
        }


    }

    #[tokio::test]
    async fn test_multiple_successful_launches() {
        let program_test = create_program_test();
        let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

        // Launch multiple different tokens
        for i in 0..3 {
            let mint = Keypair::new();
            let token_account = Keypair::new();
            let metadata_account = Pubkey::find_program_address(
                &[b"launched_asset", mint.pubkey().as_ref()], 
                &token_launch::id()
            ).0;

            let config = LaunchConfig {
                asset_type: AssetType::SplTokenLegacy,
                name: format!("Multi Token {}", i),
                symbol: format!("MT{}", i),
                decimals: 6,
                total_supply: 1_000_000 * (i + 1) as u64,
                metadata_uri: format!("https://example.com/multi-{}-metadata.json", i),
                creator: payer.pubkey(),
                is_mutable: i % 2 == 0, // Alternate mutability
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
            assert!(result.is_ok(), "Multi launch {} should succeed", i);
        }
    }

}
