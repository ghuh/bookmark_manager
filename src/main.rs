// Enable cargo build --fail-on-warnings
// https://www.reddit.com/r/rust/comments/8oz7md/make_cargo_fail_on_warning/e087nj8?utm_source=share&utm_medium=web2x&context=3
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

use std::{env, process};
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{Write, BufReader, BufRead};
use clap::Clap;
use validator::Validate;
use anyhow::{ensure, Result, Context};
use ansi_term::Colour::{Red, Green, Blue};
use regex::Regex;

use config::{Command, Add, Search};
use format_output::FormatOutput;

mod config;
mod format_output;

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
    }

    Ok(())
}

/// If the CSV already exists, do nothing.  Otherwise create it with headers
fn create_csv(csv_path: &str) -> Result<()> {
    let path = Path::new(csv_path);
    if !path.exists() {
        let mut file = File::create(path).context("Couldn't create CSV file")?;
        writeln!(file, "{}", ORDERED_HEADERS.join("|")).context("Couldn't write headers to new CSV file")?;
        print_success("CSV file created")
    }

    Ok(())
}

fn print_success(msg: &str) {
    println!("{}", Green.paint(msg));
}

fn exit_error(msg: &str) {
    eprintln!("{}", Red.paint(msg));
    process::exit(1);
}

fn add(add_opts: &Add, csv: &String) -> Result<()> {
    // Make sure Url is valid
    add_opts.validate()?;

    // Prevent duplicate bookmarks
    if url_exists(add_opts.url.as_str(), csv)? {
        exit_error(format!("{} has already been book marked", add_opts.url).as_str());
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
    let f = File::open(&csv).context("Could not open CSV file")?;
    let reader = BufReader::new(f);

    for line_result in reader.lines() {
        let line = line_result.context("Could not read line from CSV")?;
        let line_parts = line.split("|").collect::<Vec<&str>>();
        ensure!(line_parts.len() == 3, format!("CSV line has more than 3 columns: {}", line));

        let line_url = line_parts[0];

        if line_url == url {
            return Ok(true);
        }
    }

    Ok(false)
}

fn search(search_opts: &Search, csv: &String) -> Result<()> {
    // Make sure either REGEX or at least one tag
    ensure!(search_opts.regex.is_some() || !search_opts.tags.is_empty(), "Either a REGEX or a tag is required");

    let f = File::open(&csv).context("Could not open CSV file")?;
    let reader = BufReader::new(f);

    let re = match &search_opts.regex {
        Some(regex) => Some(Regex::new(regex.as_str()).context("Invalid REGEX")?),
        None => None
    };

    let mut out = FormatOutput::new();

    let mut first_line = true;
    for line_result in reader.lines() {
        // Skip headers
        if first_line {
            first_line = false;
            continue;
        }

        let line = line_result.context("Could not read line from CSV")?;
        let line_parts = line.split("|").collect::<Vec<&str>>();
        ensure!(line_parts.len() == 3, format!("CSV line has more than 3 columns: {}", line));

        let url = line_parts[0];
        let description = line_parts[1];
        let tags_all = line_parts[2];

        let mut tags = tags_all.split(",").map(|tag| tag.to_lowercase()).collect::<Vec<String>>();

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