#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
use itertools::Itertools;
use petgraph::Undirected;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;

type Input = (Graph<String, u32, Undirected>, HashMap<String, NodeIndex>);

#[must_use]
pub fn parse(input: &str) -> Input {
    let mut graph = Graph::new_undirected();
    let mut lookup = HashMap::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let a = iter.next().unwrap().to_string();
        iter.next().unwrap();
        let b = iter.next().unwrap().to_string();

        if !lookup.contains_key(&a) {
            let node = graph.add_node(a.clone());
            lookup.insert(a.clone(), node);
        }

        if !lookup.contains_key(&b) {
            let node = graph.add_node(b.clone());
            lookup.insert(b.clone(), node);
        }
    }

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let a = iter.next().unwrap().to_string();
        iter.next().unwrap();
        let b = iter.next().unwrap().to_string();
        iter.next().unwrap();
        let e = iter.next().unwrap().parse().unwrap();

        graph.add_edge(lookup[&a], lookup[&b], e);
    }

    (graph, lookup)
}

#[must_use]
pub fn part1(input: &Input) -> u32 {
    min_max(input).0
}

#[must_use]
pub fn part2(input: &Input) -> u32 {
    min_max(input).1
}

#[must_use]
pub fn min_max(input: &Input) -> (u32, u32) {
    let graph = &input.0;
    let lookup = &input.1;
    let mut min = u32::MAX;
    let mut max = 0;
    for path in lookup.keys().permutations(lookup.keys().len()) {
        let cost = path
            .iter()
            .tuple_windows()
            .map(|(&a, &b)| {
                *graph
                    .edge_weight(graph.find_edge(lookup[a], lookup[b]).unwrap())
                    .unwrap()
            })
            .sum();

        min = min.min(cost);
        max = max.max(cost);
    }
    (min, max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_part1() {
        let input = parse(&template::read_file("examples", year!(2015), day!(9)));
        assert_eq!(part1(&input), 605);
    }

    #[test]
    fn test_part2() {
        let input = parse(&template::read_file("examples", year!(2015), day!(9)));
        assert_eq!(part2(&input), 982);
    }
}
