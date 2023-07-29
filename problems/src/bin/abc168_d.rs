use itertools::Itertools;
use proconio::{input, marker::Usize1};

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

    let guides = bfs(&graph, 0);

    if guides.iter().any(|&v| v == guides.len()) {
        println!("No");
        return;
    }

    println!("Yes");
    println!("{}", guides.iter().skip(1).map(|&v| v + 1).join("\n"));
}

pub fn bfs(graph: &Vec<Vec<usize>>, s: usize) -> Vec<usize> {
    let n = graph.len();
    let mut guides = vec![n; n];
    let mut que = std::collections::VecDeque::new();
    guides[s] = 0;
    que.push_back(s);
    while let Some(u) = que.pop_front() {
        for &v in &graph[u] {
            if guides[v] != n {
                continue;
            }
            guides[v] = u;
            que.push_back(v);
        }
    }
    guides
}
