use proconio::input;
use std::collections::BTreeMap;
use superslice::*;

fn main() {
    input! {
        w: usize,
        h: usize,
        n: usize,
        pq: [(usize,usize); n],
        a_n: usize,
        mut a: [usize; a_n],
        b_n: usize,
        mut b: [usize; b_n],
    }

    a.push(w);
    b.push(h);

    let mut cake_count_map = BTreeMap::new();
    for (p, q) in &pq {
        let i = a.lower_bound(p);
        let j = b.lower_bound(q);

        cake_count_map
            .entry((i, j))
            // .entry((a_n + 1) * j + i) 一次元にすると間違いを生みやすいYO
            .and_modify(|cur| *cur += 1)
            .or_insert(1);
    }

    let mut min = 1 << 30;
    let mut max = 0;

    if cake_count_map.len() < (a_n + 1) * (b_n + 1) {
        min = 0;
    }

    for &cnt in cake_count_map.values() {
        min = min.min(cnt);
        max = max.max(cnt);
    }

    println!("{} {}", min, max);
}
