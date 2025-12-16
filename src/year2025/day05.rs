use itertools::Itertools;
use std::ops::RangeInclusive;

pub type Input = (Vec<RangeInclusive<u64>>, Vec<u64>);

pub fn parse(input: &str) -> Input {
    let mut values = Vec::<u64>::new();
    let mut intervals = Vec::<RangeInclusive<u64>>::new();
    for line in input.lines().map(|x| x.trim()).filter(|x| !x.is_empty()) {
        let mut iter = line.split("-");

        let first: u64 = iter.next().unwrap().parse().unwrap();
        let second = iter.next();

        match second {
            Some(val) => {
                let end: u64 = val.parse().unwrap();
                intervals.push(first..=end);
            }
            None => {
                values.push(first);
            }
        }
    }

    (squash_intervals(&intervals), values)
}

pub fn part1(input: &Input) -> u64 {
    count_members(&input.0, &input.1)
}

pub fn part2(input: &Input) -> u64 {
    count_possible_intervals(&input.0)
}

pub fn squash_intervals(intervals: &[RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
    let mut squashed = Vec::<RangeInclusive<u64>>::new();
    for interval in intervals.iter().sorted_by_key(|x| *x.start()) {
        if let Some(last_interval) = squashed.last_mut()
            && is_overlapping(interval, last_interval)
        {
            *last_interval = *last_interval.start()..=*interval.end().max(last_interval.end());
        } else {
            squashed.push(interval.clone());
        }
    }

    squashed
}

fn is_overlapping<T: Ord>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool {
    a.start() <= b.end() && b.start() <= a.end()
}

pub fn count_members(intervals: &Vec<RangeInclusive<u64>>, values: &Vec<u64>) -> u64 {
    let mut total = 0;
    for value in values {
        for interval in intervals {
            if interval.contains(value) {
                total += 1;
                break;
            }
        }
    }

    total
}

// Must be squashed intervals
pub fn count_possible_intervals(intervals: &Vec<RangeInclusive<u64>>) -> u64 {
    let mut total = 0;
    for interval in intervals {
        total += interval.end() - interval.start() + 1
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_part_one() {
        let input = parse(&template::read_file("examples", year!(2025), day!(5)));
        let result = part1(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_two() {
        let input = parse(&template::read_file("examples", year!(2025), day!(5)));
        let result = part2(&input);
        assert_eq!(result, 14);
    }

    #[test]
    fn interval_squashing() {
        let intervals = vec![1..=5, 7..=10];
        let overlapping_intervals = vec![3..=8, 1..=5, 7..=10];
        assert_eq!(intervals, squash_intervals(&intervals));
        assert_eq!(vec![1..=10], squash_intervals(&overlapping_intervals));
    }

    #[test]
    fn check() {
        let intervals = vec![3..=5, 10..=14, 16..=20, 12..=18];
        let values = vec![1, 5, 8, 11, 17, 32];

        assert_eq!(count_members(&intervals, &values), 3);
        assert_eq!(count_possible_intervals(&squash_intervals(&intervals)), 14);
    }
}
