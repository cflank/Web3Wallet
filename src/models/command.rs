//! # Command Model
//!
//! CLI command structures and validation logic.

use crate::config;
use crate::errors::{UserInputError, WalletResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutputFormat {
    /// Human-readable table format
    Table,
    /// Machine-readable JSON format
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = UserInputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "table" => Ok(OutputFormat::Table),
            "json" => Ok(OutputFormat::Json),
            _ => Err(UserInputError::UnsupportedFormat {
                format: s.to_string(),
                supported: vec!["table".to_string(), "json".to_string()],
            }),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Table => write!(f, "table"),
            OutputFormat::Json => write!(f, "json"),
        }
    }
}

/// Base command structure with common options
#[derive(Debug, Clone)]
pub struct BaseCommand {
    /// Output format
    pub output: OutputFormat,
    /// Verbose logging
    pub verbose: bool,
    /// Configuration file path
    pub config: Option<PathBuf>,
}

impl Default for BaseCommand {
    fn default() -> Self {
        Self {
            output: OutputFormat::Table,
            verbose: false,
            config: None,
        }
    }
}

/// Wallet creation command
#[derive(Debug, Clone)]
pub struct CreateCommand {
    /// Base command options
    pub base: BaseCommand,
    /// Number of mnemonic words (12 or 24)
    pub words: u8,
    /// Save wallet to file
    pub save: Option<String>,
    /// Target network
    pub network: String,
}

impl CreateCommand {
    /// Create with defaults
    pub fn new() -> Self {
        Self {
            base: BaseCommand::default(),
            words: config::bip39::DEFAULT_WORD_COUNT,
            save: None,
            network: config::DEFAULT_NETWORK.to_string(),
        }
    }

    /// Validate command parameters
    pub fn validate(&self) -> WalletResult<()> {
        // Validate word count
        if !config::is_supported_word_count(self.words) {
            return Err(UserInputError::InvalidParameters {
                parameter: "words".to_string(),
                value: self.words.to_string(),
                expected: "12 or 24".to_string(),
            }
            .into());
        }

        // Validate network
        if !config::is_supported_network(&self.network) {
            return Err(UserInputError::InvalidNetwork {
                network: self.network.clone(),
                supported: config::SUPPORTED_NETWORKS.iter().map(|s| s.to_string()).collect(),
            }
            .into());
        }

        // Validate save path if provided
        if let Some(ref save_path) = self.save {
            crate::utils::validate_file_path(save_path)?;
        }

        Ok(())
    }
}

/// Wallet import command
#[derive(Debug, Clone)]
pub struct ImportCommand {
    /// Base command options
    pub base: BaseCommand,
    /// BIP39 mnemonic phrase
    pub mnemonic: Option<String>,
    /// Private key (hex format)
    pub private_key: Option<String>,
    /// Save wallet to file
    pub save: Option<String>,
    /// Target network
    pub network: String,
}

impl ImportCommand {
    /// Create with defaults
    pub fn new() -> Self {
        Self {
            base: BaseCommand::default(),
            mnemonic: None,
            private_key: None,
            save: None,
            network: config::DEFAULT_NETWORK.to_string(),
        }
    }

    /// Validate command parameters
    pub fn validate(&self) -> WalletResult<()> {
        // Validate that exactly one import source is provided
        match (&self.mnemonic, &self.private_key) {
            (Some(_), Some(_)) => {
                return Err(UserInputError::ConflictingOptions {
                    option1: "mnemonic".to_string(),
                    option2: "private-key".to_string(),
                    suggestion: "Use either mnemonic or private key, not both".to_string(),
                }
                .into())
            }
            (None, None) => {
                return Err(UserInputError::MissingParameter {
                    parameter: "mnemonic or private-key".to_string(),
                    hint: "Provide either --mnemonic or --private-key".to_string(),
                }
                .into())
            }
            _ => {} // Valid: exactly one option provided
        }

        // Validate mnemonic if provided
        if let Some(ref mnemonic) = self.mnemonic {
            let word_count = mnemonic.split_whitespace().count();
            if !config::is_supported_word_count(word_count as u8) {
                return Err(UserInputError::InvalidParameters {
                    parameter: "mnemonic".to_string(),
                    value: format!("{} words", word_count),
                    expected: "12 or 24 words".to_string(),
                }
                .into());
            }
        }

        // Validate private key if provided
        if let Some(ref private_key) = self.private_key {
            crate::utils::validate_private_key(private_key)?;
        }

        // Validate network
        if !config::is_supported_network(&self.network) {
            return Err(UserInputError::InvalidNetwork {
                network: self.network.clone(),
                supported: config::SUPPORTED_NETWORKS.iter().map(|s| s.to_string()).collect(),
            }
            .into());
        }

        // Validate save path if provided
        if let Some(ref save_path) = self.save {
            crate::utils::validate_file_path(save_path)?;
        }

        Ok(())
    }
}

