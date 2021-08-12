// Enable cargo build --fail-on-warnings
// https://www.reddit.com/r/rust/comments/8oz7md/make_cargo_fail_on_warning/e087nj8?utm_source=share&utm_medium=web2x&context=3
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

use std::env;
use clap::Clap;

mod config;

const ENV_CSV: &str = "BOOKMARK_MANAGER_CSV";

fn main() {
    let opt = config::Opts::parse();
    let csv = env::var(ENV_CSV).expect(&*format!("Environmental variable {}", ENV_CSV));

    println!("Options {:?}", opt);
    println!("ENV {:?}", csv);
}
