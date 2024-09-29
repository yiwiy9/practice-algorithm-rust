use itertools::Itertools;
use proconio::{input, marker::Usize1};

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut uvw: [(Usize1,Usize1,i64); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        graph[u].push((v, w));
        graph[v].push((u, -w));
    }

    let mut dist = vec![INF; n];
    for v in 0..n {
        if dist[v] != INF {
            continue;
        }
        dist[v] = 0;
        dfs(&graph, &mut dist, v);
    }

    println!("{}", dist.iter().join(" "));
}

pub fn dfs(graph: &Vec<Vec<(usize, i64)>>, dist: &mut Vec<i64>, v: usize) {
    for &next_v in &graph[v] {
        if dist[next_v.0] != INF {
            continue;
        }
        dist[next_v.0] = dist[v] + next_v.1;
        dfs(graph, dist, next_v.0);
    }
}
