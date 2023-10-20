use proconio::{input, marker::Usize1};

/**
 * https://atcoder.jp/contests/abc288/tasks/abc288_c
 * https://atcoder.jp/contests/abc288/editorial/5662
 * 最大でL本の辺を残せるとすると、答えは M-L
 * 頂点数がXの連結成分について全域木を考えると、X-1本の辺を残せる
 * => グラフの連結成分の個数をSとすると、L = Σ1~S(X-1) = N - S
 * => M - L = M - N + S
 */
fn main() {
    input! {
        n: usize,
        m: usize,
        ab:[(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut s = 0;
    let mut seen = vec![false; n];
    for v in 0..n {
        if !seen[v] {
            s += 1;
            dfs(&graph, &mut seen, v)
        }
    }

    println!("{}", m + s - n);
}

pub fn dfs(graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, v: usize) {
    seen[v] = true;
    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        dfs(graph, seen, next_v);
    }
}
