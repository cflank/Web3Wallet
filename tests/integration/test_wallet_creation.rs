//! Integration tests for wallet creation flow
//!
//! Tests the complete end-to-end wallet creation process.

use tempfile::TempDir;
use web3wallet_cli::{WalletConfig, WalletManager, WalletResult};

/// Test complete wallet creation flow
#[tokio::test]
async fn test_wallet_creation_flow() -> WalletResult<()> {
    let temp_dir = TempDir::new().map_err(|e| {
        web3wallet_cli::WalletError::FileSystem(
            web3wallet_cli::errors::FileSystemError::DirectoryNotAccessible {
                path: "temp".to_string(),
                details: e.to_string(),
            },
        )
    })?;

    let config = WalletConfig {
        network: "testnet".to_string(),
        wallet_dir: temp_dir.path().to_path_buf(),
        kdf_iterations: 1, // Fast for testing
        kdf_memory: 1024,
        kdf_parallelism: 1,
    };

    let manager = WalletManager::new(config);

    // Test 12-word wallet creation
    let wallet_12 = manager.create_wallet(12).await?;
    assert_eq!(wallet_12.mnemonic().split_whitespace().count(), 12);
    assert!(wallet_12.address().starts_with("0x"));
    assert_eq!(wallet_12.address().len(), 42);

    // Test 24-word wallet creation
    let wallet_24 = manager.create_wallet(24).await?;
    assert_eq!(wallet_24.mnemonic().split_whitespace().count(), 24);
    assert!(wallet_24.address().starts_with("0x"));

    // Test that different wallets have different addresses
    assert_ne!(wallet_12.address(), wallet_24.address());

    Ok(())
}

/// Test wallet creation performance (<1s)
#[tokio::test]
async fn test_wallet_creation_performance() -> WalletResult<()> {
    use std::time::Instant;

    let temp_dir = TempDir::new().unwrap();
    let config = WalletConfig {
        network: "testnet".to_string(),
        wallet_dir: temp_dir.path().to_path_buf(),
        kdf_iterations: 1,
        kdf_memory: 1024,
        kdf_parallelism: 1,
    };

    let manager = WalletManager::new(config);

    let start = Instant::now();
    let _wallet = manager.create_wallet(12).await?;
    let duration = start.elapsed();

    assert!(duration.as_secs() < 1, "Wallet creation took {:?}, should be <1s", duration);

    Ok(())
}

/// Test wallet creation with encryption and save
#[tokio::test]
async fn test_wallet_creation_with_save() -> WalletResult<()> {
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

    // Test saving wallet
    let wallet_path = temp_dir.path().join("test-wallet.json");
    let password = "test_password_123";

    manager.save_wallet(&wallet, &wallet_path, password).await?;

    // Verify file exists
    assert!(wallet_path.exists());

    // Test loading wallet back
    let loaded_wallet = manager.load_wallet(&wallet_path, password).await?;

    // Verify loaded wallet matches original
    assert_eq!(wallet.address(), loaded_wallet.address());
    assert_eq!(wallet.mnemonic(), loaded_wallet.mnemonic());

    Ok(())
}