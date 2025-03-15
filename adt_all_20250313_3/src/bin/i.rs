use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
    }

    let mut seen = vec![false; n];
    let mut finished = vec![false; n];
    let mut has_circle = vec![false; n];
    for v in 0..n {
        if seen[v] {
            continue;
        }
        has_circle[v] = dfs(&graph, &mut seen, &mut finished, &mut has_circle, v);
    }

    println!("{}", has_circle.iter().filter(|&&b| b).count());
}

pub fn dfs(
    graph: &Vec<Vec<usize>>,
    seen: &mut Vec<bool>,
    finished: &mut Vec<bool>,
    has_circle: &mut Vec<bool>,
    v: usize,
) -> bool {
    seen[v] = true;

    let mut current = false;
    for &next_v in &graph[v] {
        if seen[next_v] {
            if has_circle[next_v] || !finished[next_v] {
                current = true;
            }
            continue;
        }
        current |= dfs(graph, seen, finished, has_circle, next_v);
    }
    finished[v] = true;

    has_circle[v] = current;
    current
}
