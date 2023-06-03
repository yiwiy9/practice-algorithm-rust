use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1,Usize1); m],
        k_n: usize,
        xy: [(Usize1,Usize1); k_n],
        q_n: usize,
        pq: [(Usize1,Usize1); q_n],
    }

    let mut uf: UnionFind<usize> = UnionFind::new(n);

    for &(u, v) in &uv {
        uf.union(u, v);
    }

    let mut root_set = HashSet::new();
    for &(x, y) in &xy {
        let x_root = uf.find(x);
        let y_root = uf.find(y);
        root_set.insert((x_root, y_root));
        root_set.insert((y_root, x_root));
    }

    for &(p, q) in &pq {
        let p_root = uf.find(p);
        let q_root = uf.find(q);
        println!(
            "{}",
            if root_set.contains(&(p_root, q_root)) {
                "No"
            } else {
                "Yes"
            }
        );
    }
}
