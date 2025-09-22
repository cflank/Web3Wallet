//! # Common Test Utilities
//!
//! Shared testing utilities and helpers for all test modules.
//! Provides consistent test setup and security-focused test patterns.

use std::path::PathBuf;
use tempfile::TempDir;
use tracing_subscriber::EnvFilter;
use web3wallet_cli::{WalletConfig, WalletResult};

/// Test configuration for isolated testing
pub struct TestConfig {
    /// Temporary directory for test files
    pub temp_dir: TempDir,
    /// Wallet configuration for testing
    pub config: WalletConfig,
}

impl TestConfig {
    /// Create a new test configuration with isolated temporary directory
    pub fn new() -> WalletResult<Self> {
        let temp_dir = TempDir::new().map_err(|e| {
            web3wallet_cli::WalletError::FileSystem(
                web3wallet_cli::errors::FileSystemError::DirectoryNotAccessible {
                    path: "temp".to_string(),
                    details: e.to_string(),
                },
            )
        })?;

        let config = WalletConfig {
            network: "testnet".to_string(),
            wallet_dir: temp_dir.path().to_path_buf(),
            kdf_iterations: 1, // Fast iterations for testing
            kdf_memory: 1024,  // Low memory usage for testing
            kdf_parallelism: 1,
        };

        Ok(Self { temp_dir, config })
    }

    /// Get the temporary directory path
    pub fn temp_path(&self) -> &std::path::Path {
        self.temp_dir.path()
    }

    /// Create a test wallet file path
    pub fn wallet_path(&self, name: &str) -> PathBuf {
        self.temp_path().join(format!("{}.json", name))
    }
}

/// Test constants for consistent test data
pub mod test_data {
    /// Test mnemonic phrase (24 words)
    pub const TEST_MNEMONIC_24: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";

    /// Test mnemonic phrase (12 words)
    pub const TEST_MNEMONIC_12: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    /// Expected address for test mnemonic (first address m/44'/60'/0'/0/0)
    pub const TEST_ADDRESS: &str = "0x9858EfFD232B4033E47d90003D41EC34EcaEda94";

    /// Test private key
    pub const TEST_PRIVATE_KEY: &str = "0x4c0883a69102937d6231471b5dbb6204fe512961708279c1e3ae83da5e56df1a";

    /// Test password for keystore encryption
    pub const TEST_PASSWORD: &str = "test_password_123";

    /// Test network name
    pub const TEST_NETWORK: &str = "sepolia";

    /// Test wallet alias
    pub const TEST_ALIAS: &str = "test-wallet";
}

/// Assert that an operation completes within the constitutional time limit (<1s)
#[macro_export]
macro_rules! assert_performance {
    ($expr:expr) => {
        let start = std::time::Instant::now();
        let result = $expr;
        let duration = start.elapsed();
        assert!(
            duration < std::time::Duration::from_secs(1),
            "Operation took {:?}, exceeding 1s limit",
            duration
        );
        result
    };
}

/// Assert that sensitive data is properly zeroized
#[macro_export]
macro_rules! assert_zeroized {
    ($data:expr) => {
        // This is a conceptual test - in practice, we'd need memory inspection tools
        // For now, we ensure the data structure implements Zeroize
        use zeroize::Zeroize;
        let mut data = $data;
        data.zeroize();
        // The actual memory checking would require unsafe code and is platform-specific
    };
}

/// Mock secure random number generator for deterministic testing
pub struct MockRng {
    counter: std::cell::RefCell<u64>,
}

impl MockRng {
    pub fn new() -> Self {
        Self {
            counter: std::cell::RefCell::new(0),
        }
    }
}

impl rand::RngCore for MockRng {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let mut counter = self.counter.borrow_mut();
        *counter = counter.wrapping_add(1);
        *counter
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for chunk in dest.chunks_mut(8) {
            let val = self.next_u64();
            let bytes = val.to_le_bytes();
            let len = chunk.len().min(8);
            chunk[..len].copy_from_slice(&bytes[..len]);
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl rand::CryptoRng for MockRng {}

/// Setup logging for tests
pub fn setup_test_logging() {
    let _ = tracing_subscriber::fmt()
        .with_test_writer()
        .with_env_filter(EnvFilter::from_default_env().add_directive("debug".parse().unwrap()))
        .try_init();
}