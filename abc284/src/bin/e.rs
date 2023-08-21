use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for (u, v) in uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    println!("{}", 1_000_000.min(dfs(&graph, &mut vec![false; n], 0)));
}

pub fn dfs(graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, v: usize) -> usize {
    seen[v] = true;
    let mut cnt = 1;
    for &next_v in &graph[v] {
        if seen[next_v] || cnt >= 1_000_000 {
            continue;
        }
        cnt += dfs(graph, seen, next_v);
    }
    seen[v] = false;
    cnt
}
