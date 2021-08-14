use anyhow::{ensure, Result, Context};
use regex::Regex;

use crate::config::Search;
use crate::csv::{Line, CsvLineReader};
use crate::cli_output::search_result_output::{TextPart, SearchResultOutput, MatchedBookmark};
use std::collections::HashMap;

pub fn search(search_opts: &Search, csv: &String) -> Result<()> {
    // Make sure either REGEX or at least one tag
    ensure!(search_opts.regex.is_some() || !search_opts.tags.is_empty(), "Either a REGEX or a tag is required");

    // Only compile the regex once
    let re = match &search_opts.regex {
        Some(regex) => Some(Regex::new(regex.as_str()).context("Invalid REGEX")?),
        None => None
    };

    let mut out = SearchResultOutput::new();
    let reader = CsvLineReader::new(csv)?;

    for line in reader {
        if let Some(m) = match_line(&re, &search_opts.tags, line?) {
            out.add_matched_bookmark(m);
        }
    }

    // For formatting purposes the output is stored in memory until the search is complete. Print to console now
    out.print();

    Ok(())
}

fn match_line(re: &Option<Regex>, search_tags: &Vec<String>, line: Line) -> Option<MatchedBookmark> {
    let url = line.url.as_str();
    let description = line.description.as_str();

    let tag_lookup = line.tags.iter().map(|tag| (tag.to_lowercase(), 1)).collect::<HashMap<String, _>>();

    // Make sure the line has all tags
    // https://stackoverflow.com/a/64227550
    if !search_tags.iter().all(|tag| tag_lookup.contains_key(&tag.to_lowercase())) {
        return None;
    }

    // If there are tags, they matched. Then, if there is a regex, it must match as well
    if let Some(regex) = &re {
        let (url_is_match, url) = wrap_matches(regex, url);
        let (desc_is_match, description) = wrap_matches(regex, description);

        if url_is_match || desc_is_match {
            return Some(MatchedBookmark::new(
                url,
                description,
                line.tags,
            ));
        }
    }
    // There is no regex, there are tags and they matched
    else {
        return Some(
            MatchedBookmark::new_tags_only(
                url,
                description,
                line.tags,
            )
        );
    }

    None
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

#[cfg(test)]
mod tests {
    use crate::commands::search::match_line;
    use regex::Regex;
    use crate::csv::Line;
    use crate::cli_output::search_result_output::{TextPart, MatchedBookmark};

    #[test]
    fn no_match() {
        let m = match_line(
            &regex_from_str("what"),
            &Vec::new(),
            Line {
                url: String::from("https://google.com"),
                description: String::from("more than one"),
                tags: Vec::new(),
            },
        );

        assert!(m.is_none());
    }

    #[test]
    fn single_word_description() {
        let m = match_line(
            &regex_from_str("Hi"),
            &Vec::new(),
            Line {
                url: String::from("https://google.com"),
                description: String::from("Hi there"),
                tags: Vec::new(),
            },
        );

        single_matched_description(m, "Hi");
    }

    #[test]
    fn regex_description() {
        let m = match_line(
            &regex_from_str("t.e"),
            &Vec::new(),
            Line {
                url: String::from("https://google.com"),
                description: String::from("Hi there"),
                tags: Vec::new(),
            },
        );

        single_matched_description(m, "the");
    }

    #[test]
    fn multi_word_description() {
        let m = match_line(
            &regex_from_str("more than"),
            &Vec::new(),
            Line {
                url: String::from("https://google.com"),
                description: String::from("more than one"),
                tags: Vec::new(),
            },
        );

        single_matched_description(m, "more than");
    }

    #[test]
    fn single_word_url() { // Multi word doesn't make sense for a URL
        let m = match_line(
            &regex_from_str("google"),
            &Vec::new(),
            Line {
                url: String::from("https://google.com"),
                description: String::from("more than one"),
                tags: Vec::new(),
            },
        );

        single_matched_url(m, "google");
    }

    #[test]
    fn regex_url() { // Multi word doesn't make sense for a URL
        let m = match_line(
            &regex_from_str("g..g"),
            &Vec::new(),
            Line {
                url: String::from("https://google.com"),
                description: String::from("more than one"),
                tags: Vec::new(),
            },
        );

        single_matched_url(m, "goog");
    }

    #[test]
    fn tags_only() { // Multi word doesn't make sense for a URL
        let m = match_line(
            &None,
            &vec![String::from("Tag1")],
            Line {
                url: String::from("https://google.com"),
                description: String::from("more than one"),
                tags: vec![String::from("Tag1")],
            },
        );

        assert!(m.is_some());
        let m = m.unwrap();
        assert!(get_matched_parts(&m.url).is_empty());
        assert!(get_matched_parts(&m.description).is_empty());
    }

    #[test]
    fn case_insensitive_tags() { // Multi word doesn't make sense for a URL
        let m = match_line(
            &None,
            &vec![String::from("tag1")],
            Line {
                url: String::from("https://google.com"),
                description: String::from("more than one"),
                tags: vec![String::from("Tag1")],
            },
        );

        assert!(m.is_some());
        let m = m.unwrap();
        assert!(get_matched_parts(&m.url).is_empty());
        assert!(get_matched_parts(&m.description).is_empty());
    }

    #[test]
    fn tags_and_url_match() { // Multi word doesn't make sense for a URL
        let m = match_line(
            &regex_from_str("g..g"),
            &vec![String::from("tag1")],
            Line {
                url: String::from("https://google.com"),
                description: String::from("more than one"),
                tags: vec![String::from("Tag1")],
            },
        );

        single_matched_url(m, "goog");
    }

    #[test]
    fn url_match_tags_do_not() { // Multi word doesn't make sense for a URL
        let m = match_line(
            &regex_from_str("g..g"),
            &vec![String::from("what")],
            Line {
                url: String::from("https://google.com"),
                description: String::from("more than one"),
                tags: vec![String::from("Tag1")],
            },
        );

        assert!(m.is_none());
    }

    fn single_matched_description(m: Option<MatchedBookmark>, expected_text: &str) {
        assert!(m.is_some());
        let matched_text = get_matched_parts(&m.unwrap().description);
        assert_eq!(matched_text.len(), 1);
        assert_eq!(matched_text.get(0).unwrap().text(), expected_text);
    }

    fn single_matched_url(m: Option<MatchedBookmark>, expected_text: &str) {
        assert!(m.is_some());
        let matched_text = get_matched_parts(&m.unwrap().url);
        assert_eq!(matched_text.len(), 1);
        assert_eq!(matched_text.get(0).unwrap().text(), expected_text);
    }

    fn get_matched_parts(parts: &Vec<TextPart>) -> Vec<TextPart> {
        parts.iter().filter_map(
            |part| match part {
                TextPart::MatchedText(v) => Some(TextPart::MatchedText(v.clone())),
                _ => None,
            }
        ).collect::<Vec<TextPart>>()
    }

    fn regex_from_str(regex: &str) -> Option<Regex> {
        Some(Regex::new(regex).unwrap())
    }
}
