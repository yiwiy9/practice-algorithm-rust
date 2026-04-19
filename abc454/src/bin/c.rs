use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
    }

    let mut seen = vec![false; n];
    dfs(&graph, &mut seen, 0);

    println!("{}", seen.iter().filter(|&ok| *ok).count());
}

pub fn dfs(graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, v: usize) {
    seen[v] = true;
    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        dfs(graph, seen, next_v);
    }
}
