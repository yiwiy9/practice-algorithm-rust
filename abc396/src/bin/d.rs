use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uvw: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, w) in &uvw {
        graph[u].push((v, w));
        graph[v].push((u, w));
    }

    let xor_vec = dfs(&graph, &mut vec![false; n], 0);

    println!("{}", xor_vec.iter().min().unwrap());
}

pub fn dfs(graph: &Vec<Vec<(usize, usize)>>, seen: &mut Vec<bool>, v: usize) -> Vec<usize> {
    if v == graph.len() - 1 {
        return vec![0];
    }

    seen[v] = true;

    let mut res = vec![];
    for &(next_v, w) in &graph[v] {
        if seen[next_v] {
            continue;
        }
        let xor_vec = dfs(graph, seen, next_v);
        for &x in &xor_vec {
            res.push(x ^ w);
        }
    }

    seen[v] = false;
    res.iter().unique().cloned().collect()
}
