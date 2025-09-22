//! Contract tests for `wallet derive` command

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;
use std::fs;

/// Test wallet derive with valid path
#[test]
fn test_derive_command_valid_path() {
    let temp_dir = TempDir::new().unwrap();
    let wallet_path = temp_dir.path().join("test-wallet.json");

    fs::write(&wallet_path, r#"{"mock": "wallet"}"#).unwrap();

    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&[
        "derive",
        "m/44'/60'/0'/0/5",
        "--from-file", wallet_path.to_str().unwrap(),
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Address:"));
}

/// Test wallet derive with invalid path
#[test]
fn test_derive_command_invalid_path() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["derive", "invalid/path"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("CRYPTO_006"));
}