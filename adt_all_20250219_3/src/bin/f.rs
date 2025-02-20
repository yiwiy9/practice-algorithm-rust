use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n],
    }

    let mut graph = vec![vec![]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let (x_i, y_i) = xy[i];
            let (x_j, y_j) = xy[j];
            let dx = x_i - x_j;
            let dy = y_i - y_j;
            if dx * dx + dy * dy <= d * d {
                graph[i].push(j);
            }
        }
    }

    let mut seen = vec![false; n];
    dfs(&graph, &mut seen, 0);

    for i in 0..n {
        if seen[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
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
