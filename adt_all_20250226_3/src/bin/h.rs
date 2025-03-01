use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n-1],
    }

    let mut graph = vec![vec![]; n];
    let mut deg = vec![0; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
        deg[u] += 1;
        deg[v] += 1;
    }

    let start = (0..n).find(|&i| deg[i] == 1).unwrap();

    let mut levels = vec![];
    dfs(&graph, &deg, &mut levels, n, start, 1);
    levels.sort();

    println!("{}", levels.iter().join(" "));
}

pub fn dfs(
    graph: &Vec<Vec<usize>>,
    deg: &Vec<usize>,
    levels: &mut Vec<usize>,
    parent: usize,
    v: usize,
    vertex_cnt: usize,
) {
    if vertex_cnt % 3 == 2 {
        levels.push(graph[v].len());
    }

    for &next_v in &graph[v] {
        if next_v == parent {
            continue;
        }
        dfs(graph, deg, levels, v, next_v, vertex_cnt + 1);
    }
}
