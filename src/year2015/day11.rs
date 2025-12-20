#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use itertools::Itertools;
use std::collections::HashSet;
use std::io::Read;

type Input = PasswordIterator;

#[derive(Clone)]
pub struct PasswordIterator {
    value: u64,
}

impl PasswordIterator {
    #[must_use]
    pub fn new(s: &str) -> Self {
        let s = s.trim();

        let mut total = 0;
        for (idx, b) in s.as_bytes().bytes().enumerate() {
            let val: u64 = (b.unwrap() - 97 + 1).into();
            total += val * 26_u64.pow((s.len() - idx - 1).try_into().unwrap());
        }

        Self { value: total }
    }
}

impl Iterator for PasswordIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.value += 1;

        let mut val = self.value;
        let mut res = String::default();

        while val > 0 {
            val -= 1;
            let ascii: u8 = 97 + (val % 26) as u8;
            res.push(ascii.into());
            val /= 26;
        }
        Some(res.chars().rev().collect())
    }
}

#[must_use]
pub fn parse(input: &str) -> Input {
    PasswordIterator::new(input)
}

#[must_use]
pub fn part1(input: &Input) -> String {
    let mut iter = input.clone();
    loop {
        let password = iter.next().unwrap();
        if check_password(&password) {
            return password;
        }
    }
}

fn check_password(password: &str) -> bool {
    has_no_forbidden_characters(password)
        && has_increasing_straight(password)
        && has_pairs(password)
}

#[must_use]
fn has_pairs(password: &str) -> bool {
    let pairs: HashSet<char> = password
        .chars()
        .tuple_windows()
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect();

    pairs.len() > 1
}

#[must_use]
fn has_no_forbidden_characters(password: &str) -> bool {
    !password.chars().any(|c| c == 'i' || c == 'o' || c == 'l')
}

#[must_use]
fn has_increasing_straight(password: &str) -> bool {
    let diff: Vec<i32> = password
        .as_bytes()
        .windows(2)
        .map(|w| i32::from(w[1]) - i32::from(w[0]))
        .collect();

    diff.windows(2).any(|w| w[0] == 1 && w[1] == 1)
}

#[must_use]
pub fn part2(_input: &Input) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let password = "abcdffaa";
        assert!(has_no_forbidden_characters(password));
        assert!(has_increasing_straight(password));
        assert!(has_pairs(password));
        assert!(check_password(password));
        //let input = parse("abcdefgh");
        //assert_eq!(part1(&input), "abcdffaa");

        let password = "ghjaabcc";
        assert!(has_no_forbidden_characters(password));
        assert!(has_increasing_straight(password));
        assert!(has_pairs(password));
        assert!(check_password(password));
        //let input = parse("ghijklmn");
        //assert_eq!(part1(&input), "ghjaabcc");
    }
}
