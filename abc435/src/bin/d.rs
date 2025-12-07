use proconio::{input, marker::Usize1};

/// https://atcoder.jp/contests/abc435
/// https://atcoder.jp/contests/abc435/editorial/14708
/// サイクルがあるグラフでDPをするなと何度も言ってるだろ！！！
/// => 黒に到達できるか = 辺の向きを逆にして黒から到達できるか
fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(Usize1, Usize1); m],
        q: usize,
        queries: [(usize, Usize1); q],
    }

    let mut graph = vec![vec![]; n];
    for &(x, y) in &xy {
        graph[y].push(x);
    }

    let mut seen = vec![false; n];
    for &(op, v) in &queries {
        if op == 1 {
            if seen[v] {
                continue;
            }
            dfs(&graph, &mut seen, v);
        } else {
            println!("{}", if seen[v] { "Yes" } else { "No" })
        }
    }
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
