//! # Cryptographic Service
//!
//! Secure encryption and decryption operations for wallet storage.
//! Uses AES-256-GCM with Argon2id key derivation.

use crate::config;
use crate::errors::{CryptographicError, WalletResult};
use crate::models::{Keystore, Wallet};
use crate::models::keystore::KdfParams;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use argon2::{Algorithm, Argon2, Params, Version};
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use sha2::Sha256;
use std::path::Path;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Cryptographic service for wallet encryption/decryption
pub struct CryptoService;

impl CryptoService {
    /// Encrypt wallet data and create keystore
    pub fn encrypt_wallet(
        wallet: &Wallet,
        password: &str,
        use_argon2: bool,
    ) -> WalletResult<Keystore> {
        // Serialize wallet data
        let wallet_data = serde_json::to_vec(wallet).map_err(|e| {
            CryptographicError::KdfFailed {
                details: format!("Wallet serialization failed: {}", e),
            }
        })?;

        // Generate random salt and nonce
        let mut salt = vec![0u8; config::crypto::SALT_LENGTH];
        let mut nonce_bytes = vec![0u8; config::crypto::NONCE_LENGTH];

        rand::thread_rng().fill_bytes(&mut salt);
        rand::thread_rng().fill_bytes(&mut nonce_bytes);

        // Derive encryption key
        let mut key_bytes = vec![0u8; config::crypto::KEY_LENGTH];
        let kdf_params = if use_argon2 {
            let (memory, iterations, parallelism) = config::get_argon2_config(false);

            Self::derive_key_argon2(
                password.as_bytes(),
                &salt,
                memory,
                iterations,
                parallelism,
                &mut key_bytes,
            )?;

            KdfParams::Argon2 {
                dklen: config::crypto::KEY_LENGTH as u32,
                memory,
                time: iterations,
                parallelism,
                salt: hex::encode(&salt),
            }
        } else {
            const PBKDF2_ITERATIONS: u32 = 100_000;

            pbkdf2_hmac::<Sha256>(
                password.as_bytes(),
                &salt,
                PBKDF2_ITERATIONS,
                &mut key_bytes,
            );

            KdfParams::Pbkdf2 {
                dklen: config::crypto::KEY_LENGTH as u32,
                c: PBKDF2_ITERATIONS,
                prf: "hmac-sha256".to_string(),
                salt: hex::encode(&salt),
            }
        };

        // Create AES-GCM cipher
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt wallet data
        let ciphertext = cipher.encrypt(nonce, wallet_data.as_ref()).map_err(|e| {
            CryptographicError::KdfFailed {
                details: format!("Encryption failed: {}", e),
            }
        })?;

        // Compute MAC over ciphertext + nonce
        let mac = Self::compute_mac(&key_bytes, &ciphertext, &nonce_bytes)?;

        // Clear sensitive data
        key_bytes.zeroize();

        // Create keystore
        Ok(Keystore::new(
            wallet.alias().map(|s| s.to_string()),
            wallet.address().to_string(),
            wallet.network().to_string(),
            ciphertext,
            salt,
            nonce_bytes,
            mac,
            kdf_params,
        ))
    }

    /// Decrypt keystore and restore wallet
    pub fn decrypt_wallet(keystore: &Keystore, password: &str) -> WalletResult<Wallet> {
        // Validate keystore
        keystore.validate()?;

        // Extract cryptographic data
        let ciphertext = keystore.encrypted_data()?;
        let salt = keystore.salt()?;
        let nonce = keystore.nonce()?;
        let stored_mac = keystore.mac()?;

        // Derive decryption key
        let mut key_bytes = vec![0u8; config::crypto::KEY_LENGTH];

        match keystore.kdf_params() {
            KdfParams::Argon2 { memory, time, parallelism, .. } => {
                Self::derive_key_argon2(
                    password.as_bytes(),
                    &salt,
                    *memory,
                    *time,
                    *parallelism,
                    &mut key_bytes,
                )?;
            }
            KdfParams::Pbkdf2 { c, .. } => {
                pbkdf2_hmac::<Sha256>(
                    password.as_bytes(),
                    &salt,
                    *c,
                    &mut key_bytes,
                );
            }
        }

        // Verify MAC
        let computed_mac = Self::compute_mac(&key_bytes, &ciphertext, &nonce)?;
        if computed_mac != stored_mac {
            return Err(CryptographicError::DecryptionFailed {
                context: "MAC verification failed - wrong password or corrupted data".to_string(),
            }
            .into());
        }

        // Decrypt wallet data
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);
        let nonce_array = Nonce::from_slice(&nonce);

