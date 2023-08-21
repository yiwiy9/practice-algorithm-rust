use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1,Usize1); m],
    }

    let mut uf: UnionFind<usize> = UnionFind::new(n);

    for &(u, v) in &uv {
        uf.union(u, v);
    }

    let mut set = BTreeSet::new();
    for i in 0..n {
        set.insert(uf.find(i));
    }

    println!("{}", set.len());
}
