//! Integration tests for MetaMask compatibility

use web3wallet_cli::{WalletConfig, WalletManager, WalletResult};
use tempfile::TempDir;

// Known test vectors for MetaMask compatibility
const METAMASK_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
const METAMASK_ADDRESS_0: &str = "0x9858EfFD232B4033E47d90003D41EC34EcaEda94"; // m/44'/60'/0'/0/0
const METAMASK_ADDRESS_1: &str = "0x6Fac4D18c912343BF86fa7049364Dd4E424Ab9C0"; // m/44'/60'/0'/0/1

/// Test that generated addresses match MetaMask for same mnemonic
#[tokio::test]
async fn test_metamask_address_compatibility() -> WalletResult<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = WalletConfig {
        network: "mainnet".to_string(), // Use mainnet for compatibility
        wallet_dir: temp_dir.path().to_path_buf(),
        kdf_iterations: 1,
        kdf_memory: 1024,
        kdf_parallelism: 1,
    };

    let manager = WalletManager::new(config);

    // Import the test mnemonic
    let wallet = manager.import_from_mnemonic(METAMASK_MNEMONIC).await?;

    // First address (index 0) should match MetaMask
    assert_eq!(wallet.address(), METAMASK_ADDRESS_0,
        "Address mismatch: expected {}, got {}", METAMASK_ADDRESS_0, wallet.address());

    Ok(())
}

/// Test HD derivation compatibility with MetaMask
#[tokio::test]
async fn test_metamask_hd_derivation_compatibility() -> WalletResult<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = WalletConfig {
        network: "mainnet".to_string(),
        wallet_dir: temp_dir.path().to_path_buf(),
        kdf_iterations: 1,
        kdf_memory: 1024,
        kdf_parallelism: 1,
    };

    let manager = WalletManager::new(config);
    let wallet = manager.import_from_mnemonic(METAMASK_MNEMONIC).await?;

    // Test derivation at index 1
    let derived_address = manager.derive_address(&wallet, 1).await?;

    assert_eq!(derived_address.address(), METAMASK_ADDRESS_1,
        "Derived address mismatch: expected {}, got {}", METAMASK_ADDRESS_1, derived_address.address());

    Ok(())
}

/// Test keystore format compatibility
#[tokio::test]
async fn test_metamask_keystore_format() -> WalletResult<()> {
    let temp_dir = TempDir::new().unwrap();
    let config = WalletConfig {
        network: "mainnet".to_string(),
        wallet_dir: temp_dir.path().to_path_buf(),
        kdf_iterations: 1,
        kdf_memory: 1024,
        kdf_parallelism: 1,
    };

    let manager = WalletManager::new(config);
    let wallet = manager.import_from_mnemonic(METAMASK_MNEMONIC).await?;

    let wallet_path = temp_dir.path().join("metamask-compat.json");
    let password = "test123";

    // Save wallet
    manager.save_wallet(&wallet, &wallet_path, password).await?;

    // Read the JSON file and verify structure
    let keystore_content = std::fs::read_to_string(&wallet_path).unwrap();
    let keystore_json: serde_json::Value = serde_json::from_str(&keystore_content).unwrap();

    // Verify it has the expected UTC/JSON keystore structure
    assert!(keystore_json.get("version").is_some());
    assert!(keystore_json.get("crypto").is_some());

    let crypto = keystore_json.get("crypto").unwrap();
    assert!(crypto.get("cipher").is_some());
    assert!(crypto.get("ciphertext").is_some());
    assert!(crypto.get("kdf").is_some());
    assert!(crypto.get("mac").is_some());

    Ok(())
}