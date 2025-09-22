//! # Configuration
//!
//! Application configuration constants and settings.
//! Follows constitutional security and performance requirements.

use std::path::PathBuf;

/// Default HD derivation path for Ethereum (BIP44)
pub const DEFAULT_DERIVATION_PATH: &str = "m/44'/60'/0'/0";

/// Default network name
pub const DEFAULT_NETWORK: &str = "mainnet";

/// Supported networks
pub const SUPPORTED_NETWORKS: &[&str] = &[
    "mainnet",
    "sepolia",
    "goerli", // Legacy testnet
    "holesky",
];

/// Default wallet directory name
pub const DEFAULT_WALLET_DIR: &str = ".web3wallet";

/// Keystore file extension
pub const KEYSTORE_EXTENSION: &str = "json";

/// Performance constraints (from constitution)
pub mod performance {
    use std::time::Duration;

    /// Maximum command response time (constitutional requirement)
    pub const MAX_RESPONSE_TIME: Duration = Duration::from_secs(1);

    /// Maximum memory usage baseline (constitutional requirement)
    pub const MAX_MEMORY_USAGE_MB: u64 = 50;

    /// Default timeout for blockchain operations
    pub const BLOCKCHAIN_OPERATION_TIMEOUT: Duration = Duration::from_secs(30);
}

/// Cryptographic configuration
pub mod crypto {
    /// Default Argon2id configuration (OWASP 2024 compliant)
    pub const DEFAULT_ARGON2_MEMORY: u32 = 47_104; // 46 MiB
    pub const DEFAULT_ARGON2_ITERATIONS: u32 = 1;
    pub const DEFAULT_ARGON2_PARALLELISM: u32 = 1;

    /// Alternative Argon2id configuration for lower memory systems
    pub const LOW_MEMORY_ARGON2_MEMORY: u32 = 19_456; // 19 MiB
    pub const LOW_MEMORY_ARGON2_ITERATIONS: u32 = 2;

    /// Salt length for key derivation
    pub const SALT_LENGTH: usize = 32;

    /// AES-GCM nonce length
    pub const NONCE_LENGTH: usize = 12;

    /// Key length for AES-256
    pub const KEY_LENGTH: usize = 32;

    /// Minimum password length
    pub const MIN_PASSWORD_LENGTH: usize = 8;

    /// Maximum password length
    pub const MAX_PASSWORD_LENGTH: usize = 1024;
}

/// File system configuration
pub mod fs {
    /// Default file permissions for keystore files (owner read/write only)
    pub const KEYSTORE_FILE_PERMISSIONS: u32 = 0o600;

    /// Default directory permissions for wallet directory
    pub const WALLET_DIR_PERMISSIONS: u32 = 0o700;

    /// Maximum keystore file size (to prevent DoS)
    pub const MAX_KEYSTORE_SIZE: u64 = 1024 * 1024; // 1 MB
}

/// BIP39 configuration
pub mod bip39 {
    /// Supported mnemonic word counts
    pub const SUPPORTED_WORD_COUNTS: &[u8] = &[12, 24];

    /// Default word count
    pub const DEFAULT_WORD_COUNT: u8 = 12;

    /// Entropy bits for different word counts
    pub const ENTROPY_BITS_12: usize = 128;
    pub const ENTROPY_BITS_24: usize = 256;
}

/// CLI output configuration
pub mod output {
    /// Table column widths
    pub const ADDRESS_COLUMN_WIDTH: usize = 43; // 0x + 40 hex chars + padding
    pub const ALIAS_COLUMN_WIDTH: usize = 20;
    pub const NETWORK_COLUMN_WIDTH: usize = 15;
    pub const DATE_COLUMN_WIDTH: usize = 20;

    /// JSON indentation
    pub const JSON_INDENT: usize = 2;
}

/// Application metadata
pub mod app {
    /// Application name
    pub const NAME: &str = "Web3 Wallet CLI";

    /// Application description
    pub const DESCRIPTION: &str = "A secure, professional-grade Web3 wallet CLI tool";

    /// Author information
    pub const AUTHORS: &str = "Web3Wallet Team";

    /// Repository URL
    pub const REPOSITORY: &str = "https://github.com/user/web3wallet-cli";

    /// License
    pub const LICENSE: &str = "MIT";
}

/// Get default wallet directory path
pub fn default_wallet_dir() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join(DEFAULT_WALLET_DIR)
}

/// Validate network name
pub fn is_supported_network(network: &str) -> bool {
    SUPPORTED_NETWORKS.contains(&network)
}

/// Get Argon2 configuration based on available memory
pub fn get_argon2_config(use_low_memory: bool) -> (u32, u32, u32) {
    if use_low_memory {
        (
            crypto::LOW_MEMORY_ARGON2_MEMORY,
            crypto::LOW_MEMORY_ARGON2_ITERATIONS,
            crypto::DEFAULT_ARGON2_PARALLELISM,
        )
    } else {
        (
            crypto::DEFAULT_ARGON2_MEMORY,
            crypto::DEFAULT_ARGON2_ITERATIONS,
            crypto::DEFAULT_ARGON2_PARALLELISM,
        )
    }
}

/// Validate word count for mnemonic generation
pub fn is_supported_word_count(count: u8) -> bool {
    bip39::SUPPORTED_WORD_COUNTS.contains(&count)
}

/// Get entropy bits for word count
pub fn entropy_bits_for_word_count(count: u8) -> Option<usize> {
    match count {
        12 => Some(bip39::ENTROPY_BITS_12),
        24 => Some(bip39::ENTROPY_BITS_24),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_networks() {
        assert!(is_supported_network("mainnet"));
        assert!(is_supported_network("sepolia"));
        assert!(!is_supported_network("invalid"));
    }

    #[test]
    fn test_supported_word_counts() {
        assert!(is_supported_word_count(12));
        assert!(is_supported_word_count(24));
        assert!(!is_supported_word_count(16));
    }

    #[test]
    fn test_entropy_calculation() {
        assert_eq!(entropy_bits_for_word_count(12), Some(128));
        assert_eq!(entropy_bits_for_word_count(24), Some(256));
        assert_eq!(entropy_bits_for_word_count(16), None);
    }

    #[test]
    fn test_argon2_config() {
        let (mem, iter, par) = get_argon2_config(false);
        assert_eq!(mem, crypto::DEFAULT_ARGON2_MEMORY);
        assert_eq!(iter, crypto::DEFAULT_ARGON2_ITERATIONS);
        assert_eq!(par, crypto::DEFAULT_ARGON2_PARALLELISM);

        let (mem_low, iter_low, par_low) = get_argon2_config(true);
        assert_eq!(mem_low, crypto::LOW_MEMORY_ARGON2_MEMORY);
        assert_eq!(iter_low, crypto::LOW_MEMORY_ARGON2_ITERATIONS);
        assert_eq!(par_low, crypto::DEFAULT_ARGON2_PARALLELISM);
    }
}