//! # Wallet Model
//!
//! Core wallet data structure with HD derivation support.
//! Implements secure patterns with zeroize for memory cleanup.

use crate::config;
use crate::errors::{CryptographicError, WalletResult};
use ethers::prelude::*;
use ethers::signers::coins_bip39::English;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// HD Wallet with BIP39/BIP44 support
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct Wallet {
    /// BIP39 mnemonic phrase
    #[zeroize(skip)]
    mnemonic: String,

    /// Master private key derived from mnemonic
    #[serde(skip)]
    master_private_key: Option<Vec<u8>>,

    /// Primary Ethereum address (index 0)
    #[zeroize(skip)]
    address: String,

    /// Base HD derivation path
    #[zeroize(skip)]
    derivation_path: String,

    /// Target network
    #[zeroize(skip)]
    network: String,

    /// Wallet creation timestamp
    #[zeroize(skip)]
    created_at: chrono::DateTime<chrono::Utc>,

    /// Optional wallet alias
    #[zeroize(skip)]
    alias: Option<String>,
}

impl Wallet {
    /// Create a new wallet from mnemonic
    pub fn from_mnemonic(
        mnemonic: &str,
        network: &str,
        alias: Option<String>,
    ) -> WalletResult<Self> {
        // Validate mnemonic
        let bip39_mnemonic = bip39::Mnemonic::from_str(mnemonic).map_err(|e| {
            CryptographicError::InvalidMnemonic {
                details: e.to_string(),
                suggestion: "Verify the mnemonic phrase has the correct number of words (12 or 24) and all words are from the BIP39 wordlist.".to_string(),
            }
        })?;

        // Generate seed from mnemonic
        let seed = bip39_mnemonic.to_seed("");

        // Create HD wallet
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(mnemonic)
            .build()
            .map_err(|e| CryptographicError::AddressGenerationFailed {
                details: e.to_string(),
            })?;

        let address = format!("{:?}", wallet.address());
        let derivation_path = config::DEFAULT_DERIVATION_PATH.to_string();

        Ok(Self {
            mnemonic: mnemonic.to_string(),
            master_private_key: Some(seed.to_vec()),
            address,
            derivation_path,
            network: network.to_string(),
            created_at: chrono::Utc::now(),
            alias,
        })
    }

    /// Create wallet from private key
    pub fn from_private_key(
        private_key: &str,
        network: &str,
        alias: Option<String>,
    ) -> WalletResult<Self> {
        // Remove 0x prefix if present
        let key_str = private_key.strip_prefix("0x").unwrap_or(private_key);

        // Validate private key format
        if key_str.len() != 64 {
            return Err(CryptographicError::InvalidPrivateKey {
                details: format!("Expected 64 hex characters, got {}", key_str.len()),
                expected: "64 hex characters (with or without 0x prefix)".to_string(),
            }
            .into());
        }

        // Parse private key
        // Parse private key into wallet
        let wallet = key_str.parse::<LocalWallet>().map_err(|e| {
            CryptographicError::InvalidPrivateKey {
                details: e.to_string(),
                expected: "valid secp256k1 private key".to_string(),
            }
        })?;
        let address = format!("{:?}", wallet.address());

        Ok(Self {
            mnemonic: String::new(), // No mnemonic for private key import
            master_private_key: Some(vec![]), // Placeholder for now
            address,
            derivation_path: config::DEFAULT_DERIVATION_PATH.to_string(),
            network: network.to_string(),
            created_at: chrono::Utc::now(),
            alias,
        })
    }

    /// Generate a new random wallet
    pub fn generate(
        word_count: u8,
        network: &str,
        alias: Option<String>,
    ) -> WalletResult<Self> {
        // Validate word count
        if !config::is_supported_word_count(word_count) {
            return Err(CryptographicError::InvalidMnemonic {
                details: format!("Unsupported word count: {}", word_count),
                suggestion: "Use 12 or 24 words".to_string(),
            }
            .into());
        }
        
        // Get entropy bits for word count
        let entropy_bits = config::entropy_bits_for_word_count(word_count).unwrap();

        // Generate random entropy
        let mut entropy = vec![0u8; entropy_bits / 8];
        use rand::RngCore;
        rand::thread_rng().fill_bytes(&mut entropy);

        // Create mnemonic from entropy
        let mnemonic = bip39::Mnemonic::from_entropy(&entropy).map_err(|e| {
            CryptographicError::InvalidMnemonic {
                details: e.to_string(),
                suggestion: "Ensure system has adequate entropy sources".to_string(),
            }
        })?;

        Self::from_mnemonic(&mnemonic.to_string(), network, alias)
    }

