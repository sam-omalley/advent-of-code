use common::Config;
use day_02::*;
use std::env;
use std::error;
use std::fs;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)?;
    run(config)?;

    Ok(())
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let mut total = 0;

    for interval in contents.split(",") {
        let interval = interval.trim();
        if interval.is_empty() {
            continue;
        }

        let Some((a, b)) = interval.split_once("-") else {
            eprintln!("Error processing {interval}");
            std::process::exit(1);
        };

        let a: u64 = a.parse()?;
        let b: u64 = b.parse()?;
        total += check_all_naughty_pairs(a, b);
    }
    println!("Total: {total}");

    Ok(())
}
