pub fn part_1(input: &str) -> usize {
    let mut counter = 0;
    loop {
        let digest = md5::compute((input.to_string() + &counter.to_string()).as_bytes());
        if check_digest(&digest) {
            eprintln!("{input}{counter}: {:x}", digest);
            break;
        }
        counter += 1;
    }
    counter
}

pub fn check_digest(digest: &md5::Digest) -> bool {
    digest[0] | digest[1] | (digest[2] >> 4) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(check_digest(&md5::compute("pqrstuv1048970")));
        assert!(check_digest(&md5::compute("abcdef609043")));
        assert_eq!(part_1("abcdef"), 609043);
    }
}
