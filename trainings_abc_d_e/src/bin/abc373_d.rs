use itertools::Itertools as _;
use proconio::{input, marker::Usize1};
const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, i64); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, w) in uvw.iter() {
        graph[u].push((v, w));
        graph[v].push((u, -w));
    }

    let mut values = vec![INF; n];
    for v in 0..n {
        if values[v] != INF {
            continue;
        }
        dfs(&graph, &mut values, v, 0);
    }

    println!("{}", values.iter().join(" "));
}

pub fn dfs(graph: &Vec<Vec<(usize, i64)>>, values: &mut Vec<i64>, v: usize, value: i64) {
    values[v] = value;
    for &(next_v, edge_w) in &graph[v] {
        if values[next_v] != INF {
            continue;
        }
        dfs(graph, values, next_v, value + edge_w);
    }
}
