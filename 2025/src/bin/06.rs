advent_of_code::solution!(6);
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u64> {
    let mut inputs = Vec::<Vec<&str>>::new();
    let num_problems = input
        .lines()
        .map(|x| x.trim())
        .find(|x| !x.is_empty())
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .len();
    inputs.resize(num_problems, Vec::new());

    let mut operations = vec![Operations::None; num_problems];

    for line in input.lines().map(|x| x.trim()).filter(|x| !x.is_empty()) {
        for (index, token) in line.split_whitespace().enumerate() {
            if token.parse::<u64>().is_ok() {
                inputs[index].push(token);
            } else {
                operations[index] = token.parse().unwrap();
            }
        }
    }

    let mut total = 0;
    for (column, operation) in inputs.into_iter().zip(operations.iter()) {
        let res = operation.apply(column.iter().map(|x| x.parse().unwrap()));
        total += res;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let num_chars = input.lines().next().unwrap().chars().count();
    let num_lines = input.lines().count();

    let mut counter = 0;

    let mut total = 0;
    let mut values = Vec::<u64>::new();
    for idx in (0..num_chars).rev() {
        let mut val = String::new();
        for line in input.lines().take(num_lines - 1) {
            val += &line.chars().nth(idx).unwrap().to_string();
        }

        if val.trim().is_empty() {
            let op = input
                .lines()
                .nth(num_lines - 1)
                .unwrap()
                .split_whitespace()
                .rev()
                .nth(counter)
                .unwrap();
            let operation: Operations = op.parse().unwrap();

            let res = operation.apply(values.into_iter());
            total += res;

            values = Vec::<u64>::new();
            counter += 1;
        } else {
            let val: u64 = val.trim().parse().unwrap();
            values.push(val);
        }
    }

    let op = input
        .lines()
        .nth(num_lines - 1)
        .unwrap()
        .split_whitespace()
        .rev()
        .nth(counter)
        .unwrap();
    let operation: Operations = op.parse().unwrap();

    let res = operation.apply(values);
    total += res;

    Some(total)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operations {
    None,
    Multiply,
    Add,
}

impl Operations {
    pub fn apply<I>(&self, iter: I) -> u64
    where
        I: IntoIterator<Item = u64>,
    {
        match self {
            Operations::Multiply => iter.into_iter().product(),
            Operations::Add => iter.into_iter().sum(),
            Operations::None => 0,
        }
    }
}

impl FromStr for Operations {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "*" => Ok(Operations::Multiply),
            "+" => Ok(Operations::Add),
            _ => Err(format!("Unknown operation: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