/// Wallet load command
#[derive(Debug, Clone)]
pub struct LoadCommand {
    /// Base command options
    pub base: BaseCommand,
    /// Wallet file path
    pub filename: String,
    /// Show only address without decrypting private data
    pub address_only: bool,
    /// Derive specific address index
    pub derive: Option<u32>,
}

impl LoadCommand {
    /// Create with defaults
    pub fn new(filename: String) -> Self {
        Self {
            base: BaseCommand::default(),
            filename,
            address_only: false,
            derive: None,
        }
    }

    /// Validate command parameters
    pub fn validate(&self) -> WalletResult<()> {
        // Validate file path
        crate::utils::validate_file_path(&self.filename)?;

        // Validate derivation index if provided
        if let Some(derive_index) = self.derive {
            // Reasonable upper limit for derivation index
            const MAX_DERIVATION_INDEX: u32 = 2_147_483_647; // 2^31 - 1
            if derive_index > MAX_DERIVATION_INDEX {
                return Err(UserInputError::ValueOutOfRange {
                    parameter: "derive".to_string(),
                    value: derive_index.to_string(),
                    range: format!("0 to {}", MAX_DERIVATION_INDEX),
                }
                .into());
            }
        }

        Ok(())
    }
}

/// Wallet list command
#[derive(Debug, Clone)]
pub struct ListCommand {
    /// Base command options
    pub base: BaseCommand,
    /// Custom wallet directory
    pub path: Option<PathBuf>,
}

impl ListCommand {
    /// Create with defaults
    pub fn new() -> Self {
        Self {
            base: BaseCommand::default(),
            path: None,
        }
    }

    /// Validate command parameters
    pub fn validate(&self) -> WalletResult<()> {
        // Validate custom path if provided
        if let Some(ref path) = self.path {
            crate::utils::validate_file_path(path)?;
        }

        Ok(())
    }
}

/// Address derivation command
#[derive(Debug, Clone)]
pub struct DeriveCommand {
    /// Base command options
    pub base: BaseCommand,
    /// HD derivation path or index
    pub path: String,
    /// Source wallet file
    pub from_file: Option<String>,
    /// Number of addresses to derive
    pub count: u32,
    /// Starting index for derivation
    pub start_index: u32,
}

impl DeriveCommand {
    /// Create with defaults
    pub fn new(path: String) -> Self {
        Self {
            base: BaseCommand::default(),
            path,
            from_file: None,
            count: 1,
            start_index: 0,
        }
    }

    /// Validate command parameters
    pub fn validate(&self) -> WalletResult<()> {
        // Validate derivation path or parse index
        if self.path.starts_with("m/") {
            // Full derivation path
            crate::utils::validate_derivation_path(&self.path)?;
        } else {
            // Should be a numeric index
            self.path.parse::<u32>().map_err(|_| {
                UserInputError::InvalidParameters {
                    parameter: "path".to_string(),
                    value: self.path.clone(),
                    expected: "derivation path (m/44'/60'/0'/0/x) or numeric index".to_string(),
                }
            })?;
        }

        // Validate source file if provided
        if let Some(ref from_file) = self.from_file {
            crate::utils::validate_file_path(from_file)?;
        }

        // Validate count
        if self.count == 0 {
            return Err(UserInputError::ValueOutOfRange {
                parameter: "count".to_string(),
                value: self.count.to_string(),
                range: "1 to 1000".to_string(),
            }
            .into());
        }

        if self.count > 1000 {
            return Err(UserInputError::ValueOutOfRange {
                parameter: "count".to_string(),
                value: self.count.to_string(),
                range: "1 to 1000".to_string(),
            }
            .into());
        }

        // Validate start index
        const MAX_INDEX: u32 = 2_147_483_647;
        if self.start_index > MAX_INDEX {
            return Err(UserInputError::ValueOutOfRange {
                parameter: "start-index".to_string(),
                value: self.start_index.to_string(),
                range: format!("0 to {}", MAX_INDEX),
            }
            .into());
        }

        // Check for overflow
        if self.start_index.saturating_add(self.count) > MAX_INDEX {
            return Err(UserInputError::ValueOutOfRange {
                parameter: "start-index + count".to_string(),
                value: format!("{} + {}", self.start_index, self.count),
                range: format!("total must not exceed {}", MAX_INDEX),
            }
            .into());
        }

        Ok(())
    }

