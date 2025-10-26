use itertools::Itertools;
use proconio::{
    input,
    marker::{Chars, Usize1},
};

/// https://atcoder.jp/contests/abc429/tasks/abc429_e
/// https://atcoder.jp/contests/abc429/editorial/14284
/// 多始点BFSということを思いつかなければいけない
/// => 危険な頂点から安全な頂点だと候補が複数あって難しい
/// => 安全な頂点から危険な頂点への最短距離と２番目に短い距離を記録すれば良いという発想になる
fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
        s: Chars
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut reached = vec![((n, 0), (n, 0)); n];
    let mut que = std::collections::VecDeque::new();

    for (v, &c) in s.iter().enumerate() {
        if c == 'S' {
            reached[v].0 = (v, 0);
            que.push_back((v, 0, v));
        }
    }

    while let Some((u, dist, start)) = que.pop_front() {
        for &v in &graph[u] {
            if reached[v].1 .0 != n || reached[v].0 .0 == start || reached[v].1 .0 == start {
                continue;
            } else if reached[v].0 .0 != n {
                reached[v].1 = (start, dist + 1);
            } else {
                reached[v].0 = (start, dist + 1);
            }

            que.push_back((v, dist + 1, start));
        }
    }

    println!(
        "{}",
        s.iter()
            .enumerate()
            .filter(|(_, &c)| c == 'D')
            .map(|(v, _)| reached[v].0 .1 + reached[v].1 .1)
            .join("\n")
    );
}
