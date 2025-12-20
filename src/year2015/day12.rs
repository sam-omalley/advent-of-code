#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use regex::Regex;

#[must_use]
pub fn parse(input: &str) -> &str {
    input.trim()
}

#[must_use]
pub fn part1(input: &str) -> i64 {
    let re = Regex::new(r"(-?\d+)").unwrap();
    let mut total = 0;
    for (_, [num]) in re.captures_iter(input).map(|c| c.extract()) {
        total += num.parse::<i64>().unwrap();
    }
    total
}

#[must_use]
pub fn part2(_input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(part1("[1,2,3]"), 6);
        assert_eq!(part1(r#"{"a":2,"b":4}"#), 6);
        assert_eq!(part1(r#"[[[3]]]"#), 3);
        assert_eq!(part1(r#"{"a":{"b":4},"c":-1}"#), 3);
        assert_eq!(part1(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(part1(r#"[-1,{"a":1}]"#), 0);
        assert_eq!(part1(r#"[]"#), 0);
        assert_eq!(part1(r#"{}"#), 0);
    }
}
