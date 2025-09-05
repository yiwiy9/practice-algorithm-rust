use ac_library::Dsu;
use proconio::{input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut c_heap = BinaryHeap::new();
    for _ in 0..m {
        input! {
            ki: usize,
            ci: usize,
            ai: [Usize1; ki],
        }
        c_heap.push(Reverse((ci, ai)));
    }

    let mut dsu = Dsu::new(n);
    let mut ans = 0;

    while let Some(Reverse((ci, ai))) = c_heap.pop() {
        let u = ai[0];
        for &v in &ai[1..] {
            if dsu.same(u, v) {
                continue;
            }
            dsu.merge(u, v);
            ans += ci;
        }
    }

    println!("{}", if dsu.size(0) == n { ans as i64 } else { -1 });
}
