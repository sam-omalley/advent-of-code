#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use fancy_regex::{Captures, Regex};

#[must_use]
pub fn parse(input: &str) -> &str {
    input.trim()
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let code_length = line.len();
        let re = Regex::new(r"\\(x[0-9A-Fa-f]{2})").unwrap();

        let decode = re
            .replace_all(
                &line[1..line.len() - 1]
                    .replace(r"\\", r"\")
                    .replace(r#"\""#, r#"""#),
                |_: &Captures| "a",
            )
            .to_string();
        total += code_length - decode.to_ascii_lowercase().len();
    }

    total
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut total = 0;
    for line in input.lines() {
        let diff = 2 + line.escape_default().to_string().len() - line.len();
        println!("{line} -> {} = {diff}", line.escape_default());
        total += diff;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test() {
        let input = &template::read_file("examples", year!(2015), day!(8));
        assert_eq!(part1(&input), 12);
        assert_eq!(part2(&input), 19);
    }
}
