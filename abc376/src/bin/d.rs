use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
    }

    println!("{}", bfs(&graph));
}

pub fn bfs(graph: &Vec<Vec<usize>>) -> i32 {
    let inf = (1 << 30) as usize;
    let n = graph.len();
    let mut dist = vec![inf; n];
    let mut que = std::collections::VecDeque::new();
    dist[0] = 0;
    que.push_back(0);
    while let Some(u) = que.pop_front() {
        for &v in &graph[u] {
            if v == 0 {
                return (dist[u] + 1) as i32;
            }
            if dist[v] != inf {
                continue;
            }
            dist[v] = dist[u] + 1;
            que.push_back(v);
        }
    }
    -1
}
