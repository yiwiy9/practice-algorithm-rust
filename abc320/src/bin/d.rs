use proconio::{input, marker::Usize1};
const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(Usize1,Usize1,i64,i64); m],
    }

    let mut graph = vec![vec![]; n];
    for (a, b, x, y) in edges {
        graph[a].push((b, (x, y)));
        graph[b].push((a, (-x, -y)));
    }
    let pos = bfs(&graph);

    for (x, y) in pos {
        if x == INF || y == INF {
            println!("undecidable")
        } else {
            println!("{} {}", x, y)
        }
    }
}

pub fn bfs(graph: &Vec<Vec<(usize, (i64, i64))>>) -> Vec<(i64, i64)> {
    let n = graph.len();
    let mut pos = vec![(INF, INF); n];
    let mut que = std::collections::VecDeque::new();
    pos[0] = (0, 0);
    que.push_back(0);
    while let Some(u) = que.pop_front() {
        for &(v, (x, y)) in &graph[u] {
            if pos[v] != (INF, INF) {
                continue;
            }
            pos[v] = (pos[u].0 + x, pos[u].1 + y);
            que.push_back(v);
        }
    }
    pos
}
