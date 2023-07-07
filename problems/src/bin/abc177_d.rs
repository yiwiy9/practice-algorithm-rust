use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1,Usize1); m],
    }

    let mut uf: UnionFind<usize> = UnionFind::new(n);

    for &(a, b) in &ab {
        uf.union(a, b);
    }

    let mut map = BTreeMap::new();
    for i in 0..n {
        map.entry(uf.find(i))
            .and_modify(|cur| *cur += 1)
            .or_insert(1);
    }

    println!("{}", map.values().max().unwrap_or(&0));
}
