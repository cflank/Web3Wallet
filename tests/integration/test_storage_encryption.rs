//! Integration tests for encrypted storage roundtrip

use tempfile::TempDir;
use web3wallet_cli::{WalletConfig, WalletManager, WalletResult};

/// Test encryption/decryption roundtrip
#[tokio::test]
async fn test_storage_encryption_roundtrip() -> WalletResult<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = WalletConfig {
        network: "testnet".to_string(),
        wallet_dir: temp_dir.path().to_path_buf(),
        kdf_iterations: 1,
        kdf_memory: 1024,
        kdf_parallelism: 1,
    };

    let manager = WalletManager::new(config);
    let original_wallet = manager.create_wallet(12).await?;

    let wallet_path = temp_dir.path().join("encrypted-wallet.json");
    let password = "strong_password_123!";

    // Save with encryption
    manager.save_wallet(&original_wallet, &wallet_path, password).await?;

    // Load and decrypt
    let loaded_wallet = manager.load_wallet(&wallet_path, password).await?;

    // Verify data integrity
    assert_eq!(original_wallet.address(), loaded_wallet.address());
    assert_eq!(original_wallet.mnemonic(), loaded_wallet.mnemonic());

    Ok(())
}

/// Test wrong password handling
#[tokio::test]
async fn test_wrong_password_error() -> WalletResult<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = WalletConfig {
        network: "testnet".to_string(),
        wallet_dir: temp_dir.path().to_path_buf(),
        kdf_iterations: 1,
        kdf_memory: 1024,
        kdf_parallelism: 1,
    };

    let manager = WalletManager::new(config);
    let wallet = manager.create_wallet(12).await?;

    let wallet_path = temp_dir.path().join("encrypted-wallet.json");
    let correct_password = "correct_password";
    let wrong_password = "wrong_password";

    // Save with correct password
    manager.save_wallet(&wallet, &wallet_path, correct_password).await?;

    // Try to load with wrong password
    let result = manager.load_wallet(&wallet_path, wrong_password).await;

    assert!(result.is_err());
    // Should be an authentication error
    match result.unwrap_err() {
        web3wallet_cli::WalletError::Authentication(_) => {},
        other => panic!("Expected authentication error, got: {:?}", other),
    }

    Ok(())
}