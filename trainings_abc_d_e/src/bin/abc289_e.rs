use proconio::{input, marker::Usize1};
use std::collections::{HashMap, VecDeque};

/// https://atcoder.jp/contests/abc289/tasks/abc289_e
/// https://atcoder.jp/contests/abc289/editorial/5726
/// この問題を解くための最初のポイントは 「操作を繰り返したときに発生する状態数が少ない」 という点です。
///
/// 具体的には、発生する状態の個数は
/// 「高橋君が頂点 1 ~ 頂点 N のどの頂点にいるか」×「青木君が頂点 1 ~ 頂点 N のどの頂点にいるか」
/// の N×N≤4×10^6 通りで抑えられます。
///
/// => 盤面の状態をBFSすれば解ける
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            c: [usize; n],
            uv: [(Usize1, Usize1); m],
        }

        let mut graph = vec![vec![]; n];
        for &(u, v) in &uv {
            graph[u].push(v);
            graph[v].push(u);
        }

        println!("{}", bfs(&graph, &c, n));
    }

    pub fn bfs(graph: &[Vec<usize>], c: &[usize], n: usize) -> i32 {
        let mut dist = HashMap::new();
        let mut que = VecDeque::new();
        dist.insert((0, n - 1), 0);
        que.push_back((0, n - 1));
        while let Some(state) = que.pop_front() {
            let cur_dist = dist[&state];
            let (t_u, a_u) = state;

            for &t_v in &graph[t_u] {
                for &a_v in &graph[a_u] {
                    if dist.contains_key(&(t_v, a_v)) {
                        continue;
                    }
                    if c[t_v] == c[a_v] {
                        continue;
                    }
                    let new_state = (t_v, a_v);
                    dist.insert(new_state, cur_dist + 1);
                    que.push_back(new_state);
                }
            }
        }

        if dist.contains_key(&(n - 1, 0)) {
            dist[&(n - 1, 0)]
        } else {
            -1
        }
    }
}
