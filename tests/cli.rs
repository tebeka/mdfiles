use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_no_args_finds_files() {
    let mut cmd = Command::cargo_bin("mdfiles").unwrap();
    cmd.assert().success();
}

#[test]
fn test_long_flag_with_valid_date() {
    let mut cmd = Command::cargo_bin("mdfiles").unwrap();
    cmd.arg("--date").arg("2025-12-25")
        .assert()
        .success();
}

#[test]
fn test_short_flag_with_valid_date() {
    let mut cmd = Command::cargo_bin("mdfiles").unwrap();
    cmd.arg("-d").arg("2024-01-01")
        .assert()
        .success();
}

#[test]
fn test_invalid_date_format() {
    let mut cmd = Command::cargo_bin("mdfiles").unwrap();
    cmd.arg("--date").arg("25-12-2025")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid date format"));
}

#[test]
fn test_invalid_date_value() {
    let mut cmd = Command::cargo_bin("mdfiles").unwrap();
    cmd.arg("-d").arg("2025-13-45")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid date format"));
}

#[test]
fn test_malformed_date() {
    let mut cmd = Command::cargo_bin("mdfiles").unwrap();
    cmd.arg("--date").arg("not-a-date")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid date format"));
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("mdfiles").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Date in YYYY-MM-DD format"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("mdfiles").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("mdfiles"));
}

#[test]
fn test_no_args_with_current_directory() {
    let mut cmd = Command::cargo_bin("mdfiles").unwrap();
    cmd.assert().success();
    // Should succeed and may or may not find files depending on current directory
}
