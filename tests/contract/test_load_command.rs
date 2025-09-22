//! Contract tests for `wallet load` command
//!
//! Tests the CLI interface contract for wallet loading functionality.

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

/// Test wallet load with existing file
#[test]
fn test_load_command_existing_file() {
    let temp_dir = TempDir::new().unwrap();
    let wallet_path = temp_dir.path().join("test-wallet.json");

    // Create a mock wallet file
    fs::write(&wallet_path, r#"{"mock": "wallet"}"#).unwrap();

    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["load", wallet_path.to_str().unwrap()]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Address:"));
}

/// Test wallet load with non-existent file
#[test]
fn test_load_command_missing_file() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["load", "non-existent-wallet.json"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("FS_002"));
}

/// Test wallet load with address-only option
#[test]
fn test_load_command_address_only() {
    let temp_dir = TempDir::new().unwrap();
    let wallet_path = temp_dir.path().join("test-wallet.json");

    fs::write(&wallet_path, r#"{"mock": "wallet"}"#).unwrap();

    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["load", wallet_path.to_str().unwrap(), "--address-only"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Address:"))
        .stdout(predicate::str::contains("not decrypted").not());
}