use common::Config;
use day_02_2015::*;
use std::{env, error, fs};

fn main() -> Result<(), Box<dyn error::Error>> {
    let config = Config::build(env::args())?;
    run(config)?;

    Ok(())
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let mut paper = 0;
    let mut ribbon = 0;
    for line in contents.lines() {
        if line.contains("x") {
            paper += calculate_paper(line);
            ribbon += calculate_ribbon(line);
        }
    }

    println!("Part 1: {paper}");
    println!("Part 1: {ribbon}");

    Ok(())
}
