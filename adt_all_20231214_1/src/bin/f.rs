use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1,Usize1,usize); m],
    }

    let mut graph = vec![vec![]; n];
    for (a, b, c) in abc {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let mut ans = 0;
    for i in 0..n {
        let mut seen = vec![false; n];
        ans = ans.max(dfs(&graph, &mut seen, (i, 0)));
    }

    println!("{}", ans);
}

pub fn dfs(graph: &Vec<Vec<(usize, usize)>>, seen: &mut Vec<bool>, v: (usize, usize)) -> usize {
    seen[v.0] = true;

    let mut max_dist = 0;
    for &next_v in &graph[v.0] {
        if seen[next_v.0] {
            continue;
        }
        max_dist = max_dist.max(dfs(graph, seen, next_v));
    }
    seen[v.0] = false;

    max_dist + v.1
}
