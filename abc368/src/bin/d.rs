use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

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

    let mut in_v = vec![false; n];
    for &v_i in &v {
        in_v[v_i] = true;
    }

    let mut seen = vec![false; n];
    let mut need = BTreeSet::new();
    dfs(&graph, &in_v, &mut seen, &mut need, v[0]);

    println!("{}", need.len());
}

pub fn dfs(
    graph: &Vec<Vec<usize>>,
    in_v: &Vec<bool>,
    seen: &mut Vec<bool>,
    need: &mut BTreeSet<usize>,
    v: usize,
) -> bool {
    seen[v] = true;

    let mut ok = false;
    if in_v[v] {
        ok = true;
        need.insert(v);
    }
    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        ok |= dfs(graph, in_v, seen, need, next_v);
    }

    if ok {
        need.insert(v);
    }
    ok
}
