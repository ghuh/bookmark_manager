// Reference doc: https://rust-cli.github.io/book/tutorial/testing.html
// Output print statements while running test: `cargo test -- --nocapture`
//   https://medium.com/@ericdreichert/how-to-print-during-rust-tests-619bdc7ccebc

use assert_cmd::prelude::*;
use std::fs;
// Add methods on commands
use predicates::prelude::*;
// Used for writing assertions
use std::process::Command;
// Run programs
use anyhow::{ensure, Result};
use git2::Repository;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use tempfile::{tempdir, TempDir};

const HEADER_ROW: &str = "URL|DESCRIPTION|TAGS";

#[test]
fn invalid_command() -> Result<()> {
    let (_csv_dir, _csv_path, mut cmd) = setup()?;

    cmd.arg("stuff");
    cmd.assert().failure();

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
    ensure!(
        first_line.trim() == HEADER_ROW,
        "Program didn't create header"
    );

    Ok(())
}

#[test]
fn no_duplicate_urls() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    setup_add(
        &csv_path,
        "https://google.com",
        "Google Search Engine",
        None,
    )?;
    cmd.arg("a").arg("https://google.com").arg("Google");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("has already been bookmarked"));

    Ok(())
}

#[test]
fn validate_fails_for_pipe_in_description() -> Result<()> {
    let (_csv_dir, _csv_path, mut cmd) = setup()?;

    cmd.arg("a").arg("https://google.com").arg("Goo|gle");
    cmd.assert().failure();

    Ok(())
}

#[test]
fn ignore_first_line() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    // Create the file, header, and a line to search
    setup_add(
        &csv_path,
        "https://google.com",
        "Google Search Engine",
        None,
    )?;

    cmd.arg("search").arg("URL");

    cmd.assert().success().stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn sort_tags() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    // Tags are out of order
    setup_add(
        &csv_path,
        "https://google.com",
        "Google Search Engine",
        Some(vec!["c", "B", "a"]),
    )?;

    cmd.arg("search").arg("google");

    cmd.assert()
        .success()
        // Tags are sorted case insensitively
        .stdout(predicate::str::contains("a | B | c"));

    Ok(())
}

#[test]
fn single_word_match() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    // Create the file, header, and a line to search
    setup_add(
        &csv_path,
        "https://google.com",
        "Google Search Engine",
        None,
    )?;
    setup_add(
        &csv_path,
        "https://bing.com",
        "MS Search",
        Some(vec!["Search", "Engine"]),
    )?;
    setup_add(
        &csv_path,
        "https://yahoo.com",
        "Yahoo Engine",
        Some(vec!["Yahoo", "Search"]),
    )?;

    // Case insensitive search
    cmd.arg("search").arg("google");

    test_count_matches(&mut cmd, 1)?;

    Ok(())
}

#[test]
fn regex_match() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    // Create the file, header, and a line to search
    setup_add(
        &csv_path,
        "https://google.com",
        "Google Search Engine",
        None,
    )?;
    setup_add(
        &csv_path,
        "https://bing.com",
        "MS Search",
        Some(vec!["Search", "Engine"]),
    )?;
    setup_add(
        &csv_path,
        "https://yahoo.com",
        "Yahoo Engine",
        Some(vec!["Yahoo", "Search"]),
    )?;

    // Note that is should only match URL and description, not tags
    cmd.arg("search").arg("S.arch");

    test_count_matches(&mut cmd, 2)?;

    Ok(())
}

#[test]
fn search_alias_s() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    // Create the file, header, and a line to search
    setup_add(
        &csv_path,
        "https://google.com",
        "Google Search Engine",
        None,
    )?;
    setup_add(
        &csv_path,
        "https://bing.com",
        "MS Search",
        Some(vec!["Search", "Engine"]),
    )?;
    setup_add(
        &csv_path,
        "https://yahoo.com",
        "Yahoo Engine",
        Some(vec!["Yahoo", "Search"]),
    )?;

    // Note that is should only match URL and description, not tags
    cmd.arg("s").arg("Engine");

    test_count_matches(&mut cmd, 2)?;

    Ok(())
}

