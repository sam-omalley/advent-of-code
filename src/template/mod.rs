use std::{env, fs};

pub mod aoc_cli;
pub mod commands;

pub use day::*;
pub use year::*;

mod day;
mod year;

pub const ANSI_ITALIC: &str = "\x1b[3m";
pub const ANSI_BOLD: &str = "\x1b[1m";
pub const ANSI_RESET: &str = "\x1b[0m";

/// Helper function that reads a text file to a string.
#[must_use]
pub fn read_file(folder: &str, year: Year, day: Day) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(format!("{year}"))
        .join(format!("{day}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

/// Helper function that reads a text file to string, appending a part suffix. E.g. like `01-2.txt`.
#[must_use]
pub fn read_file_part(folder: &str, year: Year, day: Day, part: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd
        .join("data")
        .join(folder)
        .join(format!("{year}"))
        .join(format!("{day}-{part}.txt"));
    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}
