use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        x: Usize1,
        y: Usize1,
    }

    let mut graph = vec![vec![]; n];
    graph[x].push(y);
    graph[y].push(x);
    for i in 0..n - 1 {
        graph[i].push(i + 1);
        graph[i + 1].push(i);
    }

    let mut ans = vec![0; n];
    for i in 0..n {
        let dist = bfs(&graph, i);
        for (j, &d) in dist.iter().enumerate() {
            if j != i {
                ans[d] += 1;
            }
        }
    }

    println!("{}", ans.iter().skip(1).map(|cnt| cnt / 2).join("\n"));
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
