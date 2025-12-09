use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1,Usize1); n-1],
    }

    let mut graph = vec![vec![]; n];
    let mut degrees = vec![0; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
        degrees[u] += 1;
        degrees[v] += 1;
    }

    let mut stars = vec![];
    dfs(
        &graph,
        &mut stars,
        n,
        degrees.iter().position(|&cnt| cnt == 1).unwrap(),
        0,
    );

    stars.sort();
    println!("{}", stars.iter().join(" "));
}

pub fn dfs(
    graph: &Vec<Vec<usize>>,
    stars: &mut Vec<usize>,
    parent_v: usize,
    v: usize,
    depth: usize,
) {
    if depth == 1 {
        stars.push(graph[v].len());
    }
    for &next_v in &graph[v] {
        if next_v == parent_v {
            continue;
        }
        dfs(graph, stars, v, next_v, (depth + 1) % 3);
    }
}
