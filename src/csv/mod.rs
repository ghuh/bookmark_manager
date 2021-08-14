pub mod csv_line_reader;
pub mod csv_line_writer;

use std::path::Path;
use std::fs::File;
use std::io::Write;
use anyhow::{Result, Context};

pub use csv_line_reader::CsvLineReader;
pub use csv_line_writer::CsvLineWriter;
use crate::output_utils;

const ORDERED_HEADERS: [&'static str; 3] = ["URL", "DESCRIPTION", "TAGS"];

/// If the CSV already exists, do nothing.  Otherwise create it with headers
pub fn create_csv(csv_path: &str) -> Result<()> {
    let path = Path::new(csv_path);
    if !path.exists() {
        let mut file = File::create(path).context("Couldn't create CSV file")?;
        writeln!(file, "{}", ORDERED_HEADERS.join("|")).context("Couldn't write headers to new CSV file")?;
        output_utils::print_success("CSV file created")
    }

    Ok(())
}
