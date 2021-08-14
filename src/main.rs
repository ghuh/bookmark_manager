// Enable cargo build --fail-on-warnings
// https://www.reddit.com/r/rust/comments/8oz7md/make_cargo_fail_on_warning/e087nj8?utm_source=share&utm_medium=web2x&context=3
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use clap::Clap;
use anyhow::{Result, Context};

use config::Command;
use crate::commands::add::add;
use crate::commands::search::search;
use crate::commands::tags::tags;

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
        Command::Tags(tags_opts) => tags(&tags_opts, &csv)?,
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