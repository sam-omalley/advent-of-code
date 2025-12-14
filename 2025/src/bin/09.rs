advent_of_code::solution!(9);
use itertools::Itertools;
use std::error::Error;

pub fn part_one(input: &str) -> Option<u64> {
    let tiles = Tiles::parse(input).unwrap();

    let mut smallest_p = Point::new(i64::MAX, i64::MAX);
    let mut biggest_p = Point::new(i64::MIN, i64::MIN);

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
    rectangles.sort_by_key(|r| r.area());

    Some(rectangles.iter().last().unwrap().area() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let tiles = Tiles::parse(input).unwrap();

    let mut rectangles: Vec<Rectangle> = tiles
        .tiles
        .iter()
        .combinations(2)
        .map(|v| Rectangle::new(**v.first().unwrap(), **v.last().unwrap()))
        .collect();
    rectangles.sort_by_key(|r| r.area());

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

        return Some(r.area() as u64);
    }
    None
}

#[derive(Clone, Default, Debug)]
pub struct Tiles {
    pub tiles: Vec<Point>,
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
            let p = Point {
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
                let p = Point {
                    x: col as i64,
                    y: row as i64,
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

#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    pub fn new(p1: Point, p2: Point) -> Self {
        Rectangle { p1, p2 }
    }

    pub fn encloses(&self, p: &Point) -> bool {
        let min = self.min();
        let max = self.max();
        min.x <= p.x && p.x <= max.x && min.y <= p.y && p.y <= max.y
    }

    pub fn min(&self) -> Point {
        Point {
            x: self.p1.x.min(self.p2.x),
            y: self.p1.y.min(self.p2.y),
        }
    }

    pub fn max(&self) -> Point {
        Point {
            x: self.p1.x.max(self.p2.x),
            y: self.p1.y.max(self.p2.y),
        }
    }

    pub fn area(&self) -> i64 {
        let min = self.min();
        let max = self.max();

        (max.x - min.x + 1) * (max.y - min.y + 1)
    }

    pub fn intersects(&self, segment: &Rectangle) -> bool {
        let r_min = self.min();
        let r_max = self.max();
        let s_min = segment.min();
        let s_max = segment.max();

        !(r_min.x >= s_max.x || r_max.x <= s_min.x || r_min.y >= s_max.y || r_max.y <= s_min.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }

    #[test]
    fn rectangle_area() {
        let r = Rectangle::new(Point::new(2, 5), Point::new(11, 1));

        assert_eq!(r.area(), 50);
    }
}
