aoc_2015::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let mut counter = 0;
    for char in input.trim().chars() {
        counter += if char == '(' {
            1
        } else if char == ')' {
            -1
        } else {
            0
        };
    }

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut counter: i64 = 0;
    for (idx, char) in input.trim().chars().enumerate() {
        counter += if char == '(' {
            1
        } else if char == ')' {
            -1
        } else {
            0
        };

        if counter == -1 {
            return Some(idx as u64 + 1);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floors() {
        assert_eq!(part_one("(())"), Some(0));
        assert_eq!(part_one("()()"), Some(0));
        assert_eq!(part_one("((("), Some(3));
        assert_eq!(part_one("(()(()("), Some(3));
        assert_eq!(part_one("))((((("), Some(3));
        assert_eq!(part_one("())"), Some(-1));
        assert_eq!(part_one("))("), Some(-1));
        assert_eq!(part_one(")))"), Some(-3));
        assert_eq!(part_one(")())())"), Some(-3));

        assert_eq!(part_two(")"), Some(1));
        assert_eq!(part_two("()())"), Some(5));
    }
}
