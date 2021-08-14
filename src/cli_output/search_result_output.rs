use core::iter;
use ansi_term::Colour::{Blue};

pub enum TextPart {
    MatchedText(String),
    Text(String),
}

impl TextPart {
    /// The number of characters in this text part.
    pub fn len(&self) -> usize {
        self.text().chars().count()
    }

    /// The raw text
    pub fn text(&self) -> &String {
        match self {
            TextPart::MatchedText(val) => val,
            TextPart::Text(val) => val,
        }
    }

    /// The total number of characters in all the text parts in the vector.
    pub fn vec_len(parts: &Vec<TextPart>) -> usize {
        parts.iter().map(|part| part.len()).sum()
    }

    /// Highlighted the matched text
    pub fn pretty_string(parts: &Vec<TextPart>) -> String {
        let mut out = String::new();

        for part in parts {
            match part {
                TextPart::MatchedText(val) => out.push_str(Blue.paint(val).to_string().as_str()),
                TextPart::Text(val) => out.push_str(val.as_str()),
            };
        }

        out
    }
}

pub struct MatchedBookmark {
    pub url: Vec<TextPart>,
    pub description: Vec<TextPart>,
    pub tags: Vec<String>,
}

impl MatchedBookmark {
    pub fn new_tags_only(
        url: &str,
        description: &str,
        tags: Vec<String>,
    ) -> Self {
        MatchedBookmark::new(
            vec![TextPart::Text(String::from(url))],
            vec![TextPart::Text(String::from(description))],
            tags,
        )
    }

    pub fn new(
        url: Vec<TextPart>,
        description: Vec<TextPart>,
        mut tags: Vec<String>,
    ) -> Self {
        // Sort tags case insensitively for output, but display in their original case
        tags.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

        Self {
            url,
            description,
            tags,
        }
    }

    /// Number of characters in URL (without formatting)
    fn url_len(&self) -> usize {
        TextPart::vec_len(&self.url)
    }

    /// Formatted URL for displaying on the terminal
    fn url_pretty_string(&self) -> String {
        TextPart::pretty_string(&self.url)
    }

    /// Number of characters in description (without formatting)
    fn description_len(&self) -> usize {
        TextPart::vec_len(&self.description)
    }

    /// Formatted description for displaying on the terminal
    fn description_pretty_string(&self) -> String {
        TextPart::pretty_string(&self.description)
    }

    /// Formatted tags for displaying on the terminal
    fn tags_pretty_string(&self) -> String {
        self.tags.join(" | ")
    }
}

pub struct SearchResultOutput {
    url_max: usize,
    desc_max: usize,
    lines: Vec<MatchedBookmark>,
}

impl SearchResultOutput {
    pub fn new() -> Self {
        Self {
            url_max: 0,
            desc_max: 0,
            lines: Vec::new(),
        }
    }

    pub fn add_matched_bookmark(
        &mut self,
        matched_bookmark: MatchedBookmark,
    ) {
        let url_len = matched_bookmark.url_len();
        if url_len > self.url_max {
            self.url_max = url_len;
        }

        let desc_len = matched_bookmark.description_len();
        if desc_len > self.desc_max {
            self.desc_max = desc_len;
        }

        self.lines.push(matched_bookmark);
    }

    pub fn print(&self) {
        for line in &self.lines {
            // Can't use println formatting width because gets messed up by colored lines
            println!(
                "{}{} {}{} {}",
                line.url_pretty_string(),
                generate_padding(line.url_len(), self.url_max),
                line.description_pretty_string(),
                generate_padding(line.description_len(), self.desc_max),
                line.tags_pretty_string(),
            );
        }
    }
}

// Inspiration: https://docs.rs/crate/tabwriter/1.2.1/source/src/lib.rs
fn generate_padding(current_len: usize, pad_to: usize) -> String {
    iter::repeat(' ').take(pad_to - current_len).collect()
}

#[test]
fn test_generate_padding() {
    assert_eq!(generate_padding(7, 10), "   ");
}