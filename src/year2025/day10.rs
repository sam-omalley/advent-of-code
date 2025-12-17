#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use itertools::Itertools;
use std::fmt;
use z3::{Optimize, SatResult, ast::Int};

#[must_use]
pub fn parse(input: &str) -> &str {
    input
}

#[must_use]
pub fn part1(input: &str) -> u64 {
    let mut counter = 0;
    for line in input.lines().map(str::trim) {
        let mut tokens = line.split_whitespace();

        let indicator = tokens.next().unwrap();
        let indicator_mask = BitMask::parse_indicators(indicator);
        tokens.next_back().unwrap();

        let mut buttons = Vec::<BitMask>::default();
        for button in tokens {
            buttons.push(BitMask::parse_button(button));
        }

        let mut combinations: u64 = 0;
        'outer: loop {
            for combos in buttons
                .iter()
                .combinations(combinations.try_into().unwrap())
            {
                let result = combos
                    .iter()
                    .copied()
                    .fold(BitMask::default(), |acc, x| BitMask(acc.0 ^ x.0));

                if result.0 == indicator_mask.0 {
                    counter += combinations;
                    break 'outer;
                }
            }
            combinations += 1;
        }
    }

    counter
}

#[must_use]
pub fn part2(input: &str) -> u64 {
    let mut total = 0;
    for line in input.lines().map(str::trim) {
        let machine = Machine::parse(line);
        total += machine.solve();
    }

    total
}

#[derive(Default, Clone, Copy)]
pub struct BitMask(pub u32);

impl BitMask {
    #[must_use]
    pub fn parse_indicators(s: &str) -> Self {
        let mut chars = s.chars().rev();

        // Pop the front and back [...] brackets.
        chars.next();
        chars.next_back();

        let mut mask: u32 = 0;

        for char in chars {
            if char == '.' {
                mask <<= 1;
            } else if char == '#' {
                mask <<= 1;
                mask += 1;
            } else {
                panic!("Bad input");
            }
        }

        Self(mask)
    }

    #[must_use]
    pub fn parse_button(s: &str) -> Self {
        let trimmed = &s[1..s.len() - 1];

        let mut mask: u32 = 0;
        for token in trimmed.split(',') {
            let val: u32 = token.parse().unwrap();
            mask |= 1 << val;
        }

        Self(mask)
    }
}

impl fmt::Display for BitMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016b}", self.0)
    }
}
impl fmt::Debug for BitMask {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
    }
}

#[derive(Clone, Debug)]
pub struct Machine {
    requirements: Vec<u32>,
    buttons: Vec<Vec<usize>>,
}

#[must_use]
pub fn get_csv(s: &str) -> Vec<&str> {
    // Slice off the brackets, and return csv iterator
    s[1..s.len() - 1].split(',').collect()
}

impl Machine {
    #[must_use]
    pub fn parse(line: &str) -> Self {
        let mut tokens = line.split_whitespace();

        // Pop front. It's unneeded.
        tokens.next().unwrap();

        // Get requirements from the end of the line.
        let requirements = {
            let s = tokens.next_back().unwrap();

            let mut requirements = Vec::<u32>::new();
            for token in get_csv(s) {
                requirements.push(token.parse().unwrap());
            }
            requirements
        };

        let buttons = {
            let mut buttons = Vec::<Vec<usize>>::new();
            for token in tokens {
                let mut wire = Vec::<usize>::new();
                for val in get_csv(token) {
                    wire.push(val.parse().unwrap());
                }
                buttons.push(wire);
            }
            buttons
        };

        Self {
            requirements,
            buttons,
        }
    }

    #[must_use]
    pub fn solve(&self) -> u64 {
        let opt = Optimize::new();

        let mut variables = Vec::new();
        for (idx, _) in self.buttons.iter().enumerate() {
            let var = Int::new_const(format!("x{idx}"));
            opt.assert(&var.ge(Int::from_u64(0)));

            variables.push(var);
        }

        for (requirement_idx, requirement) in self.requirements.iter().enumerate() {
            let mut function = Vec::new();
            for (button_idx, button) in self.buttons.iter().enumerate() {
                if button.contains(&requirement_idx) {
                    function.push(&variables[button_idx]);
                }
            }

            opt.assert(
                &function
                    .iter()
                    .fold(Int::from_u64(0), |acc, &x| &acc + x)
                    .eq(Int::from_u64(u64::from(*requirement))),
            );
        }

        let total = variables.iter().fold(Int::from_u64(0), |acc, x| &acc + x);
        opt.minimize(&total);

        let mut min_total = 0;
        // Solve
        match opt.check(&[]) {
            SatResult::Sat => {
                let model = opt.get_model().unwrap();
                min_total = model.eval(&total, true).unwrap().as_u64().unwrap();
                #[cfg(test)]
                {
                    eprintln!("min total = {min_total}");
                    for (i, v) in variables.iter().enumerate() {
                        eprintln!("x{} = {}", i, model.eval(v, true).unwrap());
                    }
                }
            }
            SatResult::Unsat => println!("unsat"),
            SatResult::Unknown => println!("unknown"),
        }
        min_total
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
            day!(10),
        )));
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part_two() {
        let result = part2(&parse(&template::read_file(
            "examples",
            year!(2025),
            day!(10),
        )));
        assert_eq!(result, 33);
    }
}
