<img src="../.assets/christmas_ferris.png" width="164">

# 🎄 Advent of Code 2025

Solutions for [Advent of Code](https://adventofcode.com/) in [Rust](https://www.rust-lang.org/).

<!--- advent_readme_stars table --->
## 2025 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2025/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2025/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2025/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2025/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2025/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2025/day/6) | ⭐ | ⭐ |
| [Day 7](https://adventofcode.com/2025/day/7) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2025/day/8) | ⭐ | ⭐ |
| [Day 9](https://adventofcode.com/2025/day/9) | ⭐ | ⭐ |
| [Day 10](https://adventofcode.com/2025/day/10) | ⭐ | ⭐ |
| [Day 11](https://adventofcode.com/2025/day/11) | ⭐ | ⭐ |
| [Day 12](https://adventofcode.com/2025/day/12) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

<!--- benchmarking table --->
## Benchmarks

| Day | Part 1 | Part 2 |
| :---: | :---: | :---:  |
| [Day 1](./src/bin/01.rs) | `88.8µs` | `86.5µs` |
| [Day 2](./src/bin/02.rs) | `3.5ms` | `273.5ms` |
| [Day 3](./src/bin/03.rs) | `131.2µs` | `164.1µs` |
| [Day 4](./src/bin/04.rs) | `189.6µs` | `1.3ms` |
| [Day 5](./src/bin/05.rs) | `112.5µs` | `64.5µs` |
| [Day 6](./src/bin/06.rs) | `163.4µs` | `13.1ms` |
| [Day 7](./src/bin/07.rs) | `250.2µs` | `76.3µs` |
| [Day 8](./src/bin/08.rs) | `33.9ms` | `37.5ms` |
| [Day 9](./src/bin/09.rs) | `8.7ms` | `27.4ms` |
| [Day 10](./src/bin/10.rs) | `740.2µs` | `203.7ms` |
| [Day 11](./src/bin/11.rs) | `272.7µs` | `285.5µs` |
| [Day 12](./src/bin/12.rs) | `293.2µs` | `17.0ns` |

**Total: 605.52ms**
<!--- benchmarking table --->

## Notes
Algorithm improvement ideas.

- Day 3: [Monotonic Stack](https://www.geeksforgeeks.org/dsa/introduction-to-monotonic-stack-2/)
- Day 8: [UnionFind/DSU](https://www.geeksforgeeks.org/dsa/introduction-to-disjoint-set-data-structure-or-union-find-algorithm/)
- Day 9: Coordinate compression and flood fill
- Solve Day 10 without Z3
  - Write an ILP solver. Simplex or Gaussian Elimination + brute force
  - Try the [bifurcation parity](https://www.reddit.com/r/adventofcode/comments/1pk87hl/2025_day_10_part_2_bifurcate_your_way_to_victory/) solution.

