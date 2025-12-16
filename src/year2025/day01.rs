pub type Input = Vec<Move>;

pub enum Move {
    Left(i32),
    Right(i32),
}

impl Move {
    pub fn parse(code: &str) -> Self {
        let code = code.trim();

        let delta = code[1..].parse::<i32>().unwrap();
        if code.starts_with("L") {
            Move::Left(delta)
        } else {
            Move::Right(delta)
        }
    }

    pub fn apply(&self, position: i32) -> i32 {
        match *self {
            Move::Left(n) => position - n,
            Move::Right(n) => position + n,
        }
    }
}

pub fn get_num_spins(start: i32, end: i32) -> u64 {
    let min = start.min(end);
    let max = start.max(end);
    // Do not count the final position as a spin.
    let correction = if end % 100 == 0 { -1 } else { 0 };
    (max.div_euclid(100) - (min - 1).div_euclid(100) + correction)
        .try_into()
        .unwrap()
}

pub fn parse(input: &str) -> Input {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(Move::parse)
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut password: u64 = 0;
    let mut position: i32 = 50;

    for movement in input
    {
        let previous_position = position;
        position = movement.apply(previous_position);

        // Rust modulo keeps the sign of the input, so we need to use mathematical modulo.
        if position.rem_euclid(100) == 0 {
            password += 1;
        }
    }

    password
}

pub fn part2(input: &Input) -> u64 {
    let mut num_spins = 0;
    let mut position: i32 = 50;

    for movement in input
    {
        let previous_position = position;
        position = movement.apply(previous_position);

        num_spins += get_num_spins(previous_position, position);
    }

    num_spins
}

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn assert_mod100_eq(a: i32, b: i32) {
        assert_eq!(a.rem_euclid(100), b.rem_euclid(100));
    }

    #[test]
    fn test_dial() {
        assert_mod100_eq(82, Move::Left(68).apply(50));
        assert_mod100_eq(52, Move::Left(30).apply(82));
        assert_mod100_eq(0, Move::Right(48).apply(52));
        assert_mod100_eq(95, Move::Left(5).apply(0));
        assert_mod100_eq(55, Move::Right(60).apply(95));
        assert_mod100_eq(0, Move::Left(55).apply(55));
        assert_mod100_eq(99, Move::Left(1).apply(0));
        assert_mod100_eq(0, Move::Left(99).apply(99));
        assert_mod100_eq(14, Move::Right(14).apply(0));
        assert_mod100_eq(32, Move::Left(82).apply(14));
    }

    #[test]
    fn test_script() {
        let input = parse("
            L68
            L30
            R48
            L5
            R60
            L55
            L1
            L99
            R14
            L82");

        assert_eq!(3, part1(&input));
        assert_eq!(6, part2(&input));
    }

    #[test]
    fn num_spins() {
        assert_eq!(1, get_num_spins(50, -18));
        assert_eq!(0, get_num_spins(82, 52));
        assert_eq!(0, get_num_spins(52, 0));
        assert_eq!(10, get_num_spins(50, 1050));
    }
}
