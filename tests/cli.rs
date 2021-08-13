// Reference doc: https://rust-cli.github.io/book/tutorial/testing.html
// Output print statements while running test: `cargo test -- --nocapture`
//   https://medium.com/@ericdreichert/how-to-print-during-rust-tests-619bdc7ccebc

use assert_cmd::prelude::*;
// Add methods on commands
use predicates::prelude::*;
// Used for writing assertions
use std::process::Command;
// Run programs
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
/// This test also tests the add alias 'a'
fn create_csv_with_headers_if_not_exist() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    cmd.arg("a").arg("https://google.com").arg("Google");
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

#[test]
fn ignore_first_line() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    // Create the file, header, and a line to search
    setup_add(&csv_path, "https://google.com", "Google Search Engine", None)?;

    cmd.arg("search").arg("URL");

    cmd.assert().success().stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn single_word_match() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    // Create the file, header, and a line to search
    setup_add(&csv_path, "https://google.com", "Google Search Engine", None)?;
    setup_add(&csv_path, "https://bing.com", "MS Search", Some(vec!["Search", "Engine"]))?;
    setup_add(&csv_path, "https://yahoo.com", "Yahoo Engine", Some(vec!["Yahoo", "Search"]))?;

    // Case insensitive search
    cmd.arg("search").arg("google");

    test_count_matches(&mut cmd, 1)?;

    Ok(())
}

#[test]
fn regex_match() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    // Create the file, header, and a line to search
    setup_add(&csv_path, "https://google.com", "Google Search Engine", None)?;
    setup_add(&csv_path, "https://bing.com", "MS Search", Some(vec!["Search", "Engine"]))?;
    setup_add(&csv_path, "https://yahoo.com", "Yahoo Engine", Some(vec!["Yahoo", "Search"]))?;

    // Note that is should only match URL and description, not tags
    cmd.arg("search").arg("S.arch");

    test_count_matches(&mut cmd, 2)?;

    Ok(())
}

#[test]
fn search_alias_s() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    // Create the file, header, and a line to search
    setup_add(&csv_path, "https://google.com", "Google Search Engine", None)?;
    setup_add(&csv_path, "https://bing.com", "MS Search", Some(vec!["Search", "Engine"]))?;
    setup_add(&csv_path, "https://yahoo.com", "Yahoo Engine", Some(vec!["Yahoo", "Search"]))?;

    // Note that is should only match URL and description, not tags
    cmd.arg("s").arg("Engine");

    test_count_matches(&mut cmd, 2)?;

    Ok(())
}

#[test]
fn multi_word_match() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    // Create the file, header, and a line to search
    setup_add(&csv_path, "https://google.com", "Google Search Engine", None)?;
    setup_add(&csv_path, "https://bing.com", "MS Search", Some(vec!["Search", "Engine"]))?;
    setup_add(&csv_path, "https://yahoo.com", "Yahoo Engine", Some(vec!["Yahoo", "Search"]))?;

    // Case insensitive search that only matches the two words together
    cmd.arg("search").arg("Search Engine");

    test_count_matches(&mut cmd, 1)?;

    Ok(())
}

/// Setup the test environment with a temporary CSV file.
/// To populate the CSV with contents, use "add" command.
///
/// Returns the temp directory and file both so they can be accessed directly, but
/// mostly so they stay in scope until the test is complete
fn setup() -> Result<(TempDir, PathBuf, Command)> {
    let dir = tempdir()?;
    let csv_path = dir.path().join("tmp.csv");

    let cmd = setup_cmd(&csv_path)?;

    Ok((dir, csv_path, cmd))
}

fn setup_cmd(csv_path: &PathBuf) -> Result<Command> {
    let mut cmd = Command::cargo_bin("bm")?;

    cmd.env("BOOKMARK_MANAGER_CSV", csv_path.to_str().unwrap());

    Ok(cmd)
}

/// Use the program to add a bookmark.
/// This is useful for setting up for a search test.
fn setup_add(csv_path: &PathBuf, url: &str, description: &str, tags: Option<Vec<&str>>) -> Result<()> {
    let mut cmd = setup_cmd(csv_path)?;

    cmd.arg("add")
        .arg(url)
        .arg(description);

    if let Some(tags) = tags {
        for tag in tags {
            cmd.arg("--tag")
                .arg(tag);
        }
    }

    cmd.assert().success();

    Ok(())
}

fn test_count_matches(cmd: &mut Command, expected_num_matches: usize) -> Result<()> {
    let assert = cmd.assert().success();

    // https://stackoverflow.com/questions/19076719/how-do-i-convert-a-vector-of-bytes-u8-to-a-string
    let stdout = std::str::from_utf8(&*assert.get_output().stdout).unwrap();

    let num_matches = &stdout.lines().count();

    ensure!(num_matches == &expected_num_matches,
        "Unexpected number of matches [{}]: {}", num_matches, stdout);

    Ok(())
}