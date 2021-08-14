use anyhow::{ensure, Result, Context};
use regex::Regex;

use crate::config::Search;
use crate::csv::CsvLineReader;
use crate::cli_output::search_result_output::{TextPart, SearchResultOutput};
use std::collections::HashMap;

pub fn search(search_opts: &Search, csv: &String) -> Result<()> {
    // Make sure either REGEX or at least one tag
    ensure!(search_opts.regex.is_some() || !search_opts.tags.is_empty(), "Either a REGEX or a tag is required");

    let re = match &search_opts.regex {
        Some(regex) => Some(Regex::new(regex.as_str()).context("Invalid REGEX")?),
        None => None
    };

    let mut out = SearchResultOutput::new();

    let reader = CsvLineReader::new(csv)?;

    for line in reader {
        let line = line?;
        let url = line.url.as_str();
        let description = line.description.as_str();

        let tag_lookup = line.tags.iter().map(|tag| (tag.to_lowercase(), 1)).collect::<HashMap<String, _>>();

        // Make sure the line has all tags
        // https://stackoverflow.com/a/64227550
        if !search_opts.tags.iter().all(|tag| tag_lookup.contains_key(&tag.to_lowercase())) {
            continue;
        }

        // Sort tags case insensitively for output, but display in their original case
        let mut output_tags = line.tags.clone();
        output_tags.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        // If there are tags, they matched. Then, if there is a regex, it must match as well
        if let Some(regex) = &re {
            let (url_is_match, url) = wrap_matches(regex, url);
            let (desc_is_match, description) = wrap_matches(regex, description);

            if url_is_match || desc_is_match {
                out.add_matched_bookmark(
                    url,
                    description,
                    output_tags,
                );
            }
        }
        // There is no regex, there are tags and they matched
        else {
            out.add_tags_only_matched_bookmark(
                url,
                description,
                output_tags,
            );
        }
    }

    out.print();

    Ok(())
}

// https://stackoverflow.com/a/56923739
/// Return if the regex was matched, the display string which will include any highlighting,
/// and the number of characters in the original string.
fn wrap_matches(re: &Regex, text: &str) -> (bool, Vec<TextPart>) {
    let mut found: bool = false;
    let mut parts: Vec<TextPart> = Vec::new();

    let mut last = 0;
    for mat in re.find_iter(text) {
        found = true;

        let start = mat.start();
        let end = mat.end();

        // Add everything up to the match
        if last != start {
            parts.push(TextPart::Text(String::from(&text[last..start])));
        }

        // Add the match
        parts.push(TextPart::MatchedText(String::from(&text[start..end])));

        last = end;
    }

    // Add any remaining text after last match
    // This will add the entire string if no matches found
    if last < text.len() {
        parts.push(TextPart::Text(String::from(&text[last..])));
    }

    (found, parts)
}
