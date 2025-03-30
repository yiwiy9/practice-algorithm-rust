use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut ans = 0;
    let mut seen = vec![false; n];
    let mut finished = vec![false; n];
    for v in 0..n {
        if seen[v] {
            continue;
        }
        ans += dfs(&graph, &mut seen, &mut finished, n, v);
    }

    println!("{}", ans);
}

pub fn dfs(
    graph: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    finished: &mut Vec<bool>,
    before_v: usize,
    v: usize,
) -> usize {
    seen[v] = true;

    let mut res = 0;
    for &next_v in &graph[v] {
        if seen[next_v] {
            if next_v != before_v && !finished[next_v] {
                res += 1;
            }
            continue;
        }
        res += dfs(graph, seen, finished, v, next_v);
    }
    finished[v] = true;

    res
}
