use petgraph::algo;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Topo;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

pub type Input = (DiGraph<String, ()>, HashMap<String, NodeIndex>);

pub fn parse(input: &str) -> Input {
    let mut graph = DiGraph::new();
    let mut lookup = HashMap::new();

    for line in input.lines().map(str::trim).filter(|&s| !s.is_empty()) {
        let mut iter = line.split_whitespace();
        let name = iter.next().unwrap().replace(":", "");

        let node = graph.add_node(name.clone());

        lookup.insert(name.clone(), node);
    }

    lookup.insert("out".into(), graph.add_node("out".into()));

    for line in input.lines().map(str::trim).filter(|&s| !s.is_empty()) {
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

pub fn part1(input: &Input) -> u64 {
    let (graph, lookup) = input;

    let origin = lookup["you"];
    let end = lookup["out"];

    let result = algo::all_simple_paths::<Vec<_>, _, RandomState>(&graph, origin, end, 0, None);

    result.count().try_into().unwrap()
}

pub fn part2(input: &Input) -> u64 {
    let (graph, lookup) = input;

    let svr = lookup["svr"];
    let dac = lookup["dac"];
    let fft = lookup["fft"];
    let out = lookup["out"];

    let svr_to_dac = count_paths_dag(graph, svr, dac);
    let svr_to_fft = count_paths_dag(graph, svr, fft);
    let fft_to_dac = count_paths_dag(graph, fft, dac);
    let dac_to_fft = count_paths_dag(graph, dac, fft);
    let dac_to_out = count_paths_dag(graph, dac, out);
    let fft_to_out = count_paths_dag(graph, fft, out);

    (svr_to_dac * dac_to_fft * fft_to_out) + (svr_to_fft * fft_to_dac * dac_to_out)
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
    use crate::*;

    #[test]
    fn test() {
        let input = parse(&template::read_file_part("examples", year!(2025), day!(11), 1));
        assert_eq!(part1(&input), 5);

        let input = parse(&template::read_file_part("examples", year!(2025), day!(11), 2));
        assert_eq!(part2(&input), 2);
    }
}
