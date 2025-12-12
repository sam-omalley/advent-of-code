fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ./app <filename>");
        std::process::exit(1);
    }

    let content = std::fs::read_to_string(&args[1]).unwrap();

    let mut safety_count = 0;

    for line in content.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

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
                println!("Safe");
                safety_count += 1;
                break;
            }
        }
    }
    println!("Safe: {}", safety_count);
}
