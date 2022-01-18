use anyhow::{ensure, Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use super::Line;

pub struct CsvLineReader {
    lines: Lines<BufReader<File>>,
}

impl CsvLineReader {
    pub fn new(csv: &str) -> Result<Self> {
        let file = File::open(csv).context("Could not open CSV file")?;
        let reader = BufReader::new(file);
        let mut iter = reader.lines();

        // Skip headers (i.e. first line)
        iter.next();

        Ok(Self { lines: iter })
    }
}

// https://doc.rust-lang.org/rust-by-example/trait/iter.html
impl Iterator for CsvLineReader {
    type Item = Result<Line>;

    fn next(&mut self) -> Option<Self::Item> {
        let line_result = match self.lines.next() {
            Some(result) => result,
            None => return None,
        };

        let line = match line_result.context("Could not read line from CSV") {
            Ok(line) => line,
            Err(e) => return Some(Err(e)),
        };
        let res: Result<Line> = parse_line(line.as_str());

        Some(res)
    }
}

fn parse_line(line: &str) -> Result<Line> {
    let line_parts = line.split('|').collect::<Vec<&str>>();
    ensure!(
        line_parts.len() == 3,
        format!("CSV line has more than 3 columns: {line}")
    );

    let url = String::from(line_parts[0]);
    let description = String::from(line_parts[1]);
    let tags = line_parts[2]
        .split(',')
        .map(String::from)
        .collect::<Vec<String>>();

    Ok(Line {
        url,
        description,
        tags,
    })
}

#[cfg(test)]
mod tests {
    use crate::csv::csv_line_reader::parse_line;

    #[test]
    fn invalid_line() {
        assert!(parse_line("four|pipes|in|line").is_err());
    }

    #[test]
    fn valid_line() {
        let url = "https://google.com";
        let description = "Google search engine";
        let tags = vec!["Search", "Engine"];
        let line_text = format!("{}|{}|{}", &url, &description, &tags.join(","));

        let line = parse_line(line_text.as_str()).unwrap();

        assert_eq!(line.url, url);
        assert_eq!(line.description, description);
        assert_eq!(line.tags, tags);
    }
}
