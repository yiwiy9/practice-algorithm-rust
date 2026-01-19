use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        s: usize,
        t: usize,
        uvc: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, c) in &uvc {
        graph[u].push((v, c));
    }

    let mut ans = vec![false; n];
    dfs(&graph, l, s, t, &mut ans, 0, 0, 0);

    println!(
        "{}",
        ans.iter()
            .enumerate()
            .filter(|&(_, &ok)| ok)
            .map(|(i, _)| i + 1)
            .join(" ")
    );
}

pub fn dfs(
    graph: &Vec<Vec<(usize, usize)>>,
    l: usize,
    s: usize,
    t: usize,
    ans: &mut Vec<bool>,
    v: usize,
    dist: usize,
    cost: usize,
) {
    if dist == l {
        if s <= cost && cost <= t {
            ans[v] = true;
        }
        return;
    }

    for &(next_v, edge_cost) in &graph[v] {
        let next_cost = cost + edge_cost;
        if next_cost <= t {
            dfs(graph, l, s, t, ans, next_v, dist + 1, next_cost);
        }
    }
}
