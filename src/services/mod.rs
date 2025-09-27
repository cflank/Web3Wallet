//! # Services
//!
//! Business logic and service layer for wallet operations.
//! All services implement secure patterns with proper error handling.

pub mod crypto;
pub mod mnemonic;
pub mod wallet_manager;

// Re-export main services
pub use crypto::CryptoService;
pub use wallet_manager::WalletManager;