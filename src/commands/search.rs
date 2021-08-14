use anyhow::{ensure, Result, Context};
use regex::Regex;
use ansi_term::Colour::{Blue};

use crate::config::Search;
use crate::csv::CsvLineReader;
use crate::format_output::FormatOutput;

pub fn search(search_opts: &Search, csv: &String) -> Result<()> {
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
