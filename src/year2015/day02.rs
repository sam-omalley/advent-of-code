pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> u64 {
    let mut paper = 0;
    for line in input.lines() {
        if line.contains("x") {
            paper += calculate_paper(line);
        }
    }

    paper
}

pub fn part2(input: &str) -> u64 {
    let mut ribbon = 0;
    for line in input.lines() {
        if line.contains("x") {
            ribbon += calculate_ribbon(line);
        }
    }

    ribbon
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
        assert_eq!(part1("2x3x4"), 58);
        assert_eq!(part1("1x1x10"), 43);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part2("2x3x4"), 34);
        assert_eq!(part2("1x1x10"), 14);
    }
}
