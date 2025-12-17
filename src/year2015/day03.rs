use std::collections::HashMap;
use crate::util::vec2::*;

pub type Input = Vec<Vec2>;

pub fn parse(input: &str) -> Input {
    let mut directions = Vec::default();
    for char in input.trim().chars() {
        let direction = match char {
            '^' => Vec2::new(0, 1),
            '>' => Vec2::new(1, 0),
            'v' => Vec2::new(0, -1),
            '<' => Vec2::new(-1, 0),
            _ => continue,
        };

        directions.push(direction);
    }

    directions
}

pub fn part1(directions: &Input) -> u64 {
    let mut visited_count = HashMap::<Vec2, usize>::new();

    let mut position = Vec2::default();
    *visited_count.entry(position).or_default() += 1;

    for direction in directions {
        position += *direction;
        *visited_count.entry(position).or_default() += 1;
    }

    visited_count.len() as u64
}

pub fn part2(directions: &Input) -> u64 {
    let mut visited_count = HashMap::<Vec2, usize>::new();

    let mut santa_position = Vec2::default();
    *visited_count.entry(santa_position).or_default() += 1;

    let mut robo_position = Vec2::default();
    *visited_count.entry(robo_position).or_default() += 1;

    for (idx, direction) in directions.iter().enumerate() {
        if idx % 2 == 0 {
            santa_position += *direction;
            *visited_count.entry(santa_position).or_default() += 1;
        } else {
            robo_position += *direction;
            *visited_count.entry(robo_position).or_default() += 1;
        }
    }

    visited_count.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part1(&parse("^v")), 2);
        assert_eq!(part1(&parse(">")), 2);
        assert_eq!(part1(&parse("^>v<")), 4);
        assert_eq!(part1(&parse("^v^v^v^v^v")), 2);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part2(&parse("^v")), 3);
        assert_eq!(part2(&parse("^>v<")), 3);
        assert_eq!(part2(&parse("^v^v^v^v^v")), 11);
    }
}
