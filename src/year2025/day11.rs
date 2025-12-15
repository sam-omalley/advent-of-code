use petgraph::algo;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Topo;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

pub fn part_one(input: &str) -> Option<u64> {
    let (graph, lookup) = parse_graph(input);

    let origin = lookup["you"];
    let end = lookup["out"];

    let result = algo::all_simple_paths::<Vec<_>, _, RandomState>(&graph, origin, end, 0, None);

    Some(result.count().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (graph, lookup) = parse_graph(input);

    let svr = lookup["svr"];
    let dac = lookup["dac"];
    let fft = lookup["fft"];
    let out = lookup["out"];

    let svr_to_dac = count_paths_dag(&graph, svr, dac);
    let svr_to_fft = count_paths_dag(&graph, svr, fft);
    let fft_to_dac = count_paths_dag(&graph, fft, dac);
    let dac_to_fft = count_paths_dag(&graph, dac, fft);
    let dac_to_out = count_paths_dag(&graph, dac, out);
    let fft_to_out = count_paths_dag(&graph, fft, out);

    let total = (svr_to_dac * dac_to_fft * fft_to_out) + (svr_to_fft * fft_to_dac * dac_to_out);
    Some(total)
}

pub fn parse_graph(value: &str) -> (DiGraph<String, ()>, HashMap<String, NodeIndex>) {
    let mut graph = DiGraph::new();

    let mut lookup = HashMap::new();

    for line in value.lines().map(str::trim).filter(|&s| !s.is_empty()) {
        let mut iter = line.split_whitespace();
        let name = iter.next().unwrap().replace(":", "");

        let node = graph.add_node(name.clone());

        lookup.insert(name.clone(), node);
    }

    lookup.insert("out".into(), graph.add_node("out".into()));

    for line in value.lines().map(str::trim).filter(|&s| !s.is_empty()) {
        let mut iter = line.split_whitespace();
        let name = iter.next().unwrap().replace(":", "");

        let src = lookup[&name];

        for dest in iter {
            let dst = lookup[dest];
            graph.add_edge(src, dst, ());
        }
    }

    (graph, lookup)
}

pub fn count_paths_dag(g: &DiGraph<String, ()>, start: NodeIndex, end: NodeIndex) -> u64 {
    let mut topo = Topo::new(&g);
    let mut order = Vec::new();
    while let Some(n) = topo.next(&g) {
        order.push(n);
    }

    let mut dp = vec![0u64; g.node_count()];
    dp[start.index()] = 1;

    for &u in &order {
        let ways = dp[u.index()];
        if ways > 0 {
            for v in g.neighbors(u) {
                dp[v.index()] += ways;
            }
        }
    }

    dp[end.index()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&aoc_2025::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&aoc_2025::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
