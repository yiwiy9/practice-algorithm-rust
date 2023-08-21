use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(Usize1, Usize1); n-1],
        px: [(Usize1, usize); q],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut sub_tree_count = vec![0; n];
    for &(p, x) in &px {
        sub_tree_count[p] += x;
    }

    let counts = bfs(&graph, &sub_tree_count, 0);

    println!("{}", counts.iter().join(" "));
}

pub fn bfs(graph: &Vec<Vec<usize>>, sub_tree_count: &[usize], s: usize) -> Vec<usize> {
    let inf = (1 << 30) as usize;
    let n = graph.len();
    let mut counts = vec![inf; n];
    let mut que = std::collections::VecDeque::new();
    counts[s] = sub_tree_count[s];
    que.push_back(s);
    while let Some(u) = que.pop_front() {
        for &v in &graph[u] {
            if counts[v] != inf {
                continue;
            }
            counts[v] = counts[u] + sub_tree_count[v];
            que.push_back(v);
        }
    }
    counts
}
