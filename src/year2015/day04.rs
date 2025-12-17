use rayon::prelude::*;

pub fn parse(input: &str) -> &str {
    input.trim()
}

pub fn part1(input: &str) -> u64 {
    find_zero_prefix_md5_hash(input, check_digest_part_1)
}

pub fn part2(input: &str) -> u64 {
    find_zero_prefix_md5_hash(input, check_digest_part_2)
}

fn find_zero_prefix_md5_hash(input: &str, check: fn(&md5::Digest) -> bool) -> u64 {
    let mut counter = 0;
    let batch = 1024 * 1024;

    loop {
        let result = (counter..counter + batch)
            .into_par_iter()
            .map(|i| {
                (
                    i,
                    md5::compute((input.to_string() + &i.to_string()).as_bytes()),
                )
            })
            .find_any(|(_, x)| check(x));

        if let Some(result) = result {
            #[cfg(test)]
            eprintln!("{}: {:?}", result.0, result.1);
            return result.0;
        } else {
            counter += batch;
        }
    }
}

fn check_digest_part_1(digest: &md5::Digest) -> bool {
    digest[0] | digest[1] | (digest[2] >> 4) == 0
}

fn check_digest_part_2(digest: &md5::Digest) -> bool {
    digest[0] | digest[1] | digest[2] == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(check_digest_part_1(&md5::compute("pqrstuv1048970")));
        assert!(check_digest_part_1(&md5::compute("abcdef609043")));
        assert_eq!(part1("abcdef"), 609043);
    }
}
