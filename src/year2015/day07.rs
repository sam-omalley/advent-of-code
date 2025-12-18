use cached::proc_macro::cached;
use std::collections::BTreeMap;
use std::str::FromStr;

pub type Input = BTreeMap<String, Operator>;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Value {
    Reference(String),
    Concrete(u16),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Operator {
    Binary {
        left: Value,
        right: Value,
        op: BinaryOp,
    },
    Unary {
        left: Value,
        negate: bool,
    },
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum BinaryOp {
    And,
    Lshift,
    Rshift,
    Or,
}

impl FromStr for Value {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = s.parse::<u16>() {
            Ok(Value::Concrete(val))
        } else {
            Ok(Value::Reference(s.to_string()))
        }
    }
}

impl FromStr for BinaryOp {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "LSHIFT" => Ok(Self::Lshift),
            "RSHIFT" => Ok(Self::Rshift),
            "OR" => Ok(Self::Or),
            _ => Err(format!("Invalid operator: {s}").to_string()),
        }
    }
}

impl FromStr for Operator {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();

        if words.len() == 3 {
            let left = words[0];
            let op = words[1];
            let right = words[2];

            Ok(Self::Binary {
                left: left.parse().unwrap(),
                right: right.parse().unwrap(),
                op: op.parse().unwrap(),
            })
        } else if words.len() == 2 {
            Ok(Self::Unary {
                left: words[1].parse().unwrap(),
                negate: true,
            })
        } else if words.len() == 1 {
            Ok(Self::Unary {
                left: words[0].parse().unwrap(),
                negate: false,
            })
        } else {
            Err(format!("Could not parse operator: {s}").to_string())
        }
    }
}

/*
Parse into DAG. Annotate nodes with operations.
Toplogically sort. Solve in order? Possible solve in reverse order from a.
 */

pub fn parse(input: &str) -> Input {
    let mut operations = BTreeMap::new();
    for line in input.lines().map(str::trim).filter(|s| !s.is_empty()) {
        let mut iter = line.split(" -> ");
        let operator = iter.next().unwrap();
        let key = iter.next().unwrap();
        operations.insert(key.to_string(), operator.parse().unwrap());
    }

    operations
}

#[cached(key = "Value", convert = r#"{ value.clone() }"#)]
pub fn solve_value(value: &Value, lookup: &Input) -> u16 {
    match value {
        Value::Reference(key) => solve_operator(&lookup[key], lookup),
        Value::Concrete(val) => *val,
    }
}

#[cached(key = "Operator", convert = r#"{ operator.clone() }"#)]
pub fn solve_operator(operator: &Operator, lookup: &Input) -> u16 {
    match operator {
        Operator::Binary { left, right, op } => {
            op.apply(solve_value(left, lookup), solve_value(right, lookup))
        }
        Operator::Unary { left, negate } => {
            if *negate {
                !solve_value(left, lookup)
            } else {
                solve_value(left, lookup)
            }
        }
    }
}

impl BinaryOp {
    pub fn apply(&self, left: u16, right: u16) -> u16 {
        match self {
            Self::And => left & right,
            Self::Lshift => left << right,
            Self::Rshift => left >> right,
            Self::Or => left | right,
        }
    }
}

pub fn part1(lookup: &Input) -> u16 {
    solve_operator(&lookup["a"], lookup)
}

pub fn part2(_input: &Input) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part_one(&advent_of_code::template::read_file("examples", 2015, 7));
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2() {
        let result = part_two(&advent_of_code::template::read_file("examples", 2015, 7));
        assert_eq!(result, 0);
    }
}