#[test]
fn multi_word_match() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    // Create the file, header, and a line to search
    setup_add(
        &csv_path,
        "https://google.com",
        "Google Search Engine",
        None,
    )?;
    setup_add(
        &csv_path,
        "https://bing.com",
        "MS Search",
        Some(vec!["Search", "Engine"]),
    )?;
    setup_add(
        &csv_path,
        "https://yahoo.com",
        "Yahoo Engine",
        Some(vec!["Yahoo", "Search"]),
    )?;

    // Case insensitive search that only matches the two words together
    cmd.arg("search").arg("Search Engine");

    test_count_matches(&mut cmd, 1)?;

    Ok(())
}

#[test]
/// This also tests multi word tags
fn tags_only_query() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    // Create the file, header, and a line to search
    setup_add(
        &csv_path,
        "https://google.com",
        "Google Search Engine",
        None,
    )?;
    setup_add(
        &csv_path,
        "https://bing.com",
        "MS Search",
        Some(vec!["Search Engine"]),
    )?;
    setup_add(
        &csv_path,
        "https://yahoo.com",
        "Yahoo Engine",
        Some(vec!["Yahoo", "Search"]),
    )?;
    setup_add(
        &csv_path,
        "https://duckduckgo.com/",
        "Privacy search Engine",
        Some(vec!["Search Engine"]),
    )?;

    // Case insensitive search that only matches the two words together
    cmd.arg("search").arg("--tag").arg("Search Engine");

    test_count_matches(&mut cmd, 2)?;

    Ok(())
}

#[test]
/// Also tests tags are case insensitive
fn multi_tag_query() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    // Create the file, header, and a line to search
    setup_add(
        &csv_path,
        "https://google.com",
        "Google match me Search Engine",
        Some(vec!["Search"]),
    )?;
    setup_add(
        &csv_path,
        "https://bing.com",
        "MS Search",
        Some(vec!["Search"]),
    )?;
    setup_add(
        &csv_path,
        "https://yahoo.com",
        "Yahoo match me Engine",
        Some(vec!["Search", "Engine"]),
    )?;
    setup_add(
        &csv_path,
        "https://duckduckgo.com/",
        "Privacy match me search Engine",
        Some(vec!["Search", "Engine"]),
    )?;

    // Case insensitive search that only matches the two words together
    cmd.arg("search")
        .arg("match me")
        .arg("--tag")
        .arg("Search")
        .arg("-t")
        .arg("engine");

    test_count_matches(&mut cmd, 2)?;

    Ok(())
}

#[test]
fn list_tags() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup()?;

    // Create the file, header, and a line to search
    setup_add(
        &csv_path,
        "https://google.com",
        "Google match me Search Engine",
        Some(vec!["Search"]),
    )?;
    setup_add(
        &csv_path,
        "https://bing.com",
        "MS Search",
        Some(vec!["search", "a"]),
    )?;
    setup_add(
        &csv_path,
        "https://yahoo.com",
        "Yahoo match me Engine",
        Some(vec!["Search", "B"]),
    )?;
    setup_add(
        &csv_path,
        "https://duckduckgo.com/",
        "Privacy match me search Engine",
        Some(vec!["c", "Engine"]),
    )?;

    cmd.arg("tags").assert().success().stdout(predicate::eq(
        "\
a
B
c
Engine
Search, search
",
    ));

    Ok(())
}

#[test]
fn csv_not_in_git_root() -> Result<()> {
    // Most of this code is from setup()
    let dir = tempdir()?;
    let sub_dir = dir.path().join("sub_dir");
    std::fs::create_dir(&sub_dir)?;

    let csv_path = sub_dir.join("tmp.csv");

    init_repo_and_create_initial_commit(dir.path())?;

    // This will do its own assert
    setup_add(
        &csv_path,
        "https://google.com",
        "Google match me Search Engine",
        Some(vec!["Search"]),
    )?;

    Ok(())
}

#[test]
fn csv_not_in_git_repo() -> Result<()> {
    // Most of this code is from setup()
    let dir = tempdir()?;
    let csv_path = dir.path().join("tmp.csv");
    let mut cmd = setup_cmd(&csv_path)?;

    cmd.arg("a").arg("https://google.com").arg("Google");
    cmd.assert().success().stdout(predicate::str::contains(
        "It appears the CSV file is not in a git repo. Use --no-commit to suppress this message",
    ));

    Ok(())
}

#[test]
fn fail_if_uncommitted_changes_in_git_rep() -> Result<()> {
    let (csv_dir, _csv_path, mut cmd) = setup()?;

    File::create(csv_dir.path().join("new_file"))?;

    cmd.arg("a").arg("https://google.com").arg("Google");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Git repo has uncommitted changes"));

    Ok(())
}

