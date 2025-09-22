//! # Keystore Model
//!
//! Encrypted storage format for wallet persistence.
//! Compatible with MetaMask and other standard wallet formats.

use crate::config;
use crate::errors::{CryptographicError, ValidationError, WalletResult};
use serde::{Deserialize, Serialize};

/// UTC/JSON Keystore format (MetaMask compatible)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keystore {
    /// Keystore format version
    pub version: String,

    /// Keystore metadata (non-sensitive)
    pub metadata: KeystoreMetadata,

    /// Encrypted data and cryptographic parameters
    pub crypto: CryptoParams,
}

/// Non-sensitive keystore metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystoreMetadata {
    /// Wallet alias
    pub alias: Option<String>,

    /// Primary Ethereum address
    pub address: String,

    /// Creation timestamp (ISO 8601)
    pub created_at: String,

    /// Target network
    pub network: String,

    /// Keystore format identifier
    pub keystore_type: String,
}

/// Cryptographic parameters for encrypted data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoParams {
    /// Encryption algorithm ("aes-256-gcm")
    pub cipher: String,

    /// Encrypted wallet data (hex encoded)
    pub ciphertext: String,

    /// Cipher-specific parameters
    pub cipherparams: CipherParams,

    /// Key derivation function ("argon2id" or "pbkdf2")
    pub kdf: String,

    /// KDF parameters
    pub kdfparams: KdfParams,

    /// Message authentication code (hex encoded)
    pub mac: String,
}

/// AES-GCM cipher parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CipherParams {
    /// Initialization vector (hex encoded)
    pub iv: String,
}

/// Key derivation function parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KdfParams {
    /// Argon2id parameters (preferred)
    Argon2 {
        /// Derived key length
        dklen: u32,
        /// Memory usage in KB
        memory: u32,
        /// Time cost (iterations)
        time: u32,
        /// Parallelism degree
        parallelism: u32,
        /// Salt (hex encoded)
        salt: String,
    },
    /// PBKDF2 parameters (legacy compatibility)
    Pbkdf2 {
        /// Derived key length
        dklen: u32,
        /// Iteration count
        c: u32,
        /// Hash function
        prf: String,
        /// Salt (hex encoded)
        salt: String,
    },
}

impl Keystore {
    /// Create a new keystore structure
    pub fn new(
        alias: Option<String>,
        address: String,
        network: String,
        encrypted_data: Vec<u8>,
        _salt: Vec<u8>,
        nonce: Vec<u8>,
        mac: Vec<u8>,
        kdf_params: KdfParams,
    ) -> Self {
        let metadata = KeystoreMetadata {
            alias,
            address,
            created_at: chrono::Utc::now().to_rfc3339(),
            network,
            keystore_type: "web3wallet-cli".to_string(),
        };

        let crypto = CryptoParams {
            cipher: "aes-256-gcm".to_string(),
            ciphertext: hex::encode(encrypted_data),
            cipherparams: CipherParams {
                iv: hex::encode(nonce),
            },
            kdf: match kdf_params {
                KdfParams::Argon2 { .. } => "argon2id".to_string(),
                KdfParams::Pbkdf2 { .. } => "pbkdf2".to_string(),
            },
            kdfparams: kdf_params,
            mac: hex::encode(mac),
        };

        Self {
            version: "1.0.0".to_string(),
            metadata,
            crypto,
        }
    }

    /// Create Argon2id keystore
    pub fn with_argon2(
        alias: Option<String>,
        address: String,
        network: String,
        encrypted_data: Vec<u8>,
        salt: Vec<u8>,
        nonce: Vec<u8>,
        mac: Vec<u8>,
        memory: u32,
        iterations: u32,
        parallelism: u32,
    ) -> Self {
        let kdf_params = KdfParams::Argon2 {
            dklen: config::crypto::KEY_LENGTH as u32,
            memory,
            time: iterations,
            parallelism,
            salt: hex::encode(&salt),
        };

        Self::new(
            alias,
            address,
            network,
            encrypted_data,
            salt,
            nonce,
            mac,
            kdf_params,
        )
    }

