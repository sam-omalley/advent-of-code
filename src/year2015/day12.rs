#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]

use regex::Regex;
use serde_json::Value;

#[must_use]
pub fn parse(input: &str) -> &str {
    input.trim()
}

#[must_use]
pub fn part1(input: &str) -> i64 {
    let re = Regex::new(r"(-?\d+)").unwrap();
    let mut total = 0;
    for (_, [num]) in re.captures_iter(input).map(|c| c.extract()) {
        total += num.parse::<i64>().unwrap();
    }
    total
}

#[must_use]
pub fn part2(input: &str) -> i64 {
    let v: Value = serde_json::from_str(input).unwrap();
    walk(&v)
}

#[must_use]
pub fn obj_contains_value(map: &serde_json::Map<String, Value>) -> bool {
    for val in map.values() {
        if let Value::String(s) = val
            && s == "red"
        {
            return true;
        }
    }
    false
}

pub fn walk(value: &Value) -> i64 {
    match value {
        Value::Array(arr) => arr.iter().map(walk).sum(),
        Value::Object(obj) => {
            if obj_contains_value(obj) {
                0
            } else {
                obj.values().map(walk).sum()
            }
        }
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(part1("[1,2,3]"), 6);
        assert_eq!(part1(r#"{"a":2,"b":4}"#), 6);
        assert_eq!(part1(r#"[[[3]]]"#), 3);
        assert_eq!(part1(r#"{"a":{"b":4},"c":-1}"#), 3);
        assert_eq!(part1(r#"{"a":[-1,1]}"#), 0);
        assert_eq!(part1(r#"[-1,{"a":1}]"#), 0);
        assert_eq!(part1(r#"[]"#), 0);
        assert_eq!(part1(r#"{}"#), 0);
    }

    #[test]
    fn part_two() {
        assert_eq!(part2(r"[1,2,3]"), 6);
        assert_eq!(part2(r#"[1,{"c":"red","b":2},3]"#), 4);
        assert_eq!(part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
        assert_eq!(part2(r#"[1,"red",5]"#), 6);
    }
}
