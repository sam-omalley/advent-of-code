aoc_2025::solution!(7);
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u64> {
    let manifold = Manifold::parse(input);
    Some(manifold.get_num_splits())
}

pub fn part_two(input: &str) -> Option<u64> {
    let manifold = Manifold::parse(input);
    Some(manifold.get_num_timelines())
}

#[derive(Default)]
pub struct Manifold {
    pub start: Option<(usize, usize)>,
    depth: usize,
    pub splitters: Vec<HashSet<usize>>,
    max_splitter_idx: usize,
}

impl Manifold {
    pub fn has_splitter(&self, row: usize, col: usize) -> bool {
        self.splitters[row].contains(&col)
    }

    pub fn parse(contents: &str) -> Manifold {
        let mut manifold = Manifold::default();

        for (row, line) in contents.lines().enumerate() {
            manifold.splitters.push(HashSet::new());
            for (col, char) in line.chars().enumerate() {
                if char == '^' {
                    manifold.splitters[row].insert(col);
                } else if char == 'S' {
                    manifold.start = Some((row, col));
                }
            }
            manifold.depth = row;
            manifold.max_splitter_idx = row.max(manifold.max_splitter_idx);
        }

        manifold
    }

    pub fn get_num_splits(&self) -> u64 {
        let (start_row, start_col) = self.start.expect("Start _must_ be set");

        let mut count = 0;

        let mut beams = HashSet::<usize>::new();
        beams.insert(start_col);
        for row in start_row..self.depth {
            for beam in beams.clone() {
                if self.has_splitter(row, beam) {
                    count += 1;
                    beams.remove(&beam);
                    beams.insert(beam - 1);
                    beams.insert(beam + 1);
                }
            }
        }

        count
    }

    pub fn get_num_timelines(&self) -> u64 {
        let (start_row, start_col) = self.start.expect("Start _must_ be set");

        let mut values = Vec::<u64>::new();
        values.resize(self.max_splitter_idx + 2, 1);

        // Work backwards merging the splitters. Once we hit the start coordinate we are done.
        for row in (start_row..self.depth).rev() {
            for col in self.splitters[row].iter() {
                values[*col] = values[*col - 1] + values[*col + 1];
            }
        }

        values[start_col]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2025::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
