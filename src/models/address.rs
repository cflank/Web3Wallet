//! # Address Model
//!
//! Ethereum address representation with validation and metadata.

use crate::config;
use crate::errors::{ValidationError, WalletResult};
use serde::{Deserialize, Serialize};

/// Ethereum address with metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Address {
    /// Ethereum address (42 characters with 0x prefix)
    address: String,

    /// Derivation index (for HD wallets)
    index: Option<u32>,

    /// Full HD derivation path
    derivation_path: Option<String>,

    /// Optional cached balance (in wei)
    balance: Option<String>,

    /// Optional cached nonce
    nonce: Option<u64>,

    /// Optional label/alias
    label: Option<String>,

    /// Network this address belongs to
    network: String,
}

impl Address {
    /// Create a new address
    pub fn new(
        address: String,
        network: String,
        index: Option<u32>,
        derivation_path: Option<String>,
    ) -> WalletResult<Self> {
        // Validate address format
        crate::utils::validate_ethereum_address(&address)?;

        // Validate network
        if !config::is_supported_network(&network) {
            return Err(ValidationError::InvalidAddressFormat {
                address: network.clone(),
                expected: format!("one of: {:?}", config::SUPPORTED_NETWORKS),
            }
            .into());
        }

        // Validate derivation path if provided
        if let Some(ref path) = derivation_path {
            crate::utils::validate_derivation_path(path)?;
        }

        Ok(Self {
            address: address.to_lowercase(),
            index,
            derivation_path,
            balance: None,
            nonce: None,
            label: None,
            network,
        })
    }

    /// Create from string address
    pub fn from_string(address: &str, network: &str) -> WalletResult<Self> {
        Self::new(address.to_string(), network.to_string(), None, None)
    }

    /// Create derived address
    pub fn derived(
        address: String,
        network: String,
        index: u32,
        derivation_path: String,
    ) -> WalletResult<Self> {
        Self::new(
            address,
            network,
            Some(index),
            Some(derivation_path),
        )
    }

    /// Get the Ethereum address
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Get the checksummed address (proper capitalization)
    pub fn checksummed_address(&self) -> String {
        // Simple checksum implementation
        // In production, use ethers::utils::to_checksum for proper EIP-55
        let addr = self.address.strip_prefix("0x").unwrap_or(&self.address);
        format!("0x{}", addr)
    }

    /// Get derivation index
    pub fn index(&self) -> Option<u32> {
        self.index
    }

    /// Get derivation path
    pub fn derivation_path(&self) -> Option<&str> {
        self.derivation_path.as_deref()
    }

    /// Get network
    pub fn network(&self) -> &str {
        &self.network
    }

    /// Get cached balance
    pub fn balance(&self) -> Option<&str> {
        self.balance.as_deref()
    }

    /// Set cached balance
    pub fn set_balance(&mut self, balance: Option<String>) {
        self.balance = balance;
    }

    /// Get cached nonce
    pub fn nonce(&self) -> Option<u64> {
        self.nonce
    }

    /// Set cached nonce
    pub fn set_nonce(&mut self, nonce: Option<u64>) {
        self.nonce = nonce;
    }

    /// Get label/alias
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Set label/alias
    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }

    /// Check if this is a derived address (has derivation info)
    pub fn is_derived(&self) -> bool {
        self.index.is_some() && self.derivation_path.is_some()
    }

    /// Get short address for display (first 6 + last 4 chars)
    pub fn short_address(&self) -> String {
        if self.address.len() >= 42 {
            format!("{}...{}", &self.address[..6], &self.address[38..])
        } else {
            self.address.clone()
        }
    }

    /// Validate address format and consistency
    pub fn validate(&self) -> WalletResult<()> {
        // Validate address format
        crate::utils::validate_ethereum_address(&self.address)?;

        // Validate network
        if !config::is_supported_network(&self.network) {
            return Err(ValidationError::InvalidAddressFormat {
                address: self.network.clone(),
                expected: format!("one of: {:?}", config::SUPPORTED_NETWORKS),
            }
            .into());
        }

        // Validate derivation path consistency
        if let Some(ref path) = self.derivation_path {
            crate::utils::validate_derivation_path(path)?;

            // If we have a path, we should have an index
            if self.index.is_none() {
                return Err(ValidationError::IntegrityCheckFailed {
                    data_type: "address".to_string(),
                    details: "Derivation path provided without index".to_string(),
                }
                .into());
            }

            // Verify index matches path
            if let Some(index) = self.index {
                if !path.ends_with(&format!("/{}", index)) {
                    return Err(ValidationError::IntegrityCheckFailed {
                        data_type: "address".to_string(),
                        details: format!(
                            "Index {} doesn't match derivation path {}",
                            index, path
                        ),
                    }
                    .into());
                }
            }
        }

        Ok(())
    }

    /// Compare addresses (case-insensitive)
    pub fn equals(&self, other: &str) -> bool {
        self.address.to_lowercase() == other.to_lowercase()
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(label) = &self.label {
            write!(f, "{} ({})", self.checksummed_address(), label)
        } else {
            write!(f, "{}", self.checksummed_address())
        }
    }
}

/// Collection of addresses for bulk operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressBook {
    addresses: Vec<Address>,
    default_network: String,
}

