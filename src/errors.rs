//! # Error Types
//!
//! Comprehensive error handling for the Web3 wallet CLI tool.
//! All errors follow the constitutional requirement for explicit error handling
//! with Result<T, E> types and user-friendly messages.

use thiserror::Error;

/// Main error type for wallet operations
#[derive(Error, Debug, Clone, PartialEq)]
pub enum WalletError {
    /// Cryptographic operation failures
    #[error("Cryptographic error: {0}")]
    Cryptographic(#[from] CryptographicError),

    /// File system operation failures
    #[error("File system error: {0}")]
    FileSystem(#[from] FileSystemError),

    /// User input validation failures
    #[error("Input validation error: {0}")]
    UserInput(#[from] UserInputError),

    /// Authentication failures
    #[error("Authentication error: {0}")]
    Authentication(#[from] AuthenticationError),

    /// Network operation failures
    #[error("Network error: {0}")]
    Network(#[from] NetworkError),

    /// Data validation failures
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    /// Feature not yet implemented
    #[error("Feature not implemented: {0}")]
    NotImplemented(String),

    /// I/O operation failures
    #[error("I/O error: {0}")]
    Io(String),

    /// JSON serialization/deserialization failures
    #[error("JSON error: {0}")]
    Json(String),
}

/// Cryptographic operation errors (CRYPTO_xxx)
#[derive(Error, Debug, Clone, PartialEq)]
pub enum CryptographicError {
    /// Insufficient entropy for secure key generation
    #[error("CRYPTO_001: Insufficient entropy for secure key generation")]
    InsufficientEntropy {
        /// Available entropy bits
        available: u32,
        /// Required entropy bits
        required: u32,
        /// Suggestion for resolution
        suggestion: String,
    },

    /// Invalid BIP39 mnemonic phrase
    #[error("CRYPTO_002: Invalid BIP39 mnemonic phrase")]
    InvalidMnemonic {
        /// Error details
        details: String,
        /// Suggestion for resolution
        suggestion: String,
    },

    /// Invalid private key format
    #[error("CRYPTO_003: Invalid private key format")]
    InvalidPrivateKey {
        /// Error details
        details: String,
        /// Expected format
        expected: String,
    },

    /// Keystore decryption failed
    #[error("CRYPTO_004: Keystore decryption failed")]
    DecryptionFailed {
        /// Error context
        context: String,
    },

    /// Data corruption detected
    #[error("CRYPTO_005: Data corruption detected during decryption")]
    DataCorruption {
        /// Corruption details
        details: String,
    },

    /// Invalid HD derivation path
    #[error("CRYPTO_006: Invalid HD derivation path")]
    InvalidDerivationPath {
        /// Provided path
        path: String,
        /// Expected format
        expected: String,
    },

    /// Derivation index out of range
    #[error("CRYPTO_007: Derivation index out of valid range")]
    IndexOutOfRange {
        /// Provided index
        index: u32,
        /// Maximum valid index
        max_index: u32,
    },

    /// Key derivation function failed
    #[error("CRYPTO_008: Key derivation function failed")]
    KdfFailed {
        /// Error details
        details: String,
    },

    /// Signature generation failed
    #[error("CRYPTO_009: Signature generation failed")]
    SignatureFailed {
        /// Error details
        details: String,
    },

    /// Address generation failed
    #[error("CRYPTO_010: Address generation failed")]
    AddressGenerationFailed {
        /// Error details
        details: String,
    },
}

/// File system operation errors (FS_xxx)
#[derive(Error, Debug, Clone, PartialEq)]
pub enum FileSystemError {
    /// Permission denied for file operation
    #[error("FS_001: Permission denied for file operation")]
    PermissionDenied {
        /// File path
        path: String,
        /// Operation attempted
        operation: String,
    },

    /// Wallet file not found
    #[error("FS_002: Wallet file not found")]
    FileNotFound {
        /// Requested file path
        path: String,
        /// Search directory
        directory: String,
    },

    /// Directory not accessible
    #[error("FS_003: Directory not accessible")]
    DirectoryNotAccessible {
        /// Directory path
        path: String,
        /// Error details
        details: String,
    },

    /// Insufficient disk space
    #[error("FS_004: Insufficient disk space for operation")]
    InsufficientSpace {
        /// Required space in bytes
        required: u64,
        /// Available space in bytes
        available: u64,
    },

    /// File already exists
    #[error("FS_005: File already exists")]
    FileExists {
        /// File path
        path: String,
        /// Suggestion for resolution
        suggestion: String,
    },

    /// Invalid file format or corruption
    #[error("FS_006: Invalid file format or corruption")]
    InvalidFormat {
        /// File path
        path: String,
        /// Error details
        details: String,
    },

    /// Path traversal security violation
    #[error("FS_007: Path traversal security violation")]
    PathTraversal {
        /// Attempted path
        path: String,
    },

    /// File lock acquisition failed
    #[error("FS_008: File lock acquisition failed")]
    LockFailed {
        /// File path
        path: String,
        /// Timeout duration
        timeout: std::time::Duration,
    },
}

/// User input validation errors (INPUT_xxx)
#[derive(Error, Debug, Clone, PartialEq)]
pub enum UserInputError {
    /// Invalid command parameters
    #[error("INPUT_001: Invalid command parameters")]
    InvalidParameters {
        /// Parameter name
        parameter: String,
        /// Provided value
        value: String,
        /// Expected format
        expected: String,
    },

    /// Conflicting command options
    #[error("INPUT_002: Conflicting command options")]
    ConflictingOptions {
        /// First option
        option1: String,
        /// Second option
        option2: String,
        /// Resolution suggestion
        suggestion: String,
    },

    /// Missing required parameter
    #[error("INPUT_003: Missing required parameter")]
    MissingParameter {
        /// Parameter name
        parameter: String,
        /// Usage hint
        hint: String,
    },

    /// Parameter value out of range
    #[error("INPUT_004: Parameter value out of valid range")]
    ValueOutOfRange {
        /// Parameter name
        parameter: String,
        /// Provided value
        value: String,
        /// Valid range
        range: String,
    },

    /// Unsupported output format
    #[error("INPUT_005: Unsupported output format")]
    UnsupportedFormat {
        /// Requested format
        format: String,
        /// Supported formats
        supported: Vec<String>,
    },

    /// Invalid network specification
    #[error("INPUT_006: Invalid network specification")]
    InvalidNetwork {
        /// Requested network
        network: String,
        /// Supported networks
        supported: Vec<String>,
    },

    /// Password confirmation mismatch
    #[error("INPUT_007: Password confirmation mismatch")]
    PasswordMismatch,

    /// Operation timeout
    #[error("INPUT_008: Operation timeout")]
    Timeout {
        /// Operation name
        operation: String,
        /// Timeout duration
        duration: std::time::Duration,
    },
}

/// Authentication errors (AUTH_xxx)
#[derive(Error, Debug, Clone, PartialEq)]
pub enum AuthenticationError {
    /// Wrong password for wallet decryption
    #[error("AUTH_001: Incorrect password for wallet decryption")]
    WrongPassword {
        /// Wallet file
        wallet_file: String,
        /// Remaining attempts
        attempts_remaining: u32,
    },

    /// Password too weak
    #[error("AUTH_002: Password does not meet minimum requirements")]
    WeakPassword {
        /// Requirements not met
        requirements: Vec<String>,
    },

    /// Maximum authentication attempts exceeded
    #[error("AUTH_003: Maximum authentication attempts exceeded")]
    MaxAttemptsExceeded {
        /// Lockout duration
        lockout_duration: std::time::Duration,
    },

    /// Session timeout
    #[error("AUTH_004: Session timeout")]
    SessionTimeout,

    /// User canceled authentication
    #[error("AUTH_005: User canceled authentication")]
    UserCanceled,
}

/// Network operation errors (NETWORK_xxx)
#[derive(Error, Debug, Clone, PartialEq)]
pub enum NetworkError {
    /// Network connectivity failure
    #[error("NETWORK_001: Network connectivity failure")]
    ConnectivityFailure {
        /// Target endpoint
        endpoint: String,
        /// Error details
        details: String,
    },

    /// Request timeout
    #[error("NETWORK_002: Request timeout")]
    RequestTimeout {
        /// Request type
        request_type: String,
        /// Timeout duration
        timeout: std::time::Duration,
    },

    /// Invalid network configuration
    #[error("NETWORK_003: Invalid network configuration")]
    InvalidConfiguration {
        /// Configuration key
        key: String,
        /// Error details
        details: String,
    },

    /// Rate limiting exceeded
    #[error("NETWORK_004: Rate limiting exceeded")]
    RateLimitExceeded {
        /// Retry after duration
        retry_after: std::time::Duration,
    },

    /// Unsupported network protocol
    #[error("NETWORK_005: Unsupported network protocol")]
    UnsupportedProtocol {
        /// Protocol name
        protocol: String,
        /// Supported protocols
        supported: Vec<String>,
    },
}

/// Data validation errors (VALIDATION_xxx)
#[derive(Error, Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// Address format validation failed
    #[error("VALIDATION_001: Address format validation failed")]
    InvalidAddressFormat {
        /// Provided address
        address: String,
        /// Expected format
        expected: String,
    },

    /// Keystore schema validation failed
    #[error("VALIDATION_002: Keystore schema validation failed")]
    InvalidKeystoreSchema {
        /// Schema error
        error: String,
        /// File path
        file_path: String,
    },

    /// Command syntax validation failed
    #[error("VALIDATION_003: Command syntax validation failed")]
    InvalidCommandSyntax {
        /// Command
        command: String,
        /// Syntax error
        error: String,
    },

    /// Data integrity check failed
    #[error("VALIDATION_004: Data integrity check failed")]
    IntegrityCheckFailed {
        /// Data type
        data_type: String,
        /// Error details
        details: String,
    },

    /// Version compatibility check failed
    #[error("VALIDATION_005: Version compatibility check failed")]
    VersionIncompatible {
        /// Current version
        current: String,
        /// Required version
        required: String,
    },
}

/// Convenient result type alias
pub type WalletResult<T> = Result<T, WalletError>;

impl WalletError {
    /// Get error code for programmatic handling
    pub fn code(&self) -> &'static str {
        match self {
            WalletError::Cryptographic(err) => err.code(),
            WalletError::FileSystem(err) => err.code(),
            WalletError::UserInput(err) => err.code(),
            WalletError::Authentication(err) => err.code(),
            WalletError::Network(err) => err.code(),
            WalletError::Validation(err) => err.code(),
            WalletError::NotImplemented(_) => "NOT_IMPLEMENTED",
            WalletError::Io(_) => "IO_ERROR",
            WalletError::Json(_) => "JSON_ERROR",
        }
    }

    /// Get user-friendly suggestion for error resolution
    pub fn suggestion(&self) -> Option<String> {
        match self {
            WalletError::Cryptographic(err) => err.suggestion(),
            WalletError::FileSystem(err) => err.suggestion(),
            WalletError::UserInput(err) => err.suggestion(),
            WalletError::Authentication(err) => err.suggestion(),
            WalletError::Network(err) => err.suggestion(),
            WalletError::Validation(err) => err.suggestion(),
            WalletError::NotImplemented(_) => Some("This feature is not yet implemented. Please check for updates or contribute to the project.".to_string()),
            WalletError::Io(_) => Some("Check file permissions and disk space.".to_string()),
            WalletError::Json(_) => Some("Verify data format and structure.".to_string()),
        }
    }
}

// Implement suggestion methods for each error type
impl CryptographicError {
    fn code(&self) -> &'static str {
        match self {
            CryptographicError::InsufficientEntropy { .. } => "CRYPTO_001",
            CryptographicError::InvalidMnemonic { .. } => "CRYPTO_002",
            CryptographicError::InvalidPrivateKey { .. } => "CRYPTO_003",
            CryptographicError::DecryptionFailed { .. } => "CRYPTO_004",
            CryptographicError::DataCorruption { .. } => "CRYPTO_005",
            CryptographicError::InvalidDerivationPath { .. } => "CRYPTO_006",
            CryptographicError::IndexOutOfRange { .. } => "CRYPTO_007",
            CryptographicError::KdfFailed { .. } => "CRYPTO_008",
            CryptographicError::SignatureFailed { .. } => "CRYPTO_009",
            CryptographicError::AddressGenerationFailed { .. } => "CRYPTO_010",
        }
    }

    fn suggestion(&self) -> Option<String> {
        match self {
            CryptographicError::InsufficientEntropy { suggestion, .. } => Some(suggestion.clone()),
            CryptographicError::InvalidMnemonic { suggestion, .. } => Some(suggestion.clone()),
            CryptographicError::InvalidPrivateKey { expected, .. } => {
                Some(format!("Expected format: {}", expected))
            }
            _ => None,
        }
    }
}

// Similar implementations for other error types...
macro_rules! impl_error_traits {
    ($error_type:ty, $prefix:expr) => {
        impl $error_type {
            fn code(&self) -> &'static str {
                concat!($prefix, "_001") // Simplified for now
            }

            fn suggestion(&self) -> Option<String> {
                None // Can be expanded for specific suggestions
            }
        }
    };
}

impl_error_traits!(FileSystemError, "FS");
impl_error_traits!(UserInputError, "INPUT");
impl_error_traits!(AuthenticationError, "AUTH");
impl_error_traits!(NetworkError, "NETWORK");
impl_error_traits!(ValidationError, "VALIDATION");

// Implement From traits for standard library errors
impl From<std::io::Error> for WalletError {
    fn from(err: std::io::Error) -> Self {
        WalletError::Io(err.to_string())
    }
}

impl From<serde_json::Error> for WalletError {
    fn from(err: serde_json::Error) -> Self {
        WalletError::Json(err.to_string())
    }
}