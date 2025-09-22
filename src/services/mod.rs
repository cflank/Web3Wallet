//! # Services
//!
//! Business logic and service layer for wallet operations.
//! All services implement secure patterns with proper error handling.

pub mod mnemonic;
pub mod wallet_manager;

// Re-export main service
pub use wallet_manager::WalletManager;