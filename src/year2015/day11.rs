#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

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

/*
>>> def p(idx):
...     res = ""
...     while idx > 0:
...         idx -= 1
...         res += l[idx % len(l)]
...         idx //= len(l)
...     return res[::-1]
 */

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
    for _ in 1..100 {
        println!("{}", iter.next().unwrap());
    }
    String::default()
}

#[must_use]
pub fn part2(_input: &Input) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_part1() {
        let input = parse(&template::read_file("examples", year!(2015), day!(11)));
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn test_part2() {
        let input = parse(&template::read_file("examples", year!(2015), day!(11)));
        assert_eq!(part2(&input), 0);
    }
}
