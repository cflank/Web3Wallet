//! Contract tests for `wallet import` command
//!
//! Tests the CLI interface contract for wallet import functionality.
//! Verifies mnemonic and private key import with proper validation.

use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::TempDir;

const VALID_MNEMONIC_12: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
const VALID_MNEMONIC_24: &str = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
const VALID_PRIVATE_KEY: &str = "0x4c0883a69102937d6231471b5dbb6204fe512961708279c1e3ae83da5e56df1a";
const EXPECTED_ADDRESS: &str = "0x9858EfFD232B4033E47d90003D41EC34EcaEda94";

/// Test wallet import with valid 12-word mnemonic
#[test]
fn test_import_command_mnemonic_12() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["import", "--mnemonic", VALID_MNEMONIC_12]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Address:"))
        .stdout(predicate::str::contains(EXPECTED_ADDRESS));
}

/// Test wallet import with valid 24-word mnemonic
#[test]
fn test_import_command_mnemonic_24() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["import", "--mnemonic", VALID_MNEMONIC_24]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Address:"));
}

/// Test wallet import with valid private key
#[test]
fn test_import_command_private_key() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["import", "--private-key", VALID_PRIVATE_KEY]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Address:"))
        .stdout(predicate::str::contains(EXPECTED_ADDRESS));
}

/// Test wallet import with invalid mnemonic
#[test]
fn test_import_command_invalid_mnemonic() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["import", "--mnemonic", "invalid mnemonic phrase"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("CRYPTO_002"));
}

/// Test wallet import with invalid private key format
#[test]
fn test_import_command_invalid_private_key() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["import", "--private-key", "invalid_key"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("CRYPTO_003"));
}

/// Test wallet import with conflicting options
#[test]
fn test_import_command_conflicting_options() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&[
        "import",
        "--mnemonic", VALID_MNEMONIC_12,
        "--private-key", VALID_PRIVATE_KEY,
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

/// Test wallet import with save option
#[test]
fn test_import_command_with_save() {
    let temp_dir = TempDir::new().unwrap();
    let wallet_path = temp_dir.path().join("imported-wallet.json");

    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&[
        "import",
        "--mnemonic", VALID_MNEMONIC_12,
        "--save", wallet_path.to_str().unwrap(),
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Wallet saved to:"));

    // Verify file was created
    assert!(wallet_path.exists());
}

/// Test wallet import with JSON output
#[test]
fn test_import_command_json_output() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&[
        "import",
        "--mnemonic", VALID_MNEMONIC_12,
        "--output", "json",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(r#""success": true"#))
        .stdout(predicate::str::contains(r#""address":"#))
        .stdout(predicate::str::contains(EXPECTED_ADDRESS));
}

/// Test wallet import MetaMask compatibility
#[test]
fn test_import_command_metamask_compatibility() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&[
        "import",
        "--mnemonic", VALID_MNEMONIC_12,
        "--output", "json",
    ]);

    let output = cmd.assert().success().get_output().stdout.clone();
    let output_str = String::from_utf8(output).unwrap();

    // Should generate the same address as MetaMask for this mnemonic
    assert!(output_str.contains(EXPECTED_ADDRESS));
}

/// Test wallet import with custom network
#[test]
fn test_import_command_custom_network() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&[
        "import",
        "--mnemonic", VALID_MNEMONIC_12,
        "--network", "sepolia",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Network: sepolia"));
}

/// Test wallet import performance requirement (<1s)
#[test]
fn test_import_command_performance() {
    use std::time::Instant;

    let start = Instant::now();
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["import", "--mnemonic", VALID_MNEMONIC_12]);

    cmd.assert().success();

    let duration = start.elapsed();
    assert!(duration.as_secs() < 1, "Command took {:?}, should be <1s", duration);
}

/// Test wallet import help text
#[test]
fn test_import_command_help() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["import", "--help"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Import an existing wallet"))
        .stdout(predicate::str::contains("--mnemonic"))
        .stdout(predicate::str::contains("--private-key"))
        .stdout(predicate::str::contains("--save"));
}

/// Test missing import source error
#[test]
fn test_import_command_missing_source() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.arg("import");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("INPUT_003")); // Missing required parameter
}