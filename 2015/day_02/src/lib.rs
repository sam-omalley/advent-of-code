pub fn calculate_paper(s: &str) -> i64 {
    let mut iter = s.split("x");

    let l: i64 = iter.next().unwrap().parse().unwrap();
    let w: i64 = iter.next().unwrap().parse().unwrap();
    let h: i64 = iter.next().unwrap().parse().unwrap();

    (2 * l * w) + (2 * w * h) + (2 * h * l) + (l * w).min(w * h).min(h * l)
}

pub fn calculate_ribbon(s: &str) -> i64 {
    let mut iter = s.split("x");

    let l: i64 = iter.next().unwrap().parse().unwrap();
    let w: i64 = iter.next().unwrap().parse().unwrap();
    let h: i64 = iter.next().unwrap().parse().unwrap();

    (l + l + w + w).min(w + w + h + h).min(h + h + l + l) + (w * h * l)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(calculate_paper("2x3x4"), 58);
        assert_eq!(calculate_paper("1x1x10"), 43);

        assert_eq!(calculate_ribbon("2x3x4"), 34);
        assert_eq!(calculate_ribbon("1x1x10"), 14);
    }
}
