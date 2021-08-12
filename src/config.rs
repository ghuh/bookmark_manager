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
    #[clap(name = "add", alias = "a")]
    Add(Add),
    #[clap(name = "search", alias = "s")]
    Search(Search),
}

#[derive(Debug, Clap, Validate)]
pub struct Add {
    #[validate(url)]
    pub url: String,
    pub description: String,

    #[clap(short, long = "tag")]
    pub tags: Vec<String>,
}

#[derive(Debug, Clap)]
pub struct Search {
    pub regex: String,

    #[clap(short, long = "tag")]
    pub tags: Vec<String>,
}
