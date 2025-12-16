pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> i64 {
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

    counter
}

pub fn part2(input: &str) -> u64 {
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
            return idx as u64 + 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floors() {
        assert_eq!(part1("(())"), 0);
        assert_eq!(part1("()()"), 0);
        assert_eq!(part1("((("), 3);
        assert_eq!(part1("(()(()("), 3);
        assert_eq!(part1("))((((("), 3);
        assert_eq!(part1("())"), -1);
        assert_eq!(part1("))("), -1);
        assert_eq!(part1(")))"), -3);
        assert_eq!(part1(")())())"), -3);

        assert_eq!(part2(")"), 1);
        assert_eq!(part2("()())"), 5);
    }
}
