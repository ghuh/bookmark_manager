use ansi_term::Colour::{Green, Red, Yellow};
use std::process;

pub fn print_success(msg: &str) {
    println!("{}", Green.paint(msg));
}

pub fn print_warning(msg: &str) {
    println!("{}", Yellow.paint(msg));
}

pub fn exit_error(msg: &str) {
    eprintln!("{}", Red.paint(msg));
    process::exit(1);
}
