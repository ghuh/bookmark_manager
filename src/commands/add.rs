use anyhow::{Result, Context};
use validator::Validate;
use std::io::Write;
use std::fs::OpenOptions;

use crate::output_utils::{print_success, exit_error};
use crate::config::Add;
use crate::csv::CsvLineReader;

pub fn add(add_opts: &Add, csv: &String) -> Result<()> {
    // Make sure Url is valid
    add_opts.validate()?;

    // Prevent duplicate bookmarks
    if url_exists(add_opts.url.as_str(), csv)? {
        exit_error(format!("{} has already been bookmarked", add_opts.url).as_str());
    }

    // Append bookmark to file
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&csv)
        .unwrap();
    writeln!(f, "{}|{}|{}", add_opts.url, add_opts.description, add_opts.tags.join(",")).context("Could not add bookmark")?;

    // Success
    print_success("Bookmark added");
    Ok(())
}

/// Check if URL already exists
fn url_exists(url: &str, csv: &String) -> Result<bool> {
    let reader = CsvLineReader::new(csv)?;

    for line in reader {
        if line?.url == url {
            return Ok(true);
        }
    }

    Ok(false)
}
