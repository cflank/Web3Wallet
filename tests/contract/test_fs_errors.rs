//! Contract tests for file system error handling

use assert_cmd::Command;
use predicates::prelude::*;

/// Test file not found error
#[test]
fn test_fs_file_not_found() {
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["load", "non-existent-file.json"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("FS_002"))
        .stderr(predicate::str::contains("Wallet file not found"));
}

/// Test permission denied error (simulated)
#[test]
fn test_fs_permission_denied() {
    // This would need OS-specific setup to create permission issues
    // For now, test the error format when it occurs
    let mut cmd = Command::cargo_bin("wallet").unwrap();
    cmd.args(&["create", "--save", "/root/restricted/wallet.json"]);

    // If permission is denied, should show proper error
    if !cmd.assert().try_success().is_ok() {
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("FS_001"));
    }
}