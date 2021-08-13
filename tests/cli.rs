use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use anyhow::Result;

#[test]
fn invalid_command() -> Result<()> {
    let mut cmd = Command::cargo_bin("bm")?;

    cmd.arg("stuff");
    cmd.assert()
        .failure();

    Ok(())
}