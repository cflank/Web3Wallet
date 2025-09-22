//! # Mnemonic Generation Service
//!
//! BIP39 mnemonic phrase generation and validation with secure entropy.

use crate::config;
use crate::errors::{CryptographicError, WalletResult};
use bip39::{Language, Mnemonic};
use rand::RngCore;
use std::str::FromStr;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Mnemonic generation service
pub struct MnemonicService;

impl MnemonicService {
    /// Generate a new random mnemonic phrase
    pub fn generate(word_count: u8) -> WalletResult<SecureMnemonic> {
        // Validate word count
        if !config::is_supported_word_count(word_count) {
            return Err(CryptographicError::InvalidMnemonic {
                details: format!("Unsupported word count: {}", word_count),
                suggestion: "Use 12 or 24 words".to_string(),
            }
            .into());
        }

        // Get entropy bits for word count
        let entropy_bits = config::entropy_bits_for_word_count(word_count)
            .ok_or_else(|| CryptographicError::InvalidMnemonic {
                details: format!("Cannot determine entropy for {} words", word_count),
                suggestion: "Use 12 or 24 words".to_string(),
            })?;

        // Generate cryptographically secure random entropy
        let mut entropy = vec![0u8; entropy_bits / 8];

        // Check if we have sufficient entropy
        Self::check_entropy_availability(entropy_bits)?;

        // Fill entropy with secure random data
        rand::thread_rng().fill_bytes(&mut entropy);

        // Create mnemonic from entropy
        let mnemonic = Mnemonic::from_entropy(&entropy).map_err(|e| {
            CryptographicError::InvalidMnemonic {
                details: e.to_string(),
                suggestion: "Ensure system has adequate entropy sources".to_string(),
            }
        })?;

        // Clear entropy from memory
        entropy.zeroize();

        Ok(SecureMnemonic::new(mnemonic.to_string()))
    }

    /// Validate an existing mnemonic phrase
    pub fn validate(mnemonic_str: &str) -> WalletResult<SecureMnemonic> {
        // Parse and validate mnemonic
        let mnemonic = Mnemonic::from_str(mnemonic_str).map_err(|e| {
            CryptographicError::InvalidMnemonic {
                details: e.to_string(),
                suggestion: "Verify the mnemonic phrase has the correct number of words (12 or 24) and all words are from the BIP39 wordlist.".to_string(),
            }
        })?;

        // Validate word count
        let word_count = mnemonic_str.split_whitespace().count();
        if !config::is_supported_word_count(word_count as u8) {
            return Err(CryptographicError::InvalidMnemonic {
                details: format!("Unsupported word count: {}", word_count),
                suggestion: "Use 12 or 24 words".to_string(),
            }
            .into());
        }

        Ok(SecureMnemonic::new(mnemonic.to_string()))
    }

    /// Generate seed from mnemonic with optional passphrase
    pub fn generate_seed(mnemonic: &SecureMnemonic, passphrase: Option<&str>) -> WalletResult<SecureSeed> {
        let bip39_mnemonic = Mnemonic::from_str(mnemonic.phrase()).map_err(|e| {
            CryptographicError::InvalidMnemonic {
                details: e.to_string(),
                suggestion: "Ensure mnemonic is valid BIP39 format".to_string(),
            }
        })?;

        let passphrase = passphrase.unwrap_or("");
        let seed = bip39_mnemonic.to_seed(passphrase);

        Ok(SecureSeed::new(seed.to_vec()))
    }

    /// Check entropy strength
    pub fn check_mnemonic_strength(mnemonic: &SecureMnemonic) -> MnemonicStrength {
        let word_count = mnemonic.phrase().split_whitespace().count();
        match word_count {
            12 => MnemonicStrength::Standard,
            24 => MnemonicStrength::High,
            _ => MnemonicStrength::Weak,
        }
    }

    /// Verify entropy availability
    fn check_entropy_availability(required_bits: usize) -> WalletResult<()> {
        // This is a simplified check - in production, you might want to
        // check /proc/sys/kernel/random/entropy_avail on Linux
        if required_bits > 512 {
            return Err(CryptographicError::InsufficientEntropy {
                available: 256, // Simplified value
                required: required_bits as u32,
                suggestion: "Ensure system has adequate entropy sources. On Linux, consider installing rng-tools.".to_string(),
            }
            .into());
        }

        Ok(())
    }

    /// Convert mnemonic to different word counts (if possible)
    pub fn convert_word_count(_mnemonic: &SecureMnemonic, target_words: u8) -> WalletResult<SecureMnemonic> {
        // This is generally not possible while maintaining the same seed
        // We generate a new mnemonic instead
        if !config::is_supported_word_count(target_words) {
            return Err(CryptographicError::InvalidMnemonic {
                details: format!("Unsupported target word count: {}", target_words),
                suggestion: "Use 12 or 24 words".to_string(),
            }
            .into());
        }

        // For conversion, we need to generate a new mnemonic
        // as changing word count changes the underlying entropy
        Self::generate(target_words)
    }

    /// Get mnemonic word list for validation
    pub fn get_word_list() -> &'static [&'static str] {
        Language::English.word_list()
    }

    /// Check if a word is in the BIP39 word list
    pub fn is_valid_word(word: &str) -> bool {
        Self::get_word_list().contains(&word)
    }

    /// Get word suggestions for partial input
    pub fn suggest_words(partial: &str) -> Vec<&'static str> {
        if partial.is_empty() {
            return Vec::new();
        }

        Self::get_word_list()
            .iter()
            .filter(|word| word.starts_with(partial))
            .take(10) // Limit suggestions
            .copied()
            .collect()
    }
}

