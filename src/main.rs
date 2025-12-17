use aoc::util::ansi::*;
use std::fs::read_to_string;
use std::time::{Duration, Instant};

use aoc::template::commands::{download, read, scaffold};
use args::{AppArguments, parse};

mod args {
    use aoc::template::{Day, Year};
    use std::process;

    pub enum AppArguments {
        Download {
            year: Year,
            day: Day,
        },
        Read {
            year: Year,
            day: Day,
        },
        Scaffold {
            year: Year,
            day: Day,
            download: bool,
            overwrite: bool,
        },
        Solve {
            year: Option<Year>,
            day: Option<Day>,
        },
    }

    pub fn parse() -> Result<AppArguments, Box<dyn std::error::Error>> {
        let mut args = pico_args::Arguments::from_env();

        let app_args = match args.subcommand()?.as_deref() {
            Some("all") => AppArguments::Solve {
                year: None,
                day: None,
            },
            Some("download") => AppArguments::Download {
                year: args.free_from_str()?,
                day: args.free_from_str()?,
            },
            Some("read") => AppArguments::Read {
                year: args.free_from_str()?,
                day: args.free_from_str()?,
            },
            Some("scaffold") => AppArguments::Scaffold {
                year: args.free_from_str()?,
                day: args.free_from_str()?,
                download: args.contains("--download"),
                overwrite: args.contains("--overwrite"),
            },
            Some("solve") => AppArguments::Solve {
                year: Some(args.free_from_str()?),
                day: args.opt_free_from_str()?,
            },
            Some(x) => {
                eprintln!("Unknown command: {x}");
                process::exit(1);
            }
            None => {
                eprintln!("No command specified.");
                process::exit(1);
            }
        };

        let remaining = args.finish();
        if !remaining.is_empty() {
            eprintln!("Warning: unknown argument(s): {remaining:?}.");
        }

        Ok(app_args)
    }
}

fn main() {
    // Build list of all solutions.
    let solutions = [
        year2015(),
        year2024(),
        year2025(),
    ];

    match parse() {
        Err(err) => {
            eprintln!("Error: {err}");
            std::process::exit(1);
        }
        Ok(args) => match args {
            AppArguments::Download { year, day } => download::handle(year, day),
            AppArguments::Read { year, day } => read::handle(year, day),
            AppArguments::Scaffold {
                year,
                day,
                download,
                overwrite,
            } => {
                scaffold::handle(year, day, overwrite);
                if download {
                    download::handle(year, day);
                }
            }
            AppArguments::Solve { year, day } => {
                // Run selected solutions.
                let (stars, duration) = solutions
                    .iter()
                    .flatten()
                    .filter(|s| year.is_none_or(|y| y == s.year))
                    .filter(|s| day.is_none_or(|d| d == s.day))
                    .fold((0, Duration::ZERO), run_solution);

                println!("{BOLD}{YELLOW}⭐ {stars}{RESET}");
                println!("{BOLD}{WHITE}🕓 {} ms{RESET}", duration.as_millis());
            }
        },
    };
}

struct Solution {
    year: u32,
    day: u8,
    wrapper: fn(&str) -> (String, String),
}

fn run_solution((stars, duration): (u32, Duration), solution: &Solution) -> (u32, Duration) {
    let Solution { year, day, wrapper } = solution;
    let path = format!("data/inputs/year{year}/day{day:02}.txt");

    if let Ok(data) = read_to_string(&path) {
        let instant = Instant::now();
        let (part1, part2) = wrapper(&data);
        let elapsed = instant.elapsed();

        println!("{BOLD}{YELLOW}{year} Day {day}{RESET}");
        println!("    Part 1: {part1}");
        println!("    Part 2: {part2}");

        (stars + 2, duration + elapsed)
    } else {
        eprintln!("{BOLD}{RED}{year} Day {day}{RESET}");
        eprintln!("    Missing input!");
        eprintln!("    Place input file in {BOLD}{WHITE}{path}{RESET}");

        (stars, duration)
    }
}

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$(
                Solution {
                    year: stringify!($year).strip_prefix("year").unwrap().parse().unwrap(),
                    day: stringify!($day).strip_prefix("day").unwrap().parse().unwrap(),
                    wrapper: |data: &str| {
                        use aoc::$year::$day::*;

                        let input = parse(data);
                        let part1 = part1(&input).to_string();
                        let part2 = part2(&input).to_string();

                        (part1, part2)
                    }
                }
            ,)*]
        }
    }
}

run!(year2015
day01, day02, day03, day04, day05, day06
);

run!(year2024
    day01, day02
);

run!(year2025
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10, day11, day12
);
