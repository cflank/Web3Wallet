//! Integration tests for wallet import flow

use tempfile::TempDir;
use web3wallet_cli::{WalletConfig, WalletManager, WalletResult};

const TEST_MNEMONIC_12: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
const TEST_PRIVATE_KEY: &str = "0x4c0883a69102937d6231471b5dbb6204fe512961708279c1e3ae83da5e56df1a";
const EXPECTED_ADDRESS: &str = "0x9858EfFD232B4033E47d90003D41EC34EcaEda94";

/// Test wallet import from mnemonic
#[tokio::test]
async fn test_wallet_import_from_mnemonic() -> WalletResult<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = WalletConfig {
        network: "testnet".to_string(),
        wallet_dir: temp_dir.path().to_path_buf(),
        kdf_iterations: 1,
        kdf_memory: 1024,
        kdf_parallelism: 1,
    };

    let manager = WalletManager::new(config);

    let wallet = manager.import_from_mnemonic(TEST_MNEMONIC_12).await?;

    assert_eq!(wallet.address(), EXPECTED_ADDRESS);
    assert_eq!(wallet.mnemonic(), TEST_MNEMONIC_12);

    Ok(())
}

/// Test wallet import from private key
#[tokio::test]
async fn test_wallet_import_from_private_key() -> WalletResult<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = WalletConfig {
        network: "testnet".to_string(),
        wallet_dir: temp_dir.path().to_path_buf(),
        kdf_iterations: 1,
        kdf_memory: 1024,
        kdf_parallelism: 1,
    };

    let manager = WalletManager::new(config);

    let wallet = manager.import_from_private_key(TEST_PRIVATE_KEY).await?;

    assert_eq!(wallet.address(), EXPECTED_ADDRESS);

    Ok(())
}

/// Test import performance
#[tokio::test]
async fn test_wallet_import_performance() -> WalletResult<()> {
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
    let _wallet = manager.import_from_mnemonic(TEST_MNEMONIC_12).await?;
    let duration = start.elapsed();

    assert!(duration.as_secs() < 1, "Wallet import took {:?}, should be <1s", duration);

    Ok(())
}