/// Secure mnemonic phrase with automatic memory cleanup
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct SecureMnemonic {
    phrase: String,
}

impl SecureMnemonic {
    /// Create new secure mnemonic
    pub fn new(phrase: String) -> Self {
        Self { phrase }
    }

    /// Get the mnemonic phrase
    pub fn phrase(&self) -> &str {
        &self.phrase
    }

    /// Get word count
    pub fn word_count(&self) -> usize {
        self.phrase.split_whitespace().count()
    }

    /// Get individual words
    pub fn words(&self) -> Vec<&str> {
        self.phrase.split_whitespace().collect()
    }

    /// Get word at specific index
    pub fn word_at(&self, index: usize) -> Option<&str> {
        self.words().get(index).copied()
    }

    /// Validate the mnemonic
    pub fn validate(&self) -> WalletResult<()> {
        MnemonicService::validate(&self.phrase)?;
        Ok(())
    }
}

/// Secure seed with automatic memory cleanup
#[derive(Debug, Clone, Zeroize, ZeroizeOnDrop)]
pub struct SecureSeed {
    bytes: Vec<u8>,
}

impl SecureSeed {
    /// Create new secure seed
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Get seed bytes (use carefully)
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Get seed length
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Check if seed is empty
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

/// Mnemonic strength levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MnemonicStrength {
    /// Weak (non-standard word count)
    Weak,
    /// Standard (12 words, 128 bits entropy)
    Standard,
    /// High (24 words, 256 bits entropy)
    High,
}

impl MnemonicStrength {
    /// Get entropy bits for strength level
    pub fn entropy_bits(self) -> usize {
        match self {
            MnemonicStrength::Weak => 0,
            MnemonicStrength::Standard => 128,
            MnemonicStrength::High => 256,
        }
    }

    /// Get description
    pub fn description(self) -> &'static str {
        match self {
            MnemonicStrength::Weak => "Weak (non-standard)",
            MnemonicStrength::Standard => "Standard (128-bit)",
            MnemonicStrength::High => "High (256-bit)",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mnemonic_generation() {
        let mnemonic = MnemonicService::generate(12).unwrap();
        assert_eq!(mnemonic.word_count(), 12);
        assert!(mnemonic.validate().is_ok());

        let mnemonic = MnemonicService::generate(24).unwrap();
        assert_eq!(mnemonic.word_count(), 24);
        assert!(mnemonic.validate().is_ok());
    }

    #[test]
    fn test_invalid_word_count() {
        let result = MnemonicService::generate(16);
        assert!(result.is_err());
    }

    #[test]
    fn test_mnemonic_validation() {
        let valid_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic = MnemonicService::validate(valid_mnemonic).unwrap();
        assert_eq!(mnemonic.word_count(), 12);

        let invalid_mnemonic = "invalid mnemonic phrase";
        let result = MnemonicService::validate(invalid_mnemonic);
        assert!(result.is_err());
    }

    #[test]
    fn test_seed_generation() {
        let valid_mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic = MnemonicService::validate(valid_mnemonic).unwrap();

        let seed = MnemonicService::generate_seed(&mnemonic, None).unwrap();
        assert!(!seed.is_empty());
        assert_eq!(seed.len(), 64); // BIP39 seed is 64 bytes

        // Test with passphrase
        let seed_with_passphrase = MnemonicService::generate_seed(&mnemonic, Some("test")).unwrap();
        assert_ne!(seed.bytes(), seed_with_passphrase.bytes());
    }

    #[test]
    fn test_mnemonic_strength() {
        let mnemonic_12 = MnemonicService::generate(12).unwrap();
        let strength_12 = MnemonicService::check_mnemonic_strength(&mnemonic_12);
        assert_eq!(strength_12, MnemonicStrength::Standard);

        let mnemonic_24 = MnemonicService::generate(24).unwrap();
        let strength_24 = MnemonicService::check_mnemonic_strength(&mnemonic_24);
        assert_eq!(strength_24, MnemonicStrength::High);
    }

    #[test]
    fn test_word_validation() {
        assert!(MnemonicService::is_valid_word("abandon"));
        assert!(MnemonicService::is_valid_word("about"));
        assert!(!MnemonicService::is_valid_word("invalid"));
        assert!(!MnemonicService::is_valid_word(""));
    }

    #[test]
    fn test_word_suggestions() {
        let suggestions = MnemonicService::suggest_words("aba");
        assert!(suggestions.contains(&"abandon"));
        assert!(suggestions.len() <= 10);

        let empty_suggestions = MnemonicService::suggest_words("");
        assert!(empty_suggestions.is_empty());
    }

    #[test]
    fn test_secure_mnemonic() {
        let phrase = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let mnemonic = SecureMnemonic::new(phrase.to_string());

        assert_eq!(mnemonic.phrase(), phrase);
        assert_eq!(mnemonic.word_count(), 12);
        assert_eq!(mnemonic.word_at(0), Some("abandon"));
        assert_eq!(mnemonic.word_at(11), Some("about"));
        assert_eq!(mnemonic.word_at(12), None);
    }
}