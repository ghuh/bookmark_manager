use std::process;
use ansi_term::Colour::{Red, Green};

pub fn print_success(msg: &str) {
    println!("{}", Green.paint(msg));
}

pub fn exit_error(msg: &str) {
    eprintln!("{}", Red.paint(msg));
    process::exit(1);
}
