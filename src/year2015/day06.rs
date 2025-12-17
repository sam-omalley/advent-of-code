use crate::util::vec2::*;
use std::collections::{HashMap, HashSet, hash_map::Entry};

#[derive(Debug)]
pub enum Operation {
    Add(Rectangle),
    Remove(Rectangle),
    Toggle(Rectangle),
}

type Input = Vec<Operation>;

fn parse_points(line: &str) -> (Vec2, Vec2) {
    let mut iter = line.split_ascii_whitespace();
    let p1 = Vec2::parse(iter.next().unwrap());
    let p2 = Vec2::parse(iter.next_back().unwrap());

    (p1, p2)
}

pub fn parse(input: &str) -> Input {
    let mut out = Vec::new();
    for line in input.lines().map(str::trim).filter(|&s| !s.is_empty()) {
        if let Some(l) = line.strip_prefix("turn on") {
            let (p1, p2) = parse_points(l);
            out.push(Operation::Add(Rectangle::new(p1, p2)));
        } else if let Some(l) = line.strip_prefix("turn off") {
            let (p1, p2) = parse_points(l);
            out.push(Operation::Remove(Rectangle::new(p1, p2)));
        } else if let Some(l) = line.strip_prefix("toggle") {
            let (p1, p2) = parse_points(l);
            out.push(Operation::Toggle(Rectangle::new(p1, p2)));
        } else {
            continue;
        }
    }

    out
}

pub fn part1(input: &Input) -> usize {
    let mut grid = HashSet::<Vec2>::default();
    for op in input {
        match op {
            Operation::Add(r) => {
                for p in r.points() {
                    grid.insert(p);
                }
            }
            Operation::Remove(r) => {
                for p in r.points() {
                    grid.remove(&p);
                }
            }
            Operation::Toggle(r) => {
                for p in r.points() {
                    if grid.contains(&p) {
                        grid.remove(&p);
                    } else {
                        grid.insert(p);
                    }
                }
            }
        }
    }
    grid.len()
}

pub fn part2(input: &Input) -> u32 {
    let mut grid = HashMap::<Vec2, u32>::default();
    for op in input {
        match op {
            Operation::Add(r) => {
                for p in r.points() {
                    *grid.entry(p).or_insert(0) += 1;
                }
            }
            Operation::Remove(r) => {
                for p in r.points() {
                    if let Entry::Occupied(mut e) = grid.entry(p) {
                        let v = e.get_mut();
                        *v = v.saturating_sub(1);
                        if *v == 0 {
                            e.remove();
                        }
                    }
                }
            }
            Operation::Toggle(r) => {
                for p in r.points() {
                    *grid.entry(p).or_insert(0) += 2;
                }
            }
        }
    }
    grid.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            part1(&vec![Operation::Add(Rectangle::new(
                Vec2::new(0, 0),
                Vec2::new(999, 999)
            ))]),
            1000000
        );
        assert_eq!(
            part1(&vec![
                Operation::Add(Rectangle::new(Vec2::new(0, 0), Vec2::new(999, 999))),
                Operation::Remove(Rectangle::new(Vec2::new(0, 0), Vec2::new(999, 0))),
            ]),
            1000000 - 1000
        );
        assert_eq!(
            part1(&vec![
                Operation::Add(Rectangle::new(Vec2::new(0, 0), Vec2::new(999, 999))),
                Operation::Remove(Rectangle::new(Vec2::new(0, 0), Vec2::new(999, 0))),
                Operation::Toggle(Rectangle::new(Vec2::new(499, 499), Vec2::new(500, 500))),
            ]),
            1000000 - 1000 - 4
        );
    }
}
