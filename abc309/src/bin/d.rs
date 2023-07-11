use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n1: usize,
        n2: usize,
        m: usize,
        ab:[(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n1 + n2];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let dist1 = bfs(&graph, 0);
    let dist2 = bfs(&graph, n1 + n2 - 1);

    println!(
        "{}",
        dist1
            .iter()
            .filter(|&d| *d != (1 << 30) as usize)
            .max()
            .unwrap()
            + dist2
                .iter()
                .filter(|&d| *d != (1 << 30) as usize)
                .max()
                .unwrap()
            + 1
    );
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
