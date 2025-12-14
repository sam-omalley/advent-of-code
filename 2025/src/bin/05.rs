advent_of_code::solution!(5);
use std::ops::RangeInclusive;

pub fn part_one(input: &str) -> Option<u64> {
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

    let intervals = squash_intervals(intervals);
    let count = count_members(&intervals, &values);

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
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

    let intervals = squash_intervals(intervals);
    let possible = count_possible_intervals(&intervals);

    Some(possible)
}

pub fn squash_intervals(mut intervals: Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    intervals.sort_by_key(|x| *x.start());

    let mut squashed = Vec::<RangeInclusive<u64>>::new();
    for interval in intervals {
        if let Some(last_interval) = squashed.last_mut()
            && is_overlapping(&interval, last_interval)
        {
            *last_interval = *last_interval.start()..=*interval.end().max(last_interval.end());
        } else {
            squashed.push(interval);
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

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn interval_squashing() {
        let intervals = vec![1..=5, 7..=10];
        let overlapping_intervals = vec![3..=8, 1..=5, 7..=10];
        assert_eq!(intervals, squash_intervals(intervals.clone()));
        assert_eq!(
            vec![1..=10],
            squash_intervals(overlapping_intervals.clone())
        );
    }

    #[test]
    fn check() {
        let intervals = vec![3..=5, 10..=14, 16..=20, 12..=18];
        let values = vec![1, 5, 8, 11, 17, 32];

        assert_eq!(count_members(&intervals, &values), 3);
        assert_eq!(count_possible_intervals(&squash_intervals(intervals)), 14);
    }
}
