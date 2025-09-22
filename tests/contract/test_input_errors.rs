//! Contract tests for user input validation errors

use assert_cmd::Command;
use predicates::prelude::*;

/// Test invalid command parameters
#[test]
fn test_input_invalid_parameters() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["create", "--words", "invalid"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("INPUT_001").or(
            predicate::str::contains("invalid value")
        ));
}

/// Test conflicting options
#[test]
fn test_input_conflicting_options() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&[
        "import",
        "--mnemonic", "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
        "--private-key", "0x4c0883a69102937d6231471b5dbb6204fe512961708279c1e3ae83da5e56df1a",
    ]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("INPUT_002").or(
            predicate::str::contains("cannot be used with")
        ));
}

/// Test missing required parameter
#[test]
fn test_input_missing_parameter() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.arg("import"); // No mnemonic or private key provided

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("INPUT_003").or(
            predicate::str::contains("required")
        ));
}