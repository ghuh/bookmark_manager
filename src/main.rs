use anyhow::Result;
use clap::Clap;
use std::env;

use crate::cli_output::utils::exit_error;
use crate::commands::add::add;
use crate::commands::search::search;
use crate::commands::tags::tags;
use config::Command;

mod cli_output;
mod commands;
mod config;
mod csv;
mod git;

const ENV_CSV: &str = "BOOKMARK_MANAGER_CSV";

fn main() -> Result<()> {
    let opt = config::Opts::parse();

    let csv = env::var(ENV_CSV).expect(&*format!("Environmental variable {} must be set", ENV_CSV));

    // The add command will create the CSV, the others will fail if it does not exit
    if let Command::Add(_) = opt.cmd {
    } else if !csv::csv_exists(csv.as_str()) {
        exit_error("The CSV file doesn't exist.  It will be created the first time you run 'add'.");
    }

    match opt.cmd {
        Command::Add(add_opts) => add(&add_opts, &csv)?,
        Command::Search(search_opts) => search(&search_opts, &csv)?,
        Command::Tags(tags_opts) => tags(&tags_opts, &csv)?,
    }

    Ok(())
}
