use proconio::{input, marker::Usize1};
use std::collections::{HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
        q: usize,
        xk: [(Usize1,usize); q],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    for &(x, k) in &xk {
        println!("{}", bfs(&graph, x, k));
    }
}

pub fn bfs(graph: &Vec<Vec<usize>>, x: usize, k: usize) -> usize {
    let mut seen = HashSet::new();
    let mut que = VecDeque::new();
    let mut ans = 0;
    seen.insert(x);
    que.push_back((x, k));
    while let Some((u, k)) = que.pop_front() {
        ans += u + 1;

        for &v in &graph[u] {
            if seen.contains(&v) || k == 0 {
                continue;
            }
            seen.insert(v);
            que.push_back((v, k - 1));
        }
    }
    ans
}