        let plaintext = cipher.decrypt(nonce_array, ciphertext.as_ref()).map_err(|e| {
            CryptographicError::DecryptionFailed {
                context: format!("Decryption failed: {}", e),
            }
        })?;

        // Clear sensitive data
        key_bytes.zeroize();

        // Deserialize wallet
        let wallet: Wallet = serde_json::from_slice(&plaintext).map_err(|e| {
            CryptographicError::DataCorruption {
                details: format!("Wallet deserialization failed: {}", e),
            }
        })?;

        // Validate restored wallet
        wallet.validate()?;

        Ok(wallet)
    }

    /// Save encrypted keystore to file
    pub async fn save_keystore(keystore: &Keystore, path: &Path) -> WalletResult<()> {
        // Validate file path
        crate::utils::validate_file_path(path)?;

        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await.map_err(|e| {
                crate::errors::FileSystemError::DirectoryNotAccessible {
                    path: parent.display().to_string(),
                    details: e.to_string(),
                }
            })?;
        }

        // Check if file already exists
        if path.exists() {
            return Err(crate::errors::FileSystemError::FileExists {
                path: path.display().to_string(),
                suggestion: "Use --force to overwrite or choose a different filename".to_string(),
            }
            .into());
        }

        // Serialize keystore to JSON
        let json_data = keystore.to_json()?;

        // Write to file with secure permissions
        tokio::fs::write(path, json_data).await.map_err(|e| {
            crate::errors::FileSystemError::PermissionDenied {
                path: path.display().to_string(),
                operation: format!("write: {}", e),
            }
        })?;

        // Set secure file permissions (Unix-like systems)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = std::fs::Permissions::from_mode(config::fs::KEYSTORE_FILE_PERMISSIONS);
            std::fs::set_permissions(path, permissions).map_err(|e| {
                crate::errors::FileSystemError::PermissionDenied {
                    path: path.display().to_string(),
                    operation: format!("set_permissions: {}", e),
                }
            })?;
        }

        Ok(())
    }

    /// Load keystore from file
    pub async fn load_keystore(path: &Path) -> WalletResult<Keystore> {
        // Validate file path
        crate::utils::validate_file_path(path)?;

        // Check if file exists
        if !path.exists() {
            return Err(crate::errors::FileSystemError::FileNotFound {
                path: path.display().to_string(),
                directory: path.parent()
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| ".".to_string()),
            }
            .into());
        }

        // Read file contents
        let json_data = tokio::fs::read_to_string(path).await.map_err(|e| {
            crate::errors::FileSystemError::PermissionDenied {
                path: path.display().to_string(),
                operation: format!("read: {}", e),
            }
        })?;

        // Check file size limit
        if json_data.len() > config::fs::MAX_KEYSTORE_SIZE as usize {
            return Err(crate::errors::FileSystemError::InvalidFormat {
                path: path.display().to_string(),
                details: format!(
                    "File too large: {} bytes (max: {} bytes)",
                    json_data.len(),
                    config::fs::MAX_KEYSTORE_SIZE
                ),
            }
            .into());
        }

        // Parse and validate keystore
        Keystore::from_json(&json_data)
    }

    /// Derive key using Argon2id
    fn derive_key_argon2(
        password: &[u8],
        salt: &[u8],
        memory: u32,
        iterations: u32,
        parallelism: u32,
        output: &mut [u8],
    ) -> WalletResult<()> {
        let params = Params::new(memory, iterations, parallelism, Some(output.len()))
            .map_err(|e| CryptographicError::KdfFailed {
                details: format!("Invalid Argon2 parameters: {}", e),
            })?;

        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        argon2.hash_password_into(password, salt, output).map_err(|e| {
            CryptographicError::KdfFailed {
                details: format!("Argon2 key derivation failed: {}", e),
            }
        })?;

        Ok(())
    }

    /// Compute MAC over ciphertext and nonce
    fn compute_mac(key: &[u8], ciphertext: &[u8], nonce: &[u8]) -> WalletResult<Vec<u8>> {
        use hmac::{Hmac, Mac};

        let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(key).map_err(|e| {
            CryptographicError::KdfFailed {
                details: format!("HMAC key setup failed: {}", e),
            }
        })?;

        mac.update(ciphertext);
        mac.update(nonce);

        Ok(mac.finalize().into_bytes().to_vec())
    }

    /// Validate password strength
    pub fn validate_password(password: &str) -> WalletResult<()> {
        let mut requirements = Vec::new();

        if password.len() < config::crypto::MIN_PASSWORD_LENGTH {
            requirements.push(format!("At least {} characters", config::crypto::MIN_PASSWORD_LENGTH));
        }

        if password.len() > config::crypto::MAX_PASSWORD_LENGTH {
            requirements.push(format!("At most {} characters", config::crypto::MAX_PASSWORD_LENGTH));
        }

        if !password.chars().any(|c| c.is_ascii_lowercase()) {
            requirements.push("At least one lowercase letter".to_string());
        }

        if !password.chars().any(|c| c.is_ascii_uppercase()) {
            requirements.push("At least one uppercase letter".to_string());
        }

        if !password.chars().any(|c| c.is_ascii_digit()) {
            requirements.push("At least one digit".to_string());
        }

        if !password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)) {
            requirements.push("At least one special character".to_string());
        }

        if !requirements.is_empty() {
            return Err(crate::errors::AuthenticationError::WeakPassword {
                requirements,
            }
            .into());
        }

        Ok(())
    }

    /// Generate secure random password
    pub fn generate_password(length: usize) -> String {
        use rand::seq::SliceRandom;

        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*";
        let mut rng = rand::thread_rng();

        (0..length)
            .map(|_| *CHARS.choose(&mut rng).unwrap() as char)
            .collect()
    }
}

