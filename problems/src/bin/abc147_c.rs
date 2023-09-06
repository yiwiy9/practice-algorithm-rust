use proconio::{input, marker::Usize1};
use std::collections::VecDeque;

/**
 * https://atcoder.jp/contests/abc147/tasks/abc147_c
 *
 * bit全探索の時点で全探索なんやから、BFSの必要なし
 * https://blog.hamayanhamayan.com/entry/2019/12/13/214342
 */
fn main() {
    input! {
        n: usize,
    }

    let mut testimony_vec = vec![];
    for _ in 0..n {
        input! {
            a: usize,
            xy: [(Usize1, i32); a],
        }
        testimony_vec.push(xy);
    }

    let mut ans = 0;
    for bit in 0..(1 << n) {
        let mut is_honest = vec![-1; n];
        let mut que = VecDeque::new();
        for i in 0..n {
            if bit & (1 << i) != 0 {
                is_honest[i] = 1;
                que.push_back(i);
            }
        }

        let mut ok = true;
        'bfs: while let Some(j) = que.pop_front() {
            for &(x, y) in &testimony_vec[j] {
                if is_honest[x] != -1 {
                    if is_honest[x] != y {
                        ok = false;
                        break 'bfs;
                    }
                    continue;
                }
                is_honest[x] = y;
                if y == 1 {
                    que.push_back(x);
                }
            }
        }

        if ok {
            ans = ans.max(
                is_honest
                    .iter()
                    .filter(|&y_i| *y_i == 1)
                    .collect::<Vec<_>>()
                    .len(),
            );
        }
    }

    println!("{}", ans);
}

pub fn bfs(graph: &Vec<Vec<usize>>, s: usize) -> Vec<usize> {
    let inf = (1 << 30) as usize;
    let n = graph.len();
    let mut dist = vec![inf; n];
    let mut que = std::collections::VecDeque::new();
    dist[s] = 0;
    que.push_back(s);
    while let Some(u) = que.pop_front() {
        for &v in &graph[u] {
            if dist[v] != inf {
                continue;
            }
            dist[v] = dist[u] + 1;
            que.push_back(v);
        }
    }
    dist
}