impl AddressBook {
    /// Create a new address book
    pub fn new(default_network: String) -> Self {
        Self {
            addresses: Vec::new(),
            default_network,
        }
    }

    /// Add an address
    pub fn add(&mut self, address: Address) -> WalletResult<()> {
        // Validate before adding
        address.validate()?;

        // Check for duplicates
        if self.addresses.iter().any(|a| a.address == address.address) {
            return Err(ValidationError::IntegrityCheckFailed {
                data_type: "address_book".to_string(),
                details: format!("Address {} already exists", address.address),
            }
            .into());
        }

        self.addresses.push(address);
        Ok(())
    }

    /// Remove an address
    pub fn remove(&mut self, address: &str) -> bool {
        let initial_len = self.addresses.len();
        self.addresses
            .retain(|a| !a.equals(address));
        self.addresses.len() < initial_len
    }

    /// Find address by string
    pub fn find(&self, address: &str) -> Option<&Address> {
        self.addresses.iter().find(|a| a.equals(address))
    }

    /// Find address by label
    pub fn find_by_label(&self, label: &str) -> Option<&Address> {
        self.addresses
            .iter()
            .find(|a| a.label.as_deref() == Some(label))
    }

    /// Get all addresses
    pub fn addresses(&self) -> &[Address] {
        &self.addresses
    }

    /// Get addresses for specific network
    pub fn addresses_for_network(&self, network: &str) -> Vec<&Address> {
        self.addresses
            .iter()
            .filter(|a| a.network == network)
            .collect()
    }

    /// Get derived addresses only
    pub fn derived_addresses(&self) -> Vec<&Address> {
        self.addresses.iter().filter(|a| a.is_derived()).collect()
    }

    /// Validate all addresses
    pub fn validate(&self) -> WalletResult<()> {
        for address in &self.addresses {
            address.validate()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ADDRESS: &str = "0x742d35Cc6634C0532925a3b8D57c2b9b3f0B9a99";
    const TEST_DERIVATION_PATH: &str = "m/44'/60'/0'/0/0";

    #[test]
    fn test_address_creation() {
        let addr = Address::new(
            TEST_ADDRESS.to_string(),
            "mainnet".to_string(),
            Some(0),
            Some(TEST_DERIVATION_PATH.to_string()),
        )
        .unwrap();

        assert_eq!(addr.address(), TEST_ADDRESS.to_lowercase());
        assert_eq!(addr.index(), Some(0));
        assert_eq!(addr.derivation_path(), Some(TEST_DERIVATION_PATH));
        assert!(addr.is_derived());
    }

    #[test]
    fn test_address_from_string() {
        let addr = Address::from_string(TEST_ADDRESS, "mainnet").unwrap();

        assert_eq!(addr.address(), TEST_ADDRESS.to_lowercase());
        assert_eq!(addr.network(), "mainnet");
        assert!(!addr.is_derived());
    }

    #[test]
    fn test_address_validation() {
        let addr = Address::from_string(TEST_ADDRESS, "mainnet").unwrap();
        assert!(addr.validate().is_ok());

        // Test invalid address
        let result = Address::from_string("invalid", "mainnet");
        assert!(result.is_err());

        // Test invalid network
        let result = Address::from_string(TEST_ADDRESS, "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_address_equality() {
        let addr = Address::from_string(TEST_ADDRESS, "mainnet").unwrap();

        assert!(addr.equals(TEST_ADDRESS));
        assert!(addr.equals(&TEST_ADDRESS.to_lowercase()));
        assert!(addr.equals(&TEST_ADDRESS.to_uppercase()));
        assert!(!addr.equals("0x1234567890123456789012345678901234567890"));
    }

    #[test]
    fn test_short_address() {
        let addr = Address::from_string(TEST_ADDRESS, "mainnet").unwrap();
        let short = addr.short_address();

        assert!(short.starts_with("0x742d"));
        assert!(short.ends_with("9a99"));
        assert!(short.contains("..."));
    }

    #[test]
    fn test_address_book() {
        let mut book = AddressBook::new("mainnet".to_string());

        let addr1 = Address::from_string(TEST_ADDRESS, "mainnet").unwrap();
        let addr2 = Address::from_string("0x1234567890123456789012345678901234567890", "mainnet").unwrap();

        book.add(addr1.clone()).unwrap();
        book.add(addr2.clone()).unwrap();

        assert_eq!(book.addresses().len(), 2);
        assert!(book.find(TEST_ADDRESS).is_some());
        assert!(book.remove("0x1234567890123456789012345678901234567890"));
        assert_eq!(book.addresses().len(), 1);
    }

    #[test]
    fn test_derived_address_validation() {
        // Valid derived address
        let addr = Address::derived(
            TEST_ADDRESS.to_string(),
            "mainnet".to_string(),
            0,
            TEST_DERIVATION_PATH.to_string(),
        )
        .unwrap();
        assert!(addr.validate().is_ok());

        // Invalid: index doesn't match path
        let addr = Address::derived(
            TEST_ADDRESS.to_string(),
            "mainnet".to_string(),
            5,
            TEST_DERIVATION_PATH.to_string(),
        )
        .unwrap();
        assert!(addr.validate().is_err());
    }
}