use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        ab: [(Usize1, Usize1); n-1],
        cd: [(Usize1, Usize1); q],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let from_0_dist = bfs(&graph, 0);

    for &(c, d) in &cd {
        println!(
            "{}",
            if from_0_dist[c] % 2 == from_0_dist[d] % 2 {
                "Town"
            } else {
                "Road"
            }
        );
    }
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
