use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1,Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
    }

    let mut seen_set = vec![HashSet::new(); n];
    let mut ans = 0;
    for i in 0..n {
        dfs(&graph, &mut seen_set, i, i);

        ans += seen_set[i].len() - (graph[i].len() + 1);
    }

    println!("{}", ans);
}

pub fn dfs(graph: &Vec<Vec<usize>>, seen_set: &mut Vec<HashSet<usize>>, root: usize, v: usize) {
    seen_set[root].insert(v);
    for &next_v in &graph[v] {
        if seen_set[root].contains(&next_v) {
            continue;
        }
        if !seen_set[next_v].is_empty() {
            let cloned = seen_set[next_v].clone();
            seen_set[root].extend(&cloned);
            continue;
        }
        dfs(graph, seen_set, root, next_v);
    }
}
