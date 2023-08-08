use petgraph::unionfind::UnionFind;
use proconio::input;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut uf = UnionFind::<usize>::new(200001);
    let mut unique_a = BTreeSet::new();
    for i in 0..(n + 1) / 2 {
        uf.union(a[i], a[n - 1 - i]);
        unique_a.insert(a[i]);
        unique_a.insert(a[n - 1 - i]);
    }

    let mut map = BTreeMap::new();
    for &a_i in &unique_a {
        map.entry(uf.find(a_i))
            .and_modify(|cur| *cur += 1)
            .or_insert(1);
    }

    println!("{}", map.iter().map(|(_, &cnt)| cnt - 1).sum::<usize>());
}
