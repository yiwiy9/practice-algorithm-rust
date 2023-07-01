use proconio::{input, marker::Usize1};
const MOD: usize = 1_000_000_007;

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

    println!("{}", bfs(&graph, 0)[n - 1]);
}

pub fn bfs(graph: &Vec<Vec<usize>>, s: usize) -> Vec<usize> {
    let inf = (1 << 30) as usize;
    let n = graph.len();
    let mut dist = vec![inf; n];
    let mut cnt = vec![0; n];
    let mut que = std::collections::VecDeque::new();
    dist[s] = 0;
    cnt[s] = 1;
    que.push_back(s);
    while let Some(u) = que.pop_front() {
        for &v in &graph[u] {
            if dist[v] != inf {
                if dist[v] == dist[u] + 1 {
                    cnt[v] += cnt[u];
                    cnt[v] %= MOD;
                }
                continue;
            }
            dist[v] = dist[u] + 1;
            cnt[v] = cnt[u];
            cnt[v] %= MOD;
            que.push_back(v);
        }
    }
    cnt
}
