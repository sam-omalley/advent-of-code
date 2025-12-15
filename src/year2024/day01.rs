aoc_2024::solution!(1);
use std::collections::HashMap;
use std::iter::zip;

pub fn part_one(input: &str) -> Option<i32> {
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

    let mut total = 0;
    for (a, b) in zip(&list_a, &list_b) {
        total += i32::abs(b - a);
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<i32> {
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

    let mut total = 0;
    for a in list_a {
        total += a * count_b.get(&a).unwrap_or(&0);
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
