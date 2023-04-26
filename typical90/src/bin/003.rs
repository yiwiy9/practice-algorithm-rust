use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        ab:[(Usize1, Usize1); n-1],
    }

    let mut graph = vec![vec![]; n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let dist_from_0 = bfs(&graph, 0);
    let max_dist_v = dist_from_0
        .iter()
        .enumerate()
        .max_by_key(|(_, &value)| value)
        .unwrap()
        .0;

    let dist = bfs(&graph, max_dist_v);
    println!("{}", dist.iter().max().unwrap() + 1);
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
