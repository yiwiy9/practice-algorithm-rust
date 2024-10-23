use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(Usize1,Usize1); n-1],
        v: [Usize1; k],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let start = v[0];
    println!("{}", dfs(&graph, &v.into_iter().collect(), n, start));
}

pub fn dfs(graph: &Vec<Vec<usize>>, v_set: &HashSet<usize>, par: usize, v: usize) -> usize {
    let mut res = 0;
    for &next_v in &graph[v] {
        if next_v == par {
            continue;
        }
        res += dfs(graph, v_set, v, next_v);
    }
    if v_set.contains(&v) || res > 0 {
        res += 1;
    }
    res
}