/// Secure string that clears memory on drop
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct SecureString {
    data: String,
}

impl SecureString {
    /// Create new secure string
    pub fn new(data: String) -> Self {
        Self { data }
    }

    /// Get string reference
    pub fn as_str(&self) -> &str {
        &self.data
    }

    /// Get string length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl From<String> for SecureString {
    fn from(data: String) -> Self {
        Self::new(data)
    }
}

impl From<&str> for SecureString {
    fn from(data: &str) -> Self {
        Self::new(data.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Wallet;

    #[test]
    fn test_password_validation() {
        // Valid password
        assert!(CryptoService::validate_password("TestPass123!").is_ok());

        // Too short
        assert!(CryptoService::validate_password("Test1!").is_err());

        // Missing uppercase
        assert!(CryptoService::validate_password("testpass123!").is_err());

        // Missing lowercase
        assert!(CryptoService::validate_password("TESTPASS123!").is_err());

        // Missing digit
        assert!(CryptoService::validate_password("TestPass!").is_err());

        // Missing special character
        assert!(CryptoService::validate_password("TestPass123").is_err());
    }

    #[tokio::test]
    async fn test_wallet_encryption_decryption() {
        let wallet = Wallet::generate(12, "mainnet", Some("test".to_string())).unwrap();
        let password = "TestPassword123!";

        // Encrypt wallet
        let keystore = CryptoService::encrypt_wallet(&wallet, password, true).unwrap();

        // Validate keystore
        assert!(keystore.validate().is_ok());

        // Decrypt wallet
        let restored_wallet = CryptoService::decrypt_wallet(&keystore, password).unwrap();

        // Verify data integrity
        assert_eq!(wallet.address(), restored_wallet.address());
        assert_eq!(wallet.mnemonic(), restored_wallet.mnemonic());
        assert_eq!(wallet.network(), restored_wallet.network());
        assert_eq!(wallet.alias(), restored_wallet.alias());
    }

    #[tokio::test]
    async fn test_wrong_password_decryption() {
        let wallet = Wallet::generate(12, "mainnet", None).unwrap();
        let password = "TestPassword123!";
        let wrong_password = "WrongPassword123!";

        // Encrypt with correct password
        let keystore = CryptoService::encrypt_wallet(&wallet, password, true).unwrap();

        // Try to decrypt with wrong password
        let result = CryptoService::decrypt_wallet(&keystore, wrong_password);
        assert!(result.is_err());
    }

    #[test]
    fn test_password_generation() {
        let password = CryptoService::generate_password(16);
        assert_eq!(password.len(), 16);
        assert!(CryptoService::validate_password(&password).is_ok());
    }

    #[test]
    fn test_secure_string() {
        let secure = SecureString::new("sensitive_data".to_string());
        assert_eq!(secure.as_str(), "sensitive_data");
        assert_eq!(secure.len(), 14);
        assert!(!secure.is_empty());
    }
}