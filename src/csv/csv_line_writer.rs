use anyhow::{Context, Result};
use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct CsvLineWriter {
    file: File,
}

impl CsvLineWriter {
    pub fn new(csv: &str) -> Result<Self> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(csv)
            .context("Could not open CSV for writing")?;

        Ok(Self { file })
    }

    pub fn write_line(&mut self, url: &str, description: &str, tags: &[String]) -> Result<()> {
        writeln!(self.file, "{}|{}|{}", url, description, tags.join(","))
            .context("Could not add bookmark")?;
        Ok(())
    }
}
