//! # Utility Functions
//!
//! Common utility functions and helpers used throughout the application.
//! All utilities follow security-first principles with proper validation.

use crate::errors::{ValidationError, WalletResult};
use std::path::Path;

/// Validate Ethereum address format
pub fn validate_ethereum_address(address: &str) -> WalletResult<()> {
    // Remove 0x prefix if present
    let addr = address.strip_prefix("0x").unwrap_or(address);

    // Check length (40 hex characters)
    if addr.len() != 40 {
        return Err(ValidationError::InvalidAddressFormat {
            address: address.to_string(),
            expected: "40 hex characters (with or without 0x prefix)".to_string(),
        }
        .into());
    }

    // Check if all characters are valid hex
    if !addr.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ValidationError::InvalidAddressFormat {
            address: address.to_string(),
            expected: "hexadecimal characters only".to_string(),
        }
        .into());
    }

    Ok(())
}

/// Validate private key format
pub fn validate_private_key(private_key: &str) -> WalletResult<()> {
    // Remove 0x prefix if present
    let key = private_key.strip_prefix("0x").unwrap_or(private_key);

    // Check length (64 hex characters)
    if key.len() != 64 {
        return Err(ValidationError::InvalidAddressFormat {
            address: private_key.to_string(),
            expected: "64 hex characters (with or without 0x prefix)".to_string(),
        }
        .into());
    }

    // Check if all characters are valid hex
    if !key.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ValidationError::InvalidAddressFormat {
            address: private_key.to_string(),
            expected: "hexadecimal characters only".to_string(),
        }
        .into());
    }

    Ok(())
}

/// Validate HD derivation path format
pub fn validate_derivation_path(path: &str) -> WalletResult<()> {
    // Check if path starts with m/
    if !path.starts_with("m/") {
        return Err(ValidationError::InvalidAddressFormat {
            address: path.to_string(),
            expected: "path starting with 'm/'".to_string(),
        }
        .into());
    }

    // Split and validate each component
    let components: Vec<&str> = path[2..].split('/').collect();
    for component in components {
        if component.is_empty() {
            return Err(ValidationError::InvalidAddressFormat {
                address: path.to_string(),
                expected: "non-empty path components".to_string(),
            }
            .into());
        }

        // Check for hardened derivation (')
        let (num_str, _) = if component.ends_with('\'') {
            (&component[..component.len() - 1], true)
        } else {
            (component, false)
        };

        // Validate that component is a number
        if num_str.parse::<u32>().is_err() {
            return Err(ValidationError::InvalidAddressFormat {
                address: path.to_string(),
                expected: "numeric path components".to_string(),
            }
            .into());
        }
    }

    Ok(())
}

/// Validate file path for security (prevent path traversal)
pub fn validate_file_path<P: AsRef<Path>>(path: P) -> WalletResult<()> {
    let path = path.as_ref();

    // Check for path traversal attempts
    for component in path.components() {
        if let std::path::Component::ParentDir = component {
            return Err(crate::errors::FileSystemError::PathTraversal {
                path: path.display().to_string(),
            }
            .into());
        }
    }

    Ok(())
}

/// Sanitize filename to prevent invalid characters
pub fn sanitize_filename(filename: &str) -> String {
    // Remove path separators and collect only alphanumeric and safe characters
    filename
        .chars()
        .filter(|c| c.is_alphanumeric() || matches!(*c, '-' | '_'))
        .collect::<String>()
        .trim_start_matches('.')
        .to_string()
}

/// Format duration for human-readable display
pub fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m{}s", secs / 60, secs % 60)
    } else {
        format!("{}h{}m", secs / 3600, (secs % 3600) / 60)
    }
}

/// Convert bytes to human-readable size
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_ethereum_address() {
        // Valid addresses
        assert!(validate_ethereum_address("0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99").is_ok());
        assert!(validate_ethereum_address("742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99").is_ok());

        // Invalid addresses
        assert!(validate_ethereum_address("0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a9").is_err()); // Too short
        assert!(validate_ethereum_address("0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99G").is_err()); // Invalid char
        assert!(validate_ethereum_address("").is_err()); // Empty
    }

    #[test]
    fn test_validate_derivation_path() {
        // Valid paths
        assert!(validate_derivation_path("m/44'/60'/0'/0/0").is_ok());
        assert!(validate_derivation_path("m/44'/60'/0'/0").is_ok());
        assert!(validate_derivation_path("m/0").is_ok());

        // Invalid paths
        assert!(validate_derivation_path("44'/60'/0'/0/0").is_err()); // No m/
        assert!(validate_derivation_path("m/44'/60'/'/0/0").is_err()); // Empty component
        assert!(validate_derivation_path("m/44'/60'/a/0/0").is_err()); // Invalid component
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("my-wallet_123"), "my-wallet_123");
        assert_eq!(sanitize_filename("my wallet!@#"), "mywallet");
        assert_eq!(sanitize_filename("../../../etc/passwd"), "etcpasswd");
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
    }
}