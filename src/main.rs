// Enable cargo build --fail-on-warnings
// https://www.reddit.com/r/rust/comments/8oz7md/make_cargo_fail_on_warning/e087nj8?utm_source=share&utm_medium=web2x&context=3
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

use std::env;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::Write;
use clap::Clap;
use validator::Validate;
use anyhow::{ensure, Result, Context};
use ansi_term::Colour::{Green, Blue};
use regex::Regex;

use config::{Command, Add, Search};
use format_output::FormatOutput;
use csv::CsvLineReader;
use crate::output_utils::exit_error;

mod config;
mod format_output;
mod csv;
mod commands;
mod output_utils;

const ENV_CSV: &str = "BOOKMARK_MANAGER_CSV";
const ORDERED_HEADERS: [&'static str; 3] = ["URL", "DESCRIPTION", "TAGS"];

fn main() -> Result<()> {
    let opt = config::Opts::parse();

    let csv = env::var(ENV_CSV)
        .expect(&*format!("Environmental variable {} must be set", ENV_CSV));
    create_csv(csv.as_str())?;

    match opt.cmd {
        Command::Add(add_opts) => add(&add_opts, &csv)?,
        Command::Search(search_opts) => search(&search_opts, &csv)?,
        Command::Tags(tags_opts) => commands::tags::tags(&tags_opts, &csv)?,
    }

    Ok(())
}

/// If the CSV already exists, do nothing.  Otherwise create it with headers
fn create_csv(csv_path: &str) -> Result<()> {
    let path = Path::new(csv_path);
    if !path.exists() {
        let mut file = File::create(path).context("Couldn't create CSV file")?;
        writeln!(file, "{}", ORDERED_HEADERS.join("|")).context("Couldn't write headers to new CSV file")?;
        output_utils::print_success("CSV file created")
    }

    Ok(())
}

fn add(add_opts: &Add, csv: &String) -> Result<()> {
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
    println!("{}", Green.paint("Bookmark added"));
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

fn search(search_opts: &Search, csv: &String) -> Result<()> {
    // Make sure either REGEX or at least one tag
    ensure!(search_opts.regex.is_some() || !search_opts.tags.is_empty(), "Either a REGEX or a tag is required");

    let re = match &search_opts.regex {
        Some(regex) => Some(Regex::new(regex.as_str()).context("Invalid REGEX")?),
        None => None
    };

    let mut out = FormatOutput::new();

    let reader = CsvLineReader::new(csv)?;

    for line in reader {
        let line = line?;
        let url = line.url.as_str();
        let description = line.description.as_str();

        let mut tags = line.tags.iter().map(|tag| tag.to_lowercase()).collect::<Vec<String>>();

        // Sort tags case insensitively for output
        tags.sort();

        // Make sure the line has all tags
        // https://stackoverflow.com/a/64227550
        if !search_opts.tags.iter().all(|tag| tags.contains(&tag.to_lowercase())) {
            continue;
        }

        // If there are tags, they matched. Then, if there is a regex, it must match as well
        if let Some(regex) = &re {
            let (url_is_match, url, url_length) = wrap_matches(regex, url);
            let (desc_is_match, description, desc_length) = wrap_matches(regex, description);

            if url_is_match || desc_is_match {
                out.add_line(
                    url,
                    url_length,
                    description,
                    desc_length,
                    &tags,
                );
            }
        }
        // There is no regex, there are tags and they matched
        else {
            out.add_line(
                String::from(url),
                url.chars().count(),
                String::from(description),
                description.chars().count(),
                &tags,
            );
        }
    }

    out.print();

    Ok(())
}

// https://stackoverflow.com/a/56923739
/// Return if the regex was matched, the display string which will include any highlighting,
/// and the number of characters in the original string.
fn wrap_matches(re: &Regex, text: &str) -> (bool, String, usize) {
    let mut found: bool = false;
    let mut out: String = String::new();

    let mut last = 0;
    for mat in re.find_iter(text) {
        found = true;

        let start = mat.start();
        let end = mat.end();

        // Add everything up to the match
        if last != start {
            out.push_str(&text[last..start]);
        }

        // Add the match
        let colored_output = Blue.paint(&text[start..end]).to_string();
        out.push_str(&colored_output);

        last = end;
    }

    // Add any remaining text after last match
    // This will add the entire string if no matches found
    if last < text.len() {
        out.push_str(&text[last..])
    }

    (found, out, text.chars().count())
}