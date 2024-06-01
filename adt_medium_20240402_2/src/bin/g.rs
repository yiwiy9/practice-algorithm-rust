use proconio::{input, marker::Usize1};

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        m: usize,
        abxy: [(Usize1, Usize1, i64, i64); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b, x, y) in &abxy {
        graph[a].push((b, x, y));
        graph[b].push((a, -x, -y));
    }

    let pos = bfs(&graph);

    for &(x, y) in &pos {
        if (x, y) == (INF, INF) {
            println!("undecidable");
            continue;
        }
        println!("{} {}", x, y);
    }
}

pub fn bfs(graph: &Vec<Vec<(usize, i64, i64)>>) -> Vec<(i64, i64)> {
    let n = graph.len();
    let mut pos: Vec<(i64, i64)> = vec![(INF, INF); n];
    let mut que = std::collections::VecDeque::new();
    pos[0] = (0, 0);
    que.push_back((0, 0, 0));
    while let Some((u, x, y)) = que.pop_front() {
        for &(v, dx, dy) in &graph[u] {
            if pos[v] != (INF, INF) {
                continue;
            }
            pos[v] = (x + dx, y + dy);
            que.push_back((v, x + dx, y + dy));
        }
    }
    pos
}