    /// Create PBKDF2 keystore (legacy compatibility)
    pub fn with_pbkdf2(
        alias: Option<String>,
        address: String,
        network: String,
        encrypted_data: Vec<u8>,
        salt: Vec<u8>,
        nonce: Vec<u8>,
        mac: Vec<u8>,
        iterations: u32,
    ) -> Self {
        let kdf_params = KdfParams::Pbkdf2 {
            dklen: config::crypto::KEY_LENGTH as u32,
            c: iterations,
            prf: "hmac-sha256".to_string(),
            salt: hex::encode(&salt),
        };

        Self::new(
            alias,
            address,
            network,
            encrypted_data,
            salt,
            nonce,
            mac,
            kdf_params,
        )
    }

    /// Get encrypted data as bytes
    pub fn encrypted_data(&self) -> WalletResult<Vec<u8>> {
        hex::decode(&self.crypto.ciphertext).map_err(|e| {
            CryptographicError::DataCorruption {
                details: format!("Invalid ciphertext hex: {}", e),
            }
            .into()
        })
    }

    /// Get salt as bytes
    pub fn salt(&self) -> WalletResult<Vec<u8>> {
        let salt_hex = match &self.crypto.kdfparams {
            KdfParams::Argon2 { salt, .. } => salt,
            KdfParams::Pbkdf2 { salt, .. } => salt,
        };

        hex::decode(salt_hex).map_err(|e| {
            CryptographicError::DataCorruption {
                details: format!("Invalid salt hex: {}", e),
            }
            .into()
        })
    }

    /// Get nonce/IV as bytes
    pub fn nonce(&self) -> WalletResult<Vec<u8>> {
        hex::decode(&self.crypto.cipherparams.iv).map_err(|e| {
            CryptographicError::DataCorruption {
                details: format!("Invalid nonce hex: {}", e),
            }
            .into()
        })
    }

    /// Get MAC as bytes
    pub fn mac(&self) -> WalletResult<Vec<u8>> {
        hex::decode(&self.crypto.mac).map_err(|e| {
            CryptographicError::DataCorruption {
                details: format!("Invalid MAC hex: {}", e),
            }
            .into()
        })
    }

    /// Get KDF parameters
    pub fn kdf_params(&self) -> &KdfParams {
        &self.crypto.kdfparams
    }

    /// Validate keystore structure
    pub fn validate(&self) -> WalletResult<()> {
        // Validate version
        if self.version.is_empty() {
            return Err(ValidationError::InvalidKeystoreSchema {
                error: "Missing version".to_string(),
                file_path: "unknown".to_string(),
            }
            .into());
        }

        // Validate address format
        crate::utils::validate_ethereum_address(&self.metadata.address)?;

        // Validate network
        if !config::is_supported_network(&self.metadata.network) {
            return Err(ValidationError::InvalidKeystoreSchema {
                error: format!("Unsupported network: {}", self.metadata.network),
                file_path: "unknown".to_string(),
            }
            .into());
        }

        // Validate cipher
        if self.crypto.cipher != "aes-256-gcm" {
            return Err(ValidationError::InvalidKeystoreSchema {
                error: format!("Unsupported cipher: {}", self.crypto.cipher),
                file_path: "unknown".to_string(),
            }
            .into());
        }

        // Validate KDF
        match self.crypto.kdf.as_str() {
            "argon2id" | "pbkdf2" => {}
            _ => {
                return Err(ValidationError::InvalidKeystoreSchema {
                    error: format!("Unsupported KDF: {}", self.crypto.kdf),
                    file_path: "unknown".to_string(),
                }
                .into())
            }
        }

        // Validate hex fields
        self.encrypted_data()?;
        self.salt()?;
        self.nonce()?;
        self.mac()?;

        // Validate KDF parameters
        match &self.crypto.kdfparams {
            KdfParams::Argon2 {
                dklen,
                memory,
                time,
                parallelism,
                ..
            } => {
                if *dklen != config::crypto::KEY_LENGTH as u32 {
                    return Err(ValidationError::InvalidKeystoreSchema {
                        error: format!("Invalid key length: {}", dklen),
                        file_path: "unknown".to_string(),
                    }
                    .into());
                }
                if *memory == 0 || *time == 0 || *parallelism == 0 {
                    return Err(ValidationError::InvalidKeystoreSchema {
                        error: "Invalid Argon2 parameters".to_string(),
                        file_path: "unknown".to_string(),
                    }
                    .into());
                }
            }
            KdfParams::Pbkdf2 { dklen, c, prf, .. } => {
                if *dklen != config::crypto::KEY_LENGTH as u32 {
                    return Err(ValidationError::InvalidKeystoreSchema {
                        error: format!("Invalid key length: {}", dklen),
                        file_path: "unknown".to_string(),
                    }
                    .into());
                }
                if *c == 0 {
                    return Err(ValidationError::InvalidKeystoreSchema {
                        error: "Invalid PBKDF2 iteration count".to_string(),
                        file_path: "unknown".to_string(),
                    }
                    .into());
                }
                if prf != "hmac-sha256" {
                    return Err(ValidationError::InvalidKeystoreSchema {
                        error: format!("Unsupported PRF: {}", prf),
                        file_path: "unknown".to_string(),
                    }
                    .into());
                }
            }
        }

        Ok(())
    }

