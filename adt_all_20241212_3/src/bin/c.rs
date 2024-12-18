use itertools::Itertools;
use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut graph = vec![BTreeSet::new(); n];
    for &(a, b) in &ab {
        graph[a].insert(b);
        graph[b].insert(a);
    }

    println!(
        "{}",
        graph
            .iter()
            .map(|adj| {
                let mut a = vec![adj.len()];
                a.extend(adj.iter().map(|&b| b + 1));
                a.iter().join(" ")
            })
            .join("\n")
    );
}
