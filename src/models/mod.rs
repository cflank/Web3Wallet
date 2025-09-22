//! # Data Models
//!
//! Core data structures for the Web3 wallet CLI tool.
//! All models follow the constitutional requirements for type safety and validation.

pub mod address;
pub mod command;
pub mod keystore;
pub mod wallet;

// Re-export main types
pub use address::Address;
pub use command::{CommandResult, OutputFormat};
pub use keystore::Keystore;
pub use wallet::Wallet;