use std::{
    fs::{File, OpenOptions, create_dir_all},
    path::Path,
    io::Write,
    process,
};

use crate::template::{Year, Day};

const MODULE_TEMPLATE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/template.txt"));

fn safe_create_file(path: &str, overwrite: bool) -> Result<File, std::io::Error> {
    let path = Path::new(path);
    let mut file = OpenOptions::new();
    if overwrite {
        file.create(true);
    } else {
        file.create_new(true);
    }

    // Create parent directories if needed.
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }

    file.truncate(true).write(true).open(path)
}

pub fn handle(year: Year, day: Day, overwrite: bool) {
    let input_path = format!("data/inputs/year{year}/day{day}.txt");
    let example_path = format!("data/examples/year{year}/day{day}.txt");
    let puzzle_path = format!("data/puzzles/year{year}/day{day}.md");
    let module_path = format!("src/year{year}/day{day}.rs");

    let mut file = match safe_create_file(&module_path, overwrite) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create module file: {e}");
            process::exit(1);
        }
    };

    match file.write_all(
        MODULE_TEMPLATE
            .replace("%YEAR%", &year.into_inner().to_string())
            .replace("%DAY%", &day.into_inner().to_string())
            .as_bytes(),
    ) {
        Ok(()) => {
            println!("Created module file \"{}\"", &module_path);
        }
        Err(e) => {
            eprintln!("Failed to write module contents: {e}");
            process::exit(1);
        }
    }

    match safe_create_file(&input_path, overwrite) {
        Ok(_) => {
            println!("Created empty input file \"{}\"", &input_path);
        }
        Err(e) => {
            eprintln!("Failed to create input file: {e}");
            process::exit(1);
        }
    }

    match safe_create_file(&example_path, overwrite) {
        Ok(_) => {
            println!("Created empty example file \"{}\"", &example_path);
        }
        Err(e) => {
            eprintln!("Failed to create example file: {e}");
            process::exit(1);
        }
    }

    match safe_create_file(&puzzle_path, overwrite) {
        Ok(_) => {
            println!("Created empty puzzle file \"{}\"", &puzzle_path);
        }
        Err(e) => {
            eprintln!("Failed to create puzzle file: {e}");
            process::exit(1);
        }
    }

    println!("---");
    println!("🎄 Type `cargo solve {year}::{day}` to run your solution.");
}