    /// Get wallet address
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Get mnemonic phrase (empty for private key imports)
    pub fn mnemonic(&self) -> &str {
        &self.mnemonic
    }

    /// Get network
    pub fn network(&self) -> &str {
        &self.network
    }

    /// Get derivation path
    pub fn derivation_path(&self) -> &str {
        &self.derivation_path
    }

    /// Get creation timestamp
    pub fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    /// Get wallet alias
    pub fn alias(&self) -> Option<&str> {
        self.alias.as_deref()
    }

    /// Set wallet alias
    pub fn set_alias(&mut self, alias: Option<String>) {
        self.alias = alias;
    }

    /// Check if wallet has mnemonic (vs private key only)
    pub fn has_mnemonic(&self) -> bool {
        !self.mnemonic.is_empty()
    }

    /// Get private key (for internal use only)
    pub(crate) fn private_key_bytes(&self) -> Option<&[u8]> {
        self.master_private_key.as_deref()
    }

    /// Derive address at specific index
    pub fn derive_address(&self, index: u32) -> WalletResult<DerivedAddress> {
        if self.mnemonic.is_empty() {
            return Err(CryptographicError::KdfFailed {
                details: "Cannot derive addresses from private key only wallet".to_string(),
            }
            .into());
        }

        let derivation_path = format!("{}/{}", self.derivation_path, index);

        // Create wallet from mnemonic with specific derivation path
        let wallet = MnemonicBuilder::<English>::default()
            .phrase(self.mnemonic.as_str())
            .derivation_path(&derivation_path)
            .map_err(|_e| CryptographicError::InvalidDerivationPath {
                path: derivation_path.clone(),
                expected: "valid BIP44 derivation path".to_string(),
            })?
            .build()
            .map_err(|e| CryptographicError::AddressGenerationFailed {
                details: e.to_string(),
            })?;

        let address = format!("{:?}", wallet.address());

        Ok(DerivedAddress {
            address,
            index,
            derivation_path,
        })
    }

    /// Validate wallet consistency
    pub fn validate(&self) -> WalletResult<()> {
        // Validate address format
        crate::utils::validate_ethereum_address(&self.address)?;

        // Validate network
        if !config::is_supported_network(&self.network) {
            return Err(CryptographicError::KdfFailed {
                details: format!("Unsupported network: {}", self.network),
            }
            .into());
        }

        // Validate derivation path
        crate::utils::validate_derivation_path(&self.derivation_path)?;

        Ok(())
    }
}

/// Derived address from HD wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivedAddress {
    /// Ethereum address
    address: String,
    /// Derivation index
    index: u32,
    /// Full derivation path
    derivation_path: String,
}

impl DerivedAddress {
    /// Get address
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Get derivation index
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Get derivation path
    pub fn derivation_path(&self) -> &str {
        &self.derivation_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MNEMONIC: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    const EXPECTED_ADDRESS: &str = "0x9858effd232b4033e47d90003d41ec34ecaeda94";

    #[test]
    fn test_wallet_from_mnemonic() {
        let wallet = Wallet::from_mnemonic(TEST_MNEMONIC, "mainnet", None).unwrap();

        assert_eq!(wallet.address(), EXPECTED_ADDRESS);
        assert_eq!(wallet.mnemonic(), TEST_MNEMONIC);
        assert_eq!(wallet.network(), "mainnet");
        assert!(wallet.has_mnemonic());
    }

    #[test]
    fn test_wallet_generation() {
        let wallet = Wallet::generate(12, "mainnet", Some("test".to_string())).unwrap();

        assert!(wallet.address().starts_with("0x"));
        assert_eq!(wallet.address().len(), 42);
        assert_eq!(wallet.mnemonic().split_whitespace().count(), 12);
        assert_eq!(wallet.alias(), Some("test"));
    }

    #[test]
    fn test_address_derivation() {
        let wallet = Wallet::from_mnemonic(TEST_MNEMONIC, "mainnet", None).unwrap();

        let derived = wallet.derive_address(1).unwrap();
        assert!(derived.address().starts_with("0x"));
        assert_eq!(derived.index(), 1);
        assert!(derived.derivation_path().ends_with("/1"));
    }

    #[test]
    fn test_wallet_validation() {
        let wallet = Wallet::from_mnemonic(TEST_MNEMONIC, "mainnet", None).unwrap();
        assert!(wallet.validate().is_ok());
    }

    #[test]
    fn test_invalid_mnemonic() {
        let result = Wallet::from_mnemonic("invalid mnemonic", "mainnet", None);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_word_count() {
        let result = Wallet::generate(16, "mainnet", None);
        assert!(result.is_err());
    }
}