use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1,Usize1); m],
        k: usize,
        pd: [(Usize1,usize); k],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut color = vec![-1; n];
    let mut black_by_p = vec![BTreeSet::new(); n];
    for &(p, d) in &pd {
        bfs(&graph, &mut color, &mut black_by_p, p, d);
    }

    let mut all_black_by_p = BTreeSet::new();
    for b_set in black_by_p {
        for &b in &b_set {
            all_black_by_p.insert(b);
        }
    }

    if pd.iter().all(|&(p, _)| all_black_by_p.contains(&p)) {
        println!("Yes");
        println!(
            "{}",
            color.iter().map(|&c| if c == -1 { 1 } else { c }).join("")
        );
    } else {
        println!("No");
    }
}

pub fn bfs(
    graph: &Vec<Vec<usize>>,
    color: &mut Vec<i64>,
    black_by_p: &mut Vec<BTreeSet<usize>>,
    p: usize,
    d: usize,
) {
    let inf = (1 << 30) as usize;
    let n = graph.len();
    let mut dist = vec![inf; n];
    let mut que = std::collections::VecDeque::new();
    dist[p] = 0;
    que.push_back(p);
    while let Some(u) = que.pop_front() {
        if dist[u] == d {
            if color[u] == 0 {
                continue;
            }
            color[u] = 1;
            black_by_p[u].insert(p);
            continue;
        }

        if color[u] == 1 {
            black_by_p[u].clear();
        }
        color[u] = 0;

        for &v in &graph[u] {
            if dist[v] != inf {
                continue;
            }
            dist[v] = dist[u] + 1;
            que.push_back(v);
        }
    }
}
