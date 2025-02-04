use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    input! {
        n: usize,
        c: [usize; n],
        ab: [(Usize1,Usize1); n-1],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut good_v = BTreeSet::new();
    dfs(
        &graph,
        &c,
        &mut vec![false; n],
        &mut BTreeMap::new(),
        &mut good_v,
        0,
    );
    println!("{}", good_v.iter().join("\n"));
}

pub fn dfs(
    graph: &Vec<Vec<usize>>,
    c: &Vec<usize>,
    seen: &mut Vec<bool>,
    on_path: &mut BTreeMap<usize, usize>,
    good_v: &mut BTreeSet<usize>,
    v: usize,
) {
    seen[v] = true;

    if !on_path.contains_key(&c[v]) {
        good_v.insert(v + 1);
    }

    on_path.entry(c[v]).and_modify(|e| *e += 1).or_insert(1);

    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        dfs(graph, c, seen, on_path, good_v, next_v);
    }

    on_path.entry(c[v]).and_modify(|e| *e -= 1);
    if on_path[&c[v]] == 0 {
        on_path.remove(&c[v]);
    }
}
