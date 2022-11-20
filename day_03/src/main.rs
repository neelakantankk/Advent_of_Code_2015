use std::fs;
use std::process;

fn main() {
    let contents = fs::read_to_string("input").expect("Failed to read input file");

    if let Err(e) = day_03::run(contents) {
        eprintln!("Could not execute: {e}");
        process::exit(1);
    };
}
