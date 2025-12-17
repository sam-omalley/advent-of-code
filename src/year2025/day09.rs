#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
use crate::util::vec2::{Rectangle, Vec2};
use itertools::Itertools;
use std::error::Error;

pub type Input = Tiles;

#[must_use]
pub fn parse(input: &str) -> Input {
    Tiles::parse(input).unwrap()
}

#[must_use]
pub fn part1(tiles: &Input) -> u64 {
    let mut smallest_p = Vec2::new(i64::MAX, i64::MAX);
    let mut biggest_p = Vec2::new(i64::MIN, i64::MIN);

    for p in &tiles.tiles {
        smallest_p = smallest_p.min(*p);
        biggest_p = biggest_p.max(*p);
    }

    let mut rectangles: Vec<Rectangle> = tiles
        .tiles
        .iter()
        .combinations(2)
        .map(|v| Rectangle::new(**v.first().unwrap(), **v.last().unwrap()))
        .collect();
    rectangles.sort_by_key(Rectangle::area);

    rectangles.iter().last().unwrap().area() as u64
}

#[must_use]
pub fn part2(tiles: &Input) -> u64 {
    let mut rectangles: Vec<Rectangle> = tiles
        .tiles
        .iter()
        .combinations(2)
        .map(|v| Rectangle::new(**v.first().unwrap(), **v.last().unwrap()))
        .collect();
    rectangles.sort_by_key(Rectangle::area);

    'outer: for r in rectangles.iter().rev() {
        for (p1, p2) in tiles
            .tiles
            .iter()
            .zip(tiles.tiles.iter().cycle().skip(1))
            .take(tiles.tiles.len())
        {
            if r.intersects(&Rectangle::new(*p1, *p2)) {
                continue 'outer;
            }
        }

        return r.area() as u64;
    }
    0
}

#[derive(Clone, Default, Debug)]
pub struct Tiles {
    pub tiles: Vec<Vec2>,
}

impl Tiles {
    pub fn parse(val: &str) -> Result<Self, Box<dyn Error>> {
        let mut tiles = Tiles::default();

        for line in val.lines().map(str::trim) {
            if !line.is_empty() {
                tiles.add_str(line).unwrap();
            }
        }
        Ok(tiles)
    }

    pub fn add_str(&mut self, val: &str) -> Result<(), Box<dyn Error>> {
        let parts: Vec<&str> = val.split(',').collect();

        if parts.len() == 2 {
            let p = Vec2 {
                x: parts[0].parse().unwrap(),
                y: parts[1].parse().unwrap(),
            };

            self.tiles.push(p);
        } else {
            println!("String does not contain exactly 2 comma-separated values");
        }

        Ok(())
    }

    pub fn print(&self, width: usize, height: usize, rectangle: Option<Rectangle>) {
        for row in 0..height {
            for col in 0..width {
                let p = Vec2 {
                    x: i64::try_from(col).ok().unwrap(),
                    y: i64::try_from(row).ok().unwrap(),
                };
                if let Some(r) = rectangle
                    && r.encloses(&p)
                {
                    print!("O");
                    continue;
                }

                if self.tiles.contains(&p) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_part_one() {
        let result = part1(&parse(&template::read_file(
            "examples",
            year!(2025),
            day!(9),
        )));
        assert_eq!(result, 50);
    }

    #[test]
    fn test_part_two() {
        let result = part2(&parse(&template::read_file(
            "examples",
            year!(2025),
            day!(9),
        )));
        assert_eq!(result, 24);
    }
}
