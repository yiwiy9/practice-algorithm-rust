use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[b].push(a);
    }

    let mut saikyo = BTreeSet::new();
    let mut seen = vec![false; n];
    for i in 0..n {
        if seen[i] {
            continue;
        }
        dfs(&graph, &mut seen, &mut saikyo, i);
    }

    println!(
        "{}",
        if saikyo.len() == 1 {
            (saikyo.iter().next().unwrap() + 1) as i64
        } else {
            -1
        }
    );
}

pub fn dfs(graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, saikyo: &mut BTreeSet<usize>, v: usize) {
    seen[v] = true;

    if graph[v].is_empty() {
        saikyo.insert(v);
    }

    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        dfs(graph, seen, saikyo, next_v);
    }
}
