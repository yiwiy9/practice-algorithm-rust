use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
    }

    let mut ans = -1;
    for s in 0..n {
        let seen = bfs(&graph, s);
        if seen.iter().all(|&seen_v| seen_v) {
            ans = s as i32 + 1;
        }
    }

    println!("{}", ans);
}

pub fn bfs(graph: &Vec<Vec<usize>>, s: usize) -> Vec<bool> {
    let n = graph.len();
    let mut seen = vec![false; n];
    let mut que = std::collections::VecDeque::new();
    seen[s] = true;
    que.push_back(s);
    while let Some(u) = que.pop_front() {
        for &v in &graph[u] {
            if seen[v] {
                continue;
            }
            seen[v] = true;
            que.push_back(v);
        }
    }
    seen
}
