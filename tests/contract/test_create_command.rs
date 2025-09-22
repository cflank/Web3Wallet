//! Contract tests for `wallet create` command
//!
//! Tests the CLI interface contract for wallet creation functionality.
//! These tests verify the command-line interface behavior and output format.

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

/// Test that wallet create command with default parameters works
#[test]
fn test_create_command_default() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.arg("create");

    // Should succeed and output wallet information
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Address:"))
        .stdout(predicate::str::contains("Mnemonic:"))
        .stdout(predicate::str::contains("0x")); // Ethereum address format
}

/// Test wallet create with 12-word mnemonic
#[test]
fn test_create_command_12_words() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["create", "--words", "12"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Address:"));
}

/// Test wallet create with 24-word mnemonic
#[test]
fn test_create_command_24_words() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["create", "--words", "24"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Address:"));
}

/// Test wallet create with invalid word count
#[test]
fn test_create_command_invalid_word_count() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["create", "--words", "16"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Word count must be 12 or 24"));
}

/// Test wallet create with save option
#[test]
fn test_create_command_with_save() {
    let temp_dir = TempDir::new().unwrap();
    let wallet_path = temp_dir.path().join("test-wallet.json");

    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["create", "--save", wallet_path.to_str().unwrap()]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Wallet saved to:"));

    // Verify file was created
    assert!(wallet_path.exists());
}

/// Test wallet create with JSON output format
#[test]
fn test_create_command_json_output() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["create", "--output", "json"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(r#""success": true"#))
        .stdout(predicate::str::contains(r#""address":"#))
        .stdout(predicate::str::contains(r#""mnemonic":"#));
}

/// Test wallet create with custom network
#[test]
fn test_create_command_custom_network() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["create", "--network", "sepolia"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Network: sepolia"));
}

/// Test wallet create performance requirement (<1s)
#[test]
fn test_create_command_performance() {
    use std::time::Instant;

    let start = Instant::now();
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.arg("create");

    cmd.assert().success();

    let duration = start.elapsed();
    assert!(duration.as_secs() < 1, "Command took {:?}, should be <1s", duration);
}

/// Test wallet create help text
#[test]
fn test_create_command_help() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["create", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Create a new wallet"))
        .stdout(predicate::str::contains("--words"))
        .stdout(predicate::str::contains("--save"))
        .stdout(predicate::str::contains("--network"));
}

/// Test that created wallets have proper MetaMask-compatible addresses
#[test]
fn test_create_command_metamask_compatibility() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["create", "--output", "json"]);

    let output = cmd.assert().success().get_output().stdout.clone();
    let output_str = String::from_utf8(output).unwrap();

    // Should contain a valid Ethereum address format
    assert!(output_str.contains("0x"));

    // Extract address and verify it's 42 characters (0x + 40 hex chars)
    if let Some(start) = output_str.find(r#""address":"0x"#) {
        let addr_start = start + r#""address":""#.len();
        let addr_end = addr_start + 42; // 0x + 40 hex chars
        if addr_end <= output_str.len() {
            let address = &output_str[addr_start..addr_end];
            assert!(address.starts_with("0x"));
            assert_eq!(address.len(), 42);
            assert!(address[2..].chars().all(|c| c.is_ascii_hexdigit()));
        }
    }
}