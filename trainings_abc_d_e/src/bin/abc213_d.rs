use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n-1],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    for graph_i in graph.iter_mut() {
        graph_i.sort_unstable();
    }

    let mut seen = vec![false; n];
    let mut order = vec![];
    dfs(&graph, &mut seen, &mut order, 0);

    println!("{}", order.iter().map(|&v| v + 1).join(" "));
}

pub fn dfs(graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, order: &mut Vec<usize>, v: usize) {
    seen[v] = true;
    order.push(v);

    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        dfs(graph, seen, order, next_v);
        order.push(v);
    }
}
