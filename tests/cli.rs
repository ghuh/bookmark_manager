use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use anyhow::{Result, ensure};
use tempfile::{tempdir, TempDir};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::PathBuf;

const HEADER_ROW: &str = "URL|DESCRIPTION|TAGS";

#[test]
fn invalid_command() -> Result<()> {
    let (_csv_dir, _csv_path, mut cmd) = setup()?;

    cmd.arg("stuff");
    cmd.assert()
        .failure();

    Ok(())
}

#[test]
fn create_csv_with_headers_if_not_exist() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    cmd.arg("add").arg("http://google.com").arg("Google");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("CSV file created"));

    let mut buffer = BufReader::new(File::open(csv_path.as_path())?);
    let mut first_line = String::new();
    buffer.read_line(&mut first_line)?;

    // Need to trim to remove new line at the end
    ensure!(first_line.trim() == HEADER_ROW, "Program didn't create header");

    Ok(())
}

/// Setup the test environment with a temporary CSV file.
/// To populate the CSV with contents, use "add" command
fn setup() -> Result<(TempDir, PathBuf, Command)> {
    let dir = tempdir()?;
    let csv_path = dir.path().join("tmp.csv");

    let mut cmd = Command::cargo_bin("bm")?;

    cmd.env("BOOKMARK_MANAGER_CSV", csv_path.to_str().unwrap());

    Ok((dir, csv_path, cmd))
}