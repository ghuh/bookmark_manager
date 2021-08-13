use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use anyhow::{Result, Context, ensure};
use tempfile::{tempdir, TempDir};
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::path::PathBuf;

const HEADER_ROW: &str = "URL|DESCRIPTION|TAGS";

#[test]
fn invalid_command() -> Result<()> {
    let (_csv_dir, _csv_path, mut cmd) = setup(None)?;

    cmd.arg("stuff");
    cmd.assert()
        .failure();

    Ok(())
}

#[test]
fn create_csv_with_headers_if_not_exist() -> Result<()> {
    let (_csv_dir, csv_path, mut cmd) = setup(None)?;

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
/// If contents are provided, a header will be prepended and the CSV file will be populated.
fn setup(csv_contents: Option<CsvContents>) -> Result<(TempDir, PathBuf, Command)> {
    let dir = tempdir()?;
    let csv_path = dir.path().join("tmp.csv");

    if let Some(contents) = csv_contents {
        let mut file = File::create(&csv_path.as_path()).context("Couldn't create CSV file")?;
        writeln!(file, "{}", HEADER_ROW).context("Couldn't write headers to new CSV file")?;
        for row in contents.rows {
            writeln!(file, "{}|{}|{}", row.url, row.description, row.tags.join(","))
                .context("Couldn't write contents to new CSV file")?;
        }
    }

    let mut cmd = Command::cargo_bin("bm")?;

    cmd.env("BOOKMARK_MANAGER_CSV", csv_path.to_str().unwrap());

    Ok((dir, csv_path, cmd))
}

struct CsvContents {
    rows: Vec<Row>,
}

struct Row {
    url: String,
    description: String,
    tags: Vec<String>,
}

impl CsvContents {
    fn new() -> Self {
        Self { rows: Vec::new() }
    }

    fn add(&mut self, url: String, description: String, tags: Vec<String>) {
        self.rows.push( Row { url, description, tags } );
    }
}