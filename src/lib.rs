//! # Web3 Wallet CLI Library
//!
//! A secure, professional-grade Web3 wallet CLI tool for Ethereum address generation and management.
//! This library provides the core functionality for creating, importing, and managing Ethereum wallets
//! with BIP39/BIP44 compliance and MetaMask compatibility.
//!
//! ## Features
//!
//! - **Security-First**: Industry-standard cryptography with AES-256-GCM encryption
//! - **HD Wallet Support**: BIP44 derivation paths with MetaMask compatibility
//! - **Memory Safety**: Secure memory cleanup using zeroize
//! - **Error Handling**: Comprehensive Result-based error handling
//! - **Performance**: <1s response time with <50MB memory usage
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use web3wallet_cli::{WalletManager, WalletConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = WalletConfig::default();
//!     let manager = WalletManager::new(config);
//!
//!     // Create a new wallet
//!     let wallet = manager.create_wallet(24).await?;
//!     println!("Address: {}", wallet.address());
//!
//!     Ok(())
//! }
//! ```

#![deny(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]

pub mod cli;
pub mod config;
pub mod errors;
pub mod models;
pub mod services;
pub mod utils;

// Re-export main types for convenience
pub use errors::{WalletError, WalletResult};
pub use models::{Address, Keystore, Wallet};
pub use services::WalletManager;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default wallet configuration
#[derive(Debug, Clone)]
pub struct WalletConfig {
    /// Default network to use
    pub network: String,
    /// Default wallet directory
    pub wallet_dir: std::path::PathBuf,
    /// KDF iteration count for Argon2id
    pub kdf_iterations: u32,
    /// Memory usage for Argon2id (in KB)
    pub kdf_memory: u32,
    /// Parallelism for Argon2id
    pub kdf_parallelism: u32,
}

impl Default for WalletConfig {
    fn default() -> Self {
        Self {
            network: "mainnet".to_string(),
            wallet_dir: dirs::home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join(".web3wallet"),
            kdf_iterations: 1,
            kdf_memory: 47_104, // 46 MiB
            kdf_parallelism: 1,
        }
    }
}


/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, WalletError>;