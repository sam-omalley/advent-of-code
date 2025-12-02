use ilog::IntLog;

pub fn check_naughty_pair(value: u64) -> bool {
    let Some(digits) = value.checked_log10() else {
        return false;
    };

    let digits = (digits + 1) as u32;

    if !digits.is_multiple_of(2) {
        return false;
    }

    let digits = digits / 2;
    let multiplier = 10u64.pow(digits);

    let chunk = value / multiplier;
    let calculated_value = chunk + chunk * multiplier;

    calculated_value == value
}

pub fn check_all_naughty_pairs(a: u64, b: u64) -> u64 {
    let mut total: u64 = 0;

    for i in a..=b {
        if check_naughty_pair(i) {
            total += i;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindromes() {
        assert_eq!(true, check_naughty_pair(1212));

        assert_eq!(check_all_naughty_pairs(11, 22), 11 + 22);
        assert_eq!(check_all_naughty_pairs(95, 115),  99);
        assert_eq!(check_all_naughty_pairs(998, 1012),  1010);
        assert_eq!(check_all_naughty_pairs(1188511880, 1188511890),  1188511885);
        assert_eq!(check_all_naughty_pairs(222220, 222224),  222222);
        assert_eq!(check_all_naughty_pairs(1698522, 1698528), 0);
        assert_eq!(check_all_naughty_pairs(446443, 446449),  446446);
        assert_eq!(check_all_naughty_pairs(38593856, 38593862),  38593859);
    }
}