#[test]
fn do_not_fail_if_ignored_file_in_git_rep() -> Result<()> {
    let (csv_dir, _csv_path, mut cmd) = setup()?;

    // Create an ignored file
    // Can't use .gitignore because then it needs to be commited so use exclude instead
    // https://docs.github.com/en/get-started/getting-started-with-git/ignoring-files#excluding-local-files-without-creating-a-gitignore-file
    fs::write(csv_dir.path().join(".git/info/exclude"), "ignored_file")?;
    File::create(csv_dir.path().join("ignored_file"))?;

    cmd.arg("a").arg("https://google.com").arg("Google");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("CSV file created"));

    Ok(())
}

#[test]
fn no_commit() -> Result<()> {
    let (csv_dir, _csv_path, mut cmd) = setup()?;

    cmd.arg("a")
        .arg("https://google.com")
        .arg("Google")
        .arg("--no-commit");
    cmd.assert().success();

    let repo = Repository::open(csv_dir.path())?;
    let statuses = repo.statuses(None)?;
    let is_dirty = statuses.iter().any(|e| e.status() != git2::Status::CURRENT);
    assert!(is_dirty, "Working tree should be dirty");

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

    init_repo_and_create_initial_commit(dir.path())?;

    let cmd = setup_cmd(&csv_path)?;

    Ok((dir, csv_path, cmd))
}

// https://github.com/rust-lang/git2-rs/blob/master/examples/init.rs#L94
/// Unlike regular "git init", this example shows how to create an initial empty
/// commit in the repository. This is the helper function that does that.
fn init_repo_and_create_initial_commit(git_repo_path: &Path) -> Result<(), git2::Error> {
    let repo = Repository::init(git_repo_path)?;

    // First use the config to initialize a commit signature for the user.
    let sig = repo.signature()?;

    // Now let's create an empty tree for this commit
    let tree_id = {
        let mut index = repo.index()?;

        // Outside of this example, you could call index.add_path()
        // here to put actual files into the index. For our purposes, we'll
        // leave it empty for now.

        index.write_tree()?
    };

    let tree = repo.find_tree(tree_id)?;

    // Ready to create the initial commit.
    //
    // Normally creating a commit would involve looking up the current HEAD
    // commit and making that be the parent of the initial commit, but here this
    // is the first commit so there will be no parent.
    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;

    Ok(())
}

fn setup_cmd(csv_path: &Path) -> Result<Command> {
    let mut cmd = Command::cargo_bin("bm")?;

    cmd.env("BOOKMARK_MANAGER_CSV", csv_path.to_str().unwrap());

    Ok(cmd)
}

/// Use the program to add a bookmark.
/// This is useful for setting up for a search test.
fn setup_add(csv_path: &Path, url: &str, description: &str, tags: Option<Vec<&str>>) -> Result<()> {
    let mut cmd = setup_cmd(csv_path)?;

    cmd.arg("add").arg(url).arg(description);

    if let Some(tags) = tags {
        for tag in tags {
            cmd.arg("--tag").arg(tag);
        }
    }

    cmd.assert().success();

    Ok(())
}

fn test_count_matches(cmd: &mut Command, expected_num_matches: usize) -> Result<()> {
    let assert = cmd.assert().success();

    // https://stackoverflow.com/questions/19076719/how-do-i-convert-a-vector-of-bytes-u8-to-a-string
    let stdout = std::str::from_utf8(&assert.get_output().stdout).unwrap();

    let num_matches = &stdout.lines().count();

    ensure!(
        num_matches == &expected_num_matches,
        "Unexpected number of matches [{}]: {}",
        num_matches,
        stdout
    );

    Ok(())
}

/// Handy utility method for printing out the current git status
#[allow(dead_code)]
fn debug_git_status(csv_path: &Path) -> Result<()> {
    let git_output = Command::new("/usr/local/bin//git")
        .arg("status")
        .current_dir(csv_path.parent().unwrap())
        .output()?;
    let git_stdout = std::str::from_utf8(&git_output.stdout).unwrap();
    println!("GIT STATUS =\n{git_stdout}");
    Ok(())
}

/// Handy utility method for printing out the current git status
#[allow(dead_code)]
fn debug_csv_file(csv_path: &Path) -> Result<()> {
    let contents = std::fs::read_to_string(csv_path)?;
    println!("CSV FILE CONTENTS =\n{contents}");
    Ok(())
}