    /// Serialize to JSON string
    pub fn to_json(&self) -> WalletResult<String> {
        serde_json::to_string_pretty(self).map_err(|e| {
            ValidationError::InvalidKeystoreSchema {
                error: format!("JSON serialization failed: {}", e),
                file_path: "unknown".to_string(),
            }
            .into()
        })
    }

    /// Deserialize from JSON string
    pub fn from_json(json: &str) -> WalletResult<Self> {
        let keystore: Self = serde_json::from_str(json).map_err(|e| {
            ValidationError::InvalidKeystoreSchema {
                error: format!("JSON deserialization failed: {}", e),
                file_path: "unknown".to_string(),
            }
        })?;

        // Validate the deserialized keystore
        keystore.validate()?;

        Ok(keystore)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keystore_creation() {
        let keystore = Keystore::with_argon2(
            Some("test".to_string()),
            "0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99".to_string(),
            "mainnet".to_string(),
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
            47104,
            1,
            1,
        );

        assert_eq!(keystore.version, "1.0.0");
        assert_eq!(keystore.metadata.alias, Some("test".to_string()));
        assert_eq!(keystore.crypto.cipher, "aes-256-gcm");
        assert_eq!(keystore.crypto.kdf, "argon2id");
    }

    #[test]
    fn test_keystore_validation() {
        let keystore = Keystore::with_argon2(
            None,
            "0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99".to_string(),
            "mainnet".to_string(),
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
            47104,
            1,
            1,
        );

        assert!(keystore.validate().is_ok());
    }

    #[test]
    fn test_keystore_serialization() {
        let keystore = Keystore::with_argon2(
            None,
            "0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99".to_string(),
            "mainnet".to_string(),
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
            47104,
            1,
            1,
        );

        let json = keystore.to_json().unwrap();
        let restored = Keystore::from_json(&json).unwrap();

        assert_eq!(keystore.version, restored.version);
        assert_eq!(keystore.metadata.address, restored.metadata.address);
    }

    #[test]
    fn test_data_extraction() {
        let keystore = Keystore::with_argon2(
            None,
            "0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99".to_string(),
            "mainnet".to_string(),
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
            47104,
            1,
            1,
        );

        assert_eq!(keystore.encrypted_data().unwrap(), vec![1, 2, 3, 4]);
        assert_eq!(keystore.salt().unwrap(), vec![5, 6, 7, 8]);
        assert_eq!(keystore.nonce().unwrap(), vec![9, 10, 11, 12]);
        assert_eq!(keystore.mac().unwrap(), vec![13, 14, 15, 16]);
    }
}