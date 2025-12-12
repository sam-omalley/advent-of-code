use std::collections::HashMap;
use std::env;
use std::fs;
use std::iter::zip;

#[derive(Debug)]
enum MyError {
    CommandLine,
}

impl std::error::Error for MyError {}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("CommandLine error")
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ./app <filename>");
        return Err(Box::new(MyError::CommandLine));
    }
    let contents = fs::read_to_string(&args[1])?;

    let mut list_a: Vec<i32> = vec![];
    let mut list_b: Vec<i32> = vec![];
    let mut count_b: HashMap<i32, i32> = Default::default();
    for line in contents.lines() {
        let mut line = line.split_whitespace();
        let a = line.next().unwrap_or("0").parse()?;
        let b = line.next().unwrap_or("0").parse()?;

        list_a.push(a);
        list_b.push(b);
        let val = count_b.entry(b).or_insert(0);
        *val += 1;
    }

    list_a.sort();
    list_b.sort();

    let mut total = 0;
    for (a, b) in zip(&list_a, &list_b) {
        total += i32::abs(b - a);
    }
    println!("Total: {}", total);

    let mut total = 0;
    for a in list_a {
        total += a * count_b.get(&a).unwrap_or(&0);
    }
    println!("Similarity: {}", total);

    Ok(())
}
