use std::collections::HashSet;

pub type Input = Manifold;

pub fn parse(input: &str) -> Input {
    Manifold::parse(input)
}

pub fn part1(manifold: &Input) -> u64 {
    manifold.get_num_splits()
}

pub fn part2(manifold: &Input) -> u64 {
    manifold.get_num_timelines()
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
    use crate::*;

    #[test]
    fn test_part_one() {
        let result = part1(&parse(&template::read_file(
            "examples",
            year!(2025),
            day!(7),
        )));
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_two() {
        let result = part2(&parse(&template::read_file(
            "examples",
            year!(2025),
            day!(7),
        )));
        assert_eq!(result, 40);
    }
}
