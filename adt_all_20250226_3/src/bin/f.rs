use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b, c) in &abc {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let mut ans = 0;
    for v in 0..n {
        ans = ans.max(dfs(&graph, &mut vec![false; n], v));
    }

    println!("{}", ans);
}

pub fn dfs(graph: &Vec<Vec<(usize, usize)>>, used: &mut Vec<bool>, v: usize) -> usize {
    used[v] = true;

    let mut res = 0;
    for &(next_v, cost) in &graph[v] {
        if used[next_v] {
            continue;
        }
        res = res.max(dfs(graph, used, next_v) + cost);
    }
    used[v] = false;

    res
}
