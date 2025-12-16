use std::collections::HashMap;
use std::iter::zip;

#[derive(Default)]
pub struct Input {
    pub list_a: Vec<i32>,
    pub list_b: Vec<i32>,
    pub count_b: HashMap<i32, i32>,
}

pub fn parse(input: &str) -> Input {
    let mut list_a: Vec<i32> = vec![];
    let mut list_b: Vec<i32> = vec![];
    let mut count_b: HashMap<i32, i32> = Default::default();
    for line in input.lines() {
        let mut line = line.split_whitespace();
        let a = line.next().unwrap_or("0").parse().unwrap();
        let b = line.next().unwrap_or("0").parse().unwrap();

        list_a.push(a);
        list_b.push(b);
        let val = count_b.entry(b).or_insert(0);
        *val += 1;
    }

    list_a.sort();
    list_b.sort();

    Input {
        list_a,
        list_b,
        count_b,
    }
}

pub fn part1(input: &Input) -> i32 {
    let mut total = 0;
    for (a, b) in zip(&input.list_a, &input.list_b) {
        total += i32::abs(b - a);
    }

    total
}

pub fn part2(input: &Input) -> i32 {
    let mut total = 0;
    for a in &input.list_a {
        total += a * input.count_b.get(a).unwrap_or(&0);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test() {
        let input = parse(&template::read_file("examples", year!(2024), day!(1)));
        assert_eq!(part1(&input), 11);
        assert_eq!(part2(&input), 31);
    }
}
