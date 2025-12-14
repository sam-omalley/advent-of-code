aoc_2015::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut paper = 0;
    for line in input.lines() {
        if line.contains("x") {
            paper += calculate_paper(line);
        }
    }

    Some(paper)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ribbon = 0;
    for line in input.lines() {
        if line.contains("x") {
            ribbon += calculate_ribbon(line);
        }
    }

    Some(ribbon)
}

pub fn calculate_paper(s: &str) -> u64 {
    let mut iter = s.split("x");

    let l: u64 = iter.next().unwrap().parse().unwrap();
    let w: u64 = iter.next().unwrap().parse().unwrap();
    let h: u64 = iter.next().unwrap().parse().unwrap();

    (2 * l * w) + (2 * w * h) + (2 * h * l) + (l * w).min(w * h).min(h * l)
}

pub fn calculate_ribbon(s: &str) -> u64 {
    let mut iter = s.split("x");

    let l: u64 = iter.next().unwrap().parse().unwrap();
    let w: u64 = iter.next().unwrap().parse().unwrap();
    let h: u64 = iter.next().unwrap().parse().unwrap();

    (l + l + w + w).min(w + w + h + h).min(h + h + l + l) + (w * h * l)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("2x3x4"), Some(58));
        assert_eq!(part_one("1x1x10"), Some(43));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("2x3x4"), Some(34));
        assert_eq!(part_two("1x1x10"), Some(14));
    }
}
