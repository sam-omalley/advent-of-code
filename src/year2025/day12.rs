pub struct Input {
    pub shapes: Vec<Shape>,
    pub spaces: Vec<Space>,
}

pub fn parse(input: &str) -> Input {
    let mut shape = Vec::new();
    let mut shapes = Vec::new();
    let mut spaces = Vec::new();

    for line in input.lines().map(str::trim) {
        if line.contains("x") {
            spaces.push(Space::new(line));
        } else if line.is_empty() {
            shapes.push(Shape::new(&shape));
            shape.clear();
        } else if line.contains(".") || line.contains("#") {
            shape.push(line);
        }
    }

    #[cfg(test)]
    for (idx, shape) in shapes.iter().enumerate() {
        println!("Shape: {idx}");
        println!("{shape}");
        println!();
    }

    Input { shapes, spaces }
}

pub fn part1(input: &Input) -> u64 {
    let mut easy_fit = 0;
    #[allow(unused_variables)]
    let mut easy_reject = 0;
    #[allow(unused_variables)]
    let mut other = 0;

    for space in &input.spaces {
        let (min, max) = space.get_bounds(&input.shapes);

        #[cfg(test)]
        {
            println!(
                "Space ({}x{}) [{}]:",
                space.width,
                space.length,
                space.area()
            );
            println!("{min} -> {max}");
        }

        if max <= space.area() {
            easy_fit += 1;
        } else if min > space.area() {
            easy_reject += 1;
        } else {
            other += 1;
        }
    }

    #[cfg(test)]
    {
        println!("Day 12 Part 1");
        println!(" - Easy Reject: {easy_reject}");
        println!(" - Easy Fit: {easy_fit}");
        println!(" - Other: {other}");
    }

    easy_fit
}

pub fn part2(_input: &Input) -> &'static str {
    "n/a"
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Shape {
    pub shape: [bool; 9],
}

impl Shape {
    pub fn new(s: &Vec<&str>) -> Self {
        let mut shape = Self { shape: [false; 9] };
        for (row, line) in s.iter().enumerate() {
            for (col, char) in line.chars().enumerate() {
                shape.shape[row * 3 + col] = char == '#'
            }
        }

        shape
    }
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, val) in self.shape.iter().enumerate() {
            if *val {
                f.write_str("#")?;
            } else {
                f.write_str(".")?;
            }
            if idx % 3 == 2 && idx < self.shape.len() - 1 {
                f.write_str("\n")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Space {
    space: Vec<bool>,
    pub width: usize,
    pub length: usize,
    pub requirements: Vec<usize>,
}

impl Space {
    pub fn new(s: &str) -> Self {
        let mut res = Self::default();

        let mut iter = s.split(":");
        let size = iter.next().unwrap().trim();
        for requirement in iter.next().unwrap().split_whitespace() {
            res.requirements.push(requirement.parse().unwrap());
        }

        let mut iter = size.split("x");
        res.width = iter.next().unwrap().parse().unwrap();
        res.length = iter.next().unwrap().parse().unwrap();

        res.space.resize(res.width * res.length, false);

        res
    }

    pub fn area(&self) -> usize {
        self.width * self.length
    }

    pub fn get_bounds(&self, shapes: &[Shape]) -> (usize, usize) {
        let mut max_area = 0;
        let mut min_area = 0;

        for (idx, shape) in shapes.iter().enumerate() {
            min_area += self.requirements[idx] * shape.shape.iter().filter(|&c| *c).count();
            max_area += self.requirements[idx] * shape.shape.iter().count();
        }

        (min_area, max_area)
    }
}

impl std::fmt::Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, val) in self.space.iter().enumerate() {
            if *val {
                f.write_str("#")?;
            } else {
                f.write_str(".")?;
            }
            if idx % self.width == self.width - 1 && idx < self.space.len() - 1 {
                f.write_str("\n")?;
            }
        }
        Ok(())
    }
}
