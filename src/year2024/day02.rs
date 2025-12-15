aoc_2024::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
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

    Some(safety_count)
}

pub fn part_two(input: &str) -> Option<u64> {
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

    Some(safety_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2024::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