    /// Get the full derivation path for a specific index
    pub fn derivation_path_for_index(&self, index: u32) -> String {
        if self.path.starts_with("m/") {
            // Replace the last component with the new index
            if let Some(last_slash) = self.path.rfind('/') {
                format!("{}/{}", &self.path[..last_slash], index)
            } else {
                format!("{}/{}", self.path, index)
            }
        } else {
            // Use default path with index
            format!("{}/{}", config::DEFAULT_DERIVATION_PATH, index)
        }
    }
}

/// Command execution result
#[derive(Debug, Clone, Serialize)]
pub struct CommandResult<T> {
    /// Operation success status
    pub success: bool,
    /// Result data (if successful)
    pub data: Option<T>,
    /// Error information (if failed)
    pub error: Option<CommandError>,
}

/// Command error information for JSON output
#[derive(Debug, Clone, Serialize)]
pub struct CommandError {
    /// Error code
    pub code: String,
    /// Human-readable message
    pub message: String,
    /// Additional error details
    pub details: Option<serde_json::Value>,
}

impl<T> CommandResult<T> {
    /// Create successful result
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// Create error result
    pub fn error(code: String, message: String, details: Option<serde_json::Value>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(CommandError {
                code,
                message,
                details,
            }),
        }
    }

    /// Create error result from WalletError
    pub fn from_error(error: crate::WalletError) -> Self {
        Self::error(
            error.code().to_string(),
            error.to_string(),
            error.suggestion().map(|s| serde_json::Value::String(s)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_format_parsing() {
        assert_eq!("table".parse::<OutputFormat>().unwrap(), OutputFormat::Table);
        assert_eq!("json".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
        assert!("invalid".parse::<OutputFormat>().is_err());
    }

    #[test]
    fn test_create_command_validation() {
        let mut cmd = CreateCommand::new();
        assert!(cmd.validate().is_ok());

        cmd.words = 16; // Invalid
        assert!(cmd.validate().is_err());

        cmd.words = 12; // Valid again
        cmd.network = "invalid".to_string();
        assert!(cmd.validate().is_err());
    }

    #[test]
    fn test_import_command_validation() {
        let mut cmd = ImportCommand::new();
        assert!(cmd.validate().is_err()); // No import source

        cmd.mnemonic = Some("abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about".to_string());
        assert!(cmd.validate().is_ok());

        cmd.private_key = Some("0x1234567890123456789012345678901234567890123456789012345678901234".to_string());
        assert!(cmd.validate().is_err()); // Both sources provided
    }

    #[test]
    fn test_derive_command_validation() {
        let mut cmd = DeriveCommand::new("m/44'/60'/0'/0/0".to_string());
        assert!(cmd.validate().is_ok());

        cmd.path = "5".to_string(); // Numeric index
        assert!(cmd.validate().is_ok());

        cmd.path = "invalid".to_string();
        assert!(cmd.validate().is_err());

        cmd.path = "0".to_string();
        cmd.count = 0; // Invalid
        assert!(cmd.validate().is_err());
    }

    #[test]
    fn test_derive_path_generation() {
        let cmd = DeriveCommand::new("m/44'/60'/0'/0/0".to_string());
        assert_eq!(cmd.derivation_path_for_index(5), "m/44'/60'/0'/0/5");

        let cmd = DeriveCommand::new("5".to_string());
        assert_eq!(cmd.derivation_path_for_index(5), "m/44'/60'/0'/0/5");
    }

    #[test]
    fn test_command_result() {
        let success = CommandResult::success("test data");
        assert!(success.success);
        assert_eq!(success.data.unwrap(), "test data");

        let error = CommandResult::<String>::error(
            "TEST_001".to_string(),
            "Test error".to_string(),
            None,
        );
        assert!(!error.success);
        assert_eq!(error.error.unwrap().code, "TEST_001");
    }
}