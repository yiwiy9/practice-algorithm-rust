use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
    }

    let mut graph = vec![];
    for _ in 0..n {
        input! {
            c: usize,
            p: [Usize1; c],
        }
        graph.push(p);
    }

    let mut order = vec![];
    dfs(&graph, &mut vec![false; n], &mut order, 0);

    println!(
        "{}",
        order.iter().take(order.len() - 1).map(|v| v + 1).join(" ")
    );
}

pub fn dfs(graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, order: &mut Vec<usize>, v: usize) {
    seen[v] = true;
    for &next_v in &graph[v] {
        if seen[next_v] {
            continue;
        }
        dfs(graph, seen, order, next_v);
    }
    order.push(v);
}
