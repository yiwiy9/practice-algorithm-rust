use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        uv: [(Usize1, Usize1); n-1],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut start_leaf = 0;
    for i in 0..n {
        if graph[i].len() == 1 {
            start_leaf = i;
            break;
        }
    }

    let mut levels = vec![];
    dfs(&graph, &mut levels, &mut vec![false; n], start_leaf, 2);
    levels.sort();

    println!("{}", levels.iter().join(" "));
}

pub fn dfs(
    graph: &Vec<Vec<usize>>,
    levels: &mut Vec<usize>,
    seen: &mut Vec<bool>,
    v: usize,
    center: usize,
) {
    seen[v] = true;

    if center == 0 {
        levels.push(graph[v].len());
    }

    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        dfs(graph, levels, seen, next_v, (center + 1) % 3);
    }
}
