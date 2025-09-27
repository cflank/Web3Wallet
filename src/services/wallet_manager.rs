//! # Wallet Manager Service
//!
//! High-level wallet management service that coordinates all wallet operations.

use crate::errors::{WalletResult};
use crate::models::{Address, Wallet};
use crate::services::{crypto::CryptoService, mnemonic::MnemonicService};
use crate::WalletConfig;
use std::path::Path;

/// Main wallet management service
pub struct WalletManager {
    config: WalletConfig,
}

impl WalletManager {
    /// Create a new wallet manager
    pub fn new(config: WalletConfig) -> Self {
        Self { config }
    }

    /// Create a new wallet with specified word count
    pub async fn create_wallet(&self, word_count: u8) -> WalletResult<Wallet> {
        let mnemonic = MnemonicService::generate(word_count)?;
        Wallet::from_mnemonic(
            mnemonic.phrase(),
            &self.config.network,
            None,
        )
    }

    /// Import wallet from mnemonic
    pub async fn import_from_mnemonic(&self, mnemonic_str: &str) -> WalletResult<Wallet> {
        let mnemonic = MnemonicService::validate(mnemonic_str)?;
        Wallet::from_mnemonic(
            mnemonic.phrase(),
            &self.config.network,
            None,
        )
    }

    /// Import wallet from private key
    pub async fn import_from_private_key(&self, private_key: &str) -> WalletResult<Wallet> {
        Wallet::from_private_key(
            private_key,
            &self.config.network,
            None,
        )
    }

    /// Save wallet to encrypted file
    pub async fn save_wallet(
        &self,
        wallet: &Wallet,
        path: &Path,
        password: &str,
    ) -> WalletResult<()> {
        // Validate password strength
        CryptoService::validate_password(password)?;

        // Encrypt wallet data using Argon2id
        let keystore = CryptoService::encrypt_wallet(wallet, password, true)?;

        // Save keystore to file
        CryptoService::save_keystore(&keystore, path).await
    }

    /// Load wallet from encrypted file
    pub async fn load_wallet(&self, path: &Path, password: &str) -> WalletResult<Wallet> {
        // Load keystore from file
        let keystore = CryptoService::load_keystore(path).await?;

        // Decrypt and return wallet
        CryptoService::decrypt_wallet(&keystore, password)
    }

    /// Derive address from wallet
    pub async fn derive_address(&self, wallet: &Wallet, index: u32) -> WalletResult<Address> {
        let derived = wallet.derive_address(index)?;
        Address::derived(
            derived.address().to_string(),
            wallet.network().to_string(),
            index,
            derived.derivation_path().to_string(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn test_config() -> WalletConfig {
        let temp_dir = TempDir::new().unwrap();
        WalletConfig {
            network: "mainnet".to_string(),
            wallet_dir: temp_dir.path().to_path_buf(),
            kdf_iterations: 1,
            kdf_memory: 1024,
            kdf_parallelism: 1,
        }
    }

    #[tokio::test]
    async fn test_wallet_creation() {
        let manager = WalletManager::new(test_config());
        let wallet = manager.create_wallet(12).await.unwrap();

        assert_eq!(wallet.mnemonic().split_whitespace().count(), 12);
        assert!(wallet.address().starts_with("0x"));
        assert_eq!(wallet.address().len(), 42);
    }

    #[tokio::test]
    async fn test_wallet_import() {
        let manager = WalletManager::new(test_config());
        let test_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

        let wallet = manager.import_from_mnemonic(test_mnemonic).await.unwrap();

        assert_eq!(wallet.mnemonic(), test_mnemonic);
        assert!(wallet.address().starts_with("0x"));
    }
}