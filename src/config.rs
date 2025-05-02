use clap::Parser;
use validator::{Validate, ValidationError};

#[derive(Debug, Parser)]
#[clap(name = "bm", about = "Bookmark Manager CLI")]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Command,
}

// subcommands: https://github.com/TeXitoi/structopt/blob/master/examples/enum_tuple.rs
// aliases: https://github.com/TeXitoi/structopt/blob/master/examples/subcommand_aliases.rs
#[derive(Debug, Parser)]
pub enum Command {
    /// Add a bookmark to the bookmarks file
    #[clap(name = "add", alias = "a")]
    Add(Add),

    /// Search for a bookmark
    #[clap(name = "search", alias = "s")]
    Search(Search),

    /// List all tags
    #[clap(name = "tags", alias = "t")]
    Tags(Tags),
}

#[derive(Debug, Parser, Validate)]
pub struct Add {
    /// URL to bookmark
    #[validate(url, custom(function = "validate_no_pipe"))]
    pub url: String,

    /// Description of the URL
    #[validate(custom(function = "validate_no_pipe"))]
    pub description: String,

    /// Tags to group bookmarks
    #[clap(short, long = "tag")]
    #[validate(custom(function = "validate_tags"))]
    pub tags: Vec<String>,

    /// Turn off automatically committing bookmarks file if it is in a git repo
    #[clap(long = "no-commit", action = clap::ArgAction::SetFalse)]
    pub commit: bool,
}

#[derive(Debug, Parser)]
pub struct Search {
    /// Perl style REGEX to run against bookmark URL and description.  Omit to do tags only search.
    pub regex: Option<String>,

    /// Only apply REGEX to bookmarks with the given tags (can be none)
    #[clap(short, long = "tag")]
    pub tags: Vec<String>,
}

#[derive(Debug, Parser)]
pub struct Tags {
    /// Output tags in a machine-readable way. i.e. Every tag is on a new line.
    /// Defaults to false which outputs in a more human-readable way. i.e. Different capitalization of the same tag is output comma separated on the same line
    #[clap(long = "machine", action = clap::ArgAction::SetTrue)]
    pub machine: bool,
}

fn validate_tags(values: &[String]) -> std::result::Result<(), ValidationError> {
    for val in values {
        validate_no_pipe(val)?;
        validate_no_comma(val)?;
    }

    Ok(())
}

fn validate_no_comma(val: &str) -> std::result::Result<(), ValidationError> {
    if val.contains(',') {
        return Err(ValidationError::new("contains_comma"));
    }

    Ok(())
}

fn validate_no_pipe(val: &str) -> std::result::Result<(), ValidationError> {
    if val.contains('|') {
        return Err(ValidationError::new("contains_pipe"));
    }

    Ok(())
}

#[cfg(test)]
mod add_tests {
    use validator::Validate;

    use crate::config::Add;

    #[test]
    fn invalid_url() {
        let add_opts = Add {
            url: String::from("not_a_url"),
            description: String::from("description"),
            tags: Vec::new(),
            commit: true,
        };

        assert!(add_opts.validate().is_err());
    }

    #[test]
    fn pipe_in_url() {
        let add_opts = Add {
            url: String::from("https://wwww.go|ogle.com"),
            description: String::from("description"),
            tags: Vec::new(),
            commit: true,
        };

        assert!(add_opts.validate().is_err());
    }

    #[test]
    fn pipe_in_description() {
        let add_opts = Add {
            url: String::from("https://wwww.google.com"),
            description: String::from("descr|iption"),
            tags: Vec::new(),
            commit: true,
        };

        assert!(add_opts.validate().is_err());
    }

    #[test]
    fn pipe_in_tags() {
        let add_opts = Add {
            url: String::from("https://wwww.google.com"),
            description: String::from("description"),
            tags: vec![String::from("t|ag")],
            commit: true,
        };

        assert!(add_opts.validate().is_err());
    }

    #[test]
    fn comma_in_tags() {
        let add_opts = Add {
            url: String::from("https://wwww.google.com"),
            description: String::from("description"),
            tags: vec![String::from("t,ag")],
            commit: true,
        };

        assert!(add_opts.validate().is_err());
    }
}
