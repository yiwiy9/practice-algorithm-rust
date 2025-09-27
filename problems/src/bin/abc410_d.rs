use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abw: [(Usize1, Usize1, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b, w) in &abw {
        graph[a].push((b, w));
    }

    let mut seen = vec![vec![false; 1 << 10]; n];
    dfs(&graph, &mut seen, 0, 0);

    let mut ans = -1;
    for i in 0..(1 << 10) {
        if seen[n - 1][i] {
            ans = i as i32;
            break;
        }
    }

    println!("{}", ans);
}

pub fn dfs(graph: &Vec<Vec<(usize, usize)>>, seen: &mut Vec<Vec<bool>>, v: usize, w: usize) {
    seen[v][w] = true;
    for &(next_v, next_w) in &graph[v] {
        if seen[next_v][w ^ next_w] {
            continue;
        }
        dfs(graph, seen, next_v, w ^ next_w);
    }
}
