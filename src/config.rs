use clap::Clap;

#[derive(Debug, Clap)]
#[clap(name = "bm", about = "Bookmark Manager CLI")]
pub struct Opts {
    #[clap(subcommand)]
    cmd: Command,
}

// https://github.com/TeXitoi/structopt/blob/master/examples/enum_tuple.rs
#[derive(Debug, Clap)]
pub enum Command {
    #[clap(name = "add", alias = "a")]
    Add(Add),
    #[clap(name = "search", alias = "s")]
    Search(Search),
}

#[derive(Debug, Clap)]
pub struct Add {
    pub url: String,
    pub description: String,

    #[clap(short, long = "tag")]
    tags: Vec<String>,
}

#[derive(Debug, Clap)]
pub struct Search {
    pub query: String,

    #[clap(short, long = "tag")]
    tags: Vec<String>,
}
