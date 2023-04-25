use proconio::{input, marker::Usize1};
use std::cmp::Ordering;
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
    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut dist_vec = Vec::new();
    for v in 0..n {
        dist_vec.push(bfs(&graph, v));
    }

    let mut black_candidates = vec![];
    let mut white_set = BTreeSet::new();
    for (p, d) in &pd {
        let mut black_vec = vec![];
        for (v, dist) in dist_vec[*p].iter().enumerate() {
            match dist.cmp(d) {
                Ordering::Greater => continue,
                Ordering::Less => {
                    white_set.insert(v);
                    continue;
                }
                Ordering::Equal => black_vec.push(v),
            };
        }
        black_candidates.push(black_vec);
    }

    if black_candidates
        .iter()
        .all(|black_vec| black_vec.iter().any(|black| !white_set.contains(black)))
    {
        let ans = (0..n)
            .map(|v| if white_set.contains(&v) { "0" } else { "1" })
            .collect::<String>();

        println!("Yes");
        println!("{}", ans);
    } else {
        println!("No");
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
