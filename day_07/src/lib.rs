use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Default)]
pub struct Manifold {
    pub start: Option<(usize, usize)>,
    depth: usize,
    pub splitters: HashSet<(usize, usize)>,
}

impl Manifold {
    pub fn parse(contents: &str) -> Manifold {
        let mut manifold = Manifold::default();

        for (row, line) in contents.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                if char == '^' {
                    manifold.splitters.insert((row, col));
                } else if char == 'S' {
                    manifold.start = Some((row, col));
                }
            }
            manifold.depth = row;
        }

        manifold
    }

    pub fn get_num_splits(&self) -> usize {
        let (start_row, start_col) = self.start.expect("Start _must_ be set");

        let mut count = 0;

        let mut beams = HashSet::<usize>::new();
        beams.insert(start_col);
        for row in start_row..self.depth {
            for beam in beams.clone() {
                if self.splitters.contains(&(row, beam)) {
                    count += 1;
                    beams.remove(&beam);
                    beams.insert(beam - 1);
                    beams.insert(beam + 1);
                }
            }
        }

        count
    }

    pub fn get_num_timelines(&self) -> usize {
        let (start_row, start_col) = self.start.expect("Start _must_ be set");

        let mut beams = HashSet::<usize>::new();
        beams.insert(start_col);

        let (_, mut min_col) = *self.splitters.iter().min_by_key(|&(_, col)| col).unwrap();
        let (_, mut max_col) = *self.splitters.iter().max_by_key(|&(_, col)| col).unwrap();

        // We need to look at values at least one less/greater than the min/max splitter.
        min_col -= 1;
        max_col += 1;

        let mut map = HashMap::<usize, usize>::new();
        let mut values = Vec::<usize>::new();
        values.resize(max_col + 1, 1);

        // Populate initial states
        for col in min_col..=max_col {
            map.insert(col, 1);
        }

        // Work backwards merging the splitters. Once we hit the start coordinate we are done.
        for row in (start_row..self.depth).rev() {
            for (_, col) in self.splitters.iter().filter(|&(r, _)| r == &row) {
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
    fn manifold() {
        let manifold = Manifold::parse(
            "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............",
        );

        assert_eq!(manifold.start, Some((0, 7)));
        assert_eq!(manifold.splitters.len(), 22);
        assert!(manifold.splitters.contains(&(8, 6)));

        assert_eq!(manifold.get_num_splits(), 21);

        assert_eq!(manifold.get_num_timelines(), 40);
    }
}
