use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1,Usize1,usize); m],
    }

    let mut graph = vec![vec![]; n];

    for &(a, b, c) in &abc {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let mut ans = 0;
    for v in 0..n {
        ans = ans.max(dfs(&graph, &mut vec![false; n], v));
    }

    println!("{}", ans);
}

pub fn dfs(graph: &Vec<Vec<(usize, usize)>>, seen: &mut Vec<bool>, v: usize) -> usize {
    seen[v] = true;

    let mut max_len = 0;
    for &(next_v, c) in &graph[v] {
        if seen[next_v] {
            continue;
        }
        max_len = max_len.max(dfs(graph, seen, next_v) + c);
    }

    seen[v] = false;
    max_len
}
