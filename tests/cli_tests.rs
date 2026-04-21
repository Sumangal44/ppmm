use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_help_command() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_ppmm"));
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "PPM is a project manager for Python",
        ));
}

#[test]
fn test_version_command() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_ppmm"));
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("ppmm"));
}

#[test]
fn test_unknown_command() {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_ppmm"));
    cmd.arg("unknown_command")
        .assert()
        .failure() // Should fail or show help/error
        .stderr(predicate::str::contains("error").or(predicate::str::contains("Usage")));
}

// More complex tests like 'init' or 'install' would require mocking stdin/stdout or setting up a temp dir environment.
// For now, we verify the binary can run and respond to basic flags.
