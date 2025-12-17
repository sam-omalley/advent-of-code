use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Default, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct Vec2 {
    pub x: i64,
    pub y: i64,
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Vec2 {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    pub fn parse(s: &str) -> Self {
        let mut iter = s.split(",");
        Self {
            x: iter.next().unwrap().parse().unwrap(),
            y: iter.next().unwrap().parse().unwrap(),
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Rectangle {
    p1: Vec2,
    p2: Vec2,
}

pub struct RectPointsIter {
    rectangle: Rectangle,
    current: Vec2,
}

impl Rectangle {
    pub fn new(p1: Vec2, p2: Vec2) -> Self {
        Rectangle { p1, p2 }
    }

    pub fn points(&self) -> RectPointsIter {
        RectPointsIter {
            rectangle: *self,
            current: self.min(),
        }
    }

    pub fn encloses(&self, p: &Vec2) -> bool {
        let min = self.min();
        let max = self.max();
        min.x <= p.x && p.x <= max.x && min.y <= p.y && p.y <= max.y
    }

    pub fn min(&self) -> Vec2 {
        Vec2 {
            x: self.p1.x.min(self.p2.x),
            y: self.p1.y.min(self.p2.y),
        }
    }

    pub fn max(&self) -> Vec2 {
        Vec2 {
            x: self.p1.x.max(self.p2.x),
            y: self.p1.y.max(self.p2.y),
        }
    }

    pub fn area(&self) -> usize {
        let min = self.min();
        let max = self.max();

        (max.x - min.x + 1) as usize * (max.y - min.y + 1) as usize
    }

    pub fn intersects(&self, segment: &Rectangle) -> bool {
        let r_min = self.min();
        let r_max = self.max();
        let s_min = segment.min();
        let s_max = segment.max();

        !(r_min.x >= s_max.x || r_max.x <= s_min.x || r_min.y >= s_max.y || r_max.y <= s_min.y)
    }
}

impl Iterator for RectPointsIter {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x > self.rectangle.max().x || self.current.y > self.rectangle.max().y {
            return None;
        }

        let p = self.current;

        // Advance cursor
        self.current.x += 1;
        if self.current.x > self.rectangle.max().x {
            self.current.x = self.rectangle.min().x;
            self.current.y += 1;
        }

        Some(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator() {
        let rect = Rectangle::new(Vec2::new(45, 46), Vec2::new(53, 51));

        assert_eq!(rect.points().count(), rect.area());
    }
}
