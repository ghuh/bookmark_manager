use anyhow::Result;
use validator::Validate;

use crate::cli_output::utils::{print_success, exit_error};
use crate::config::Add;
use crate::csv::{CsvLineReader, CsvLineWriter};

pub fn add(add_opts: &Add, csv: &String) -> Result<()> {
    // Make sure Url is valid
    add_opts.validate()?;

    // Prevent duplicate bookmarks
    if url_exists(add_opts.url.as_str(), csv)? {
        exit_error(format!("{} has already been bookmarked", add_opts.url).as_str());
    }

    // Append bookmark to file
    let mut writer = CsvLineWriter::new(csv.as_str())?;
    writer.write_line(
        add_opts.url.as_str(),
        add_opts.description.as_str(),
        &add_opts.tags,
    )?;

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
