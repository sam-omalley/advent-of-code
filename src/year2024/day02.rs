pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> u64 {
    let mut safety_count = 0;

    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        #[cfg(test)]
        println!("{}", line);

        let mut increasing: Option<bool> = None;
        let mut safe = true;
        for (a, b) in levels.iter().zip(levels.iter().skip(1)) {
            let diff = a - b;

            if increasing.is_none() {
                increasing = Some(diff < 0);
            } else if increasing != Some(diff < 0) {
                safe = false;
                break;
            }

            if i32::abs(diff) < 1 || i32::abs(diff) > 3 {
                safe = false;
                break;
            }
        }

        if safe {
            #[cfg(test)]
            println!("Safe");
            safety_count += 1;
        }
    }

    safety_count
}

pub fn part2(input: &str) -> u64 {
    let mut safety_count = 0;

    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        #[cfg(test)]
        println!("{}", line);

        for skip_idx in 0..levels.len() {
            let subset: Vec<i32> = levels
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != skip_idx)
                .map(|(_, v)| *v)
                .collect();
            let mut increasing: Option<bool> = None;
            let mut safe = true;
            for (a, b) in subset.iter().zip(subset.iter().skip(1)) {
                let diff = a - b;

                if increasing.is_none() {
                    increasing = Some(diff < 0);
                } else if increasing != Some(diff < 0) {
                    safe = false;
                    break;
                }

                if i32::abs(diff) < 1 || i32::abs(diff) > 3 {
                    safe = false;
                    break;
                }
            }

            if safe {
                #[cfg(test)]
                println!("Safe");
                safety_count += 1;
                break;
            }
        }
    }

    safety_count
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test() {
        let input = template::read_file("examples", year!(2024), day!(2));
        assert_eq!(part1(&input), 2);
        assert_eq!(part2(&input), 4);
    }
}
