#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use itertools::Itertools;

#[must_use]
pub fn parse(input: &str) -> &str {
    input.trim()
}

#[must_use]
pub fn part1(input: &str) -> usize {
    let mut prev = input.to_string();
    for _ in 0..40 {
        prev = next(&prev);
    }
    prev.len()
}

#[must_use]
pub fn next(input: &str) -> String {
    input
        .chars()
        .chunk_by(|&c| c)
        .into_iter()
        .map(|(digit, group)| group.count().to_string() + &digit.to_string())
        .collect()
}

#[must_use]
pub fn part2(input: &str) -> usize {
    let mut prev = input.to_string();
    for _ in 0..50 {
        prev = next(&prev);
    }
    prev.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1";
        assert_eq!(part1(&input), "312211");
    }
}
