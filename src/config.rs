use clap::Clap;
use validator::Validate;

#[derive(Debug, Clap)]
#[clap(name = "bm", about = "Bookmark Manager CLI")]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Command,
}

// subcommands: https://github.com/TeXitoi/structopt/blob/master/examples/enum_tuple.rs
// aliases: https://github.com/TeXitoi/structopt/blob/master/examples/subcommand_aliases.rs
#[derive(Debug, Clap)]
pub enum Command {
    /// Add a bookmark to the bookmarks file
    #[clap(name = "add", alias = "a")]
    Add(Add),

    /// Search for a bookmark
    #[clap(name = "search", alias = "s")]
    Search(Search),
}

#[derive(Debug, Clap, Validate)]
pub struct Add {
    /// URL to bookmark
    #[validate(url)]
    pub url: String,

    /// Description of the URL
    pub description: String,

    /// Tags to group bookmarks
    #[clap(short, long = "tag")]
    pub tags: Vec<String>,

    // https://github.com/TeXitoi/structopt/blob/master/examples/negative_flag.rs
    /// Turn off automatically committing bookmarks file if it is in a git repo
    #[clap(long = "no-commit", parse(from_flag = std::ops::Not::not))]
    pub commit: bool,
}

#[derive(Debug, Clap)]
pub struct Search {
    /// Perl style REGEX to run against bookmark URL and description.  Omit to do tags only search.
    pub regex: Option<String>,

    /// Only apply REGEX to bookmarks with the given tags (can be none)
    #[clap(short, long = "tag")]
    pub tags: Vec<String>,
}
