use itertools::Itertools;
use std::collections::HashSet;
use std::error::Error;

pub fn part_one(input: &str) -> Option<u64> {
    let mut cloud = PointCloud::default();
    for line in input.lines() {
        cloud.add_str(line).unwrap();
    }

    let iterations = if cfg!(test) { 10 } else { 1000 };
    solve(cloud, iterations)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut cloud = PointCloud::default();
    for line in input.lines() {
        cloud.add_str(line).unwrap();
    }

    solve(cloud, 0)
}

pub fn solve(cloud: PointCloud, max_connections: usize) -> Option<u64> {
    let mut circuits = Vec::<HashSet<Point>>::default();
    let mut last_connection = (Point::default(), Point::default());

    'outer: for (p1, p2, _) in cloud
        .points
        .iter()
        .combinations(2)
        .map(|pair| {
            let p1 = pair.first().unwrap();
            let p2 = pair.last().unwrap();
            (*p1, *p2, p1.distance2(p2))
        })
        .sorted_by(|(_, _, d1), (_, _, d2)| d1.partial_cmp(d2).unwrap())
        .take(if max_connections > 0 {
            max_connections
        } else {
            usize::MAX
        })
    {
        if circuits.len() == 1 && circuits[0].len() == cloud.points.len() {
            return Some(last_connection.0.0 as u64 * last_connection.1.0 as u64);
        }

        last_connection = (p1.clone(), p2.clone());
        let mut candidate_circuits = HashSet::<usize>::default();
        for (idx, circuit) in circuits.iter().enumerate() {
            if circuit.contains(p1) || circuit.contains(p2) {
                candidate_circuits.insert(idx);
            }

            if circuit.contains(p1) && circuit.contains(p2) {
                continue 'outer;
            }
        }

        if candidate_circuits.is_empty() {
            let mut junction = HashSet::new();
            junction.insert(p1.clone());
            junction.insert(p2.clone());
            circuits.push(junction);
        } else if candidate_circuits.len() == 2 {
            let mut iter = candidate_circuits.iter();

            let a_idx = iter.next().unwrap();
            let b_idx = iter.next().unwrap();

            let b = circuits[*b_idx].clone();
            circuits[*a_idx].extend(b);
            circuits.remove(*b_idx);

            continue 'outer;
        } else if candidate_circuits.len() == 1 {
            let mut iter = candidate_circuits.iter();
            let a_idx = iter.next().unwrap();
            circuits[*a_idx].insert(p1.clone());
            circuits[*a_idx].insert(p2.clone());
            continue 'outer;
        } else {
            panic!("This shouldn't happen: {}", candidate_circuits.len());
        }
    }

    if max_connections > 0 {
        let mut lengths: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
        lengths.sort_unstable_by(|a, b| b.cmp(a));
        Some(lengths.iter().take(3).product::<usize>() as u64)
    } else {
        None
    }
}

#[derive(Default, Clone)]
pub struct PointCloud {
    pub points: Vec<Point>,
}

impl PointCloud {
    pub fn add_str(&mut self, val: &str) -> Result<(), Box<dyn Error>> {
        let parts: Vec<&str> = val.split(',').collect();

        if parts.len() == 3 {
            let mut p = Point::default();
            p.set_x(parts[0].parse().unwrap());
            p.set_y(parts[1].parse().unwrap());
            p.set_z(parts[2].parse().unwrap());

            self.points.push(p);
        }

        Ok(())
    }
}

#[derive(Hash, Default, Debug, PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct Point(pub i64, pub i64, pub i64);

impl Point {
    pub fn set_x(&mut self, val: i64) {
        self.0 = val;
    }
    pub fn set_y(&mut self, val: i64) {
        self.1 = val;
    }
    pub fn set_z(&mut self, val: i64) {
        self.2 = val;
    }

    pub fn distance2(&self, other: &Point) -> i64 {
        (other.0 - self.0).pow(2) + (other.1 - self.1).pow(2) + (other.2 - self.2).pow(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }

    #[test]
    fn distance() {
        let distance2 = Point(162, 817, 812).distance2(&Point(425, 690, 689));
        assert_eq!(distance2, 100427);
    }
}
