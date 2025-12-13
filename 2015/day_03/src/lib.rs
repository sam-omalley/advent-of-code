use std::collections::HashMap;
use std::ops::{Add, AddAssign};

#[derive(Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2(pub i64, pub i64);

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1)
    }
}

pub fn part_1(contents: &str) -> usize {
    let mut visited_count = HashMap::<Vec2, usize>::new();

    let mut position = Vec2::default();
    *visited_count.entry(position).or_default() += 1;

    for char in contents.trim().chars() {
        let direction = match char {
            '^' => Vec2(0, 1),
            '>' => Vec2(1, 0),
            'v' => Vec2(0, -1),
            '<' => Vec2(-1, 0),
            _ => continue,
        };

        position += direction;
        *visited_count.entry(position).or_default() += 1;
    }

    visited_count.len()
}

pub fn part_2(contents: &str) -> usize {
    let mut visited_count = HashMap::<Vec2, usize>::new();

    let mut santa_position = Vec2::default();
    *visited_count.entry(santa_position).or_default() += 1;

    let mut robo_position = Vec2::default();
    *visited_count.entry(robo_position).or_default() += 1;

    for (idx, char) in contents.trim().chars().enumerate() {
        let direction = match char {
            '^' => Vec2(0, 1),
            '>' => Vec2(1, 0),
            'v' => Vec2(0, -1),
            '<' => Vec2(-1, 0),
            _ => continue,
        };

        if idx % 2 == 0 {
            santa_position += direction;
            *visited_count.entry(santa_position).or_default() += 1;
        } else {
            robo_position += direction;
            *visited_count.entry(robo_position).or_default() += 1;
        }
    }

    visited_count.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(part_1("^v"), 2);
        assert_eq!(part_1(">"), 2);
        assert_eq!(part_1("^>v<"), 4);
        assert_eq!(part_1("^v^v^v^v^v"), 2);

        assert_eq!(part_2("^v"), 3);
        assert_eq!(part_2("^>v<"), 3);
        assert_eq!(part_2("^v^v^v^v^v"), 11);
    }
}
