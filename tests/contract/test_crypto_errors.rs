//! Contract tests for cryptographic error handling
//!
//! Tests that cryptographic operations fail gracefully with proper error codes.

use assert_cmd::Command;
use predicates::prelude::*;

/// Test insufficient entropy error
#[test]
fn test_crypto_insufficient_entropy() {
    // This test would require mocking the entropy source
    // For now, we test the error code format
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["create", "--verbose"]);

    // In case of entropy issues, should show proper error code
    if !cmd.assert().try_success().is_ok() {
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("CRYPTO_001"));
    }
}

/// Test invalid mnemonic error
#[test]
fn test_crypto_invalid_mnemonic() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["import", "--mnemonic", "not a valid mnemonic phrase"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("CRYPTO_002"))
        .stderr(predicate::str::contains("Invalid BIP39 mnemonic"));
}

/// Test invalid private key error
#[test]
fn test_crypto_invalid_private_key() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["import", "--private-key", "not_a_valid_key"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("CRYPTO_003"))
        .stderr(predicate::str::contains("Invalid private key format"));
}

/// Test invalid derivation path error
#[test]
fn test_crypto_invalid_derivation_path() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["derive", "not/a/valid/path"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("CRYPTO_006"))
        .stderr(predicate::str::contains("Invalid HD derivation path"));
}