use anyhow::Result;
use validator::Validate;

use crate::cli_output::utils::{exit_error, print_success};
use crate::config::Add;
use crate::csv::{CsvLineReader, CsvLineWriter, create_csv};
use crate::git::Git;

pub fn add(add_opts: &Add, csv: &str) -> Result<()> {
    // Make sure Url is valid
    add_opts.validate()?;

    // Open git repo unless user doesn't want to commit changes
    let git = match add_opts.commit {
        false => None,
        true => Git::new(csv),
    };

    // Make sure there aren't any uncommitted changes to the git repo before making any additional changes
    if let Some(git) = &git {
        if !git.is_clean()? {
            exit_error("Git repo has uncommitted changes");
        }
    }

    let created = create_csv(csv)?;

    // Prevent duplicate bookmarks (only on pre-existing files)
    if !created && url_exists(add_opts.url.as_str(), csv)? {
        exit_error(format!("{} has already been bookmarked", add_opts.url).as_str());
    }

    // Append bookmark to file
    let mut writer = CsvLineWriter::new(csv)?;
    writer.write_line(
        add_opts.url.as_str(),
        add_opts.description.as_str(),
        &add_opts.tags,
    )?;

    if let Some(git) = &git {
        git.add_and_commit_bookmark(add_opts.url.as_str(), add_opts.description.as_str())?;
    }

    // Success
    match &git {
        Some(_) => print_success("Bookmark added and committed to git"),
        None => print_success("Bookmark added"),
    }
    Ok(())
}

/// Check if URL already exists
fn url_exists(url: &str, csv: &str) -> Result<bool> {
    let reader = CsvLineReader::new(csv)?;

    for line in reader {
        if line?.url == url {
            return Ok(true);
        }
    }

    Ok(false)
}
