use itertools::Itertools as _;
use proconio::{input, marker::Usize1};
use std::collections::{BTreeSet, BinaryHeap};

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(Usize1,Usize1); m],
        ph: [(Usize1,usize); k],
    }

    let mut graph = vec![vec![]; n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut ph_heap = BinaryHeap::new();
    for &(p, h) in &ph {
        ph_heap.push((h, p));
    }

    let mut guarded = BTreeSet::new();
    while let Some((h, p)) = ph_heap.pop() {
        if guarded.contains(&p) {
            continue;
        }
        guarded.insert(p);

        if h == 0 {
            continue;
        }

        for &v in &graph[p] {
            ph_heap.push((h - 1, v));
        }
    }

    println!("{}", guarded.len());
    println!("{}", guarded.iter().map(|&p| p + 1).join(" "));
}
