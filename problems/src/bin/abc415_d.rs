use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        mut n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut min_heap = BinaryHeap::new();
    for &(a, b) in &ab {
        min_heap.push(Reverse((a - b, a, b)));
    }

    let mut ans = 0;
    while let Some(Reverse((a_b, a, _))) = min_heap.pop() {
        if n < a {
            continue;
        }

        ans += (n - a) / a_b;
        n -= ((n - a) / a_b) * a_b;
        ans += 1;
        n -= a_b;
    }

    println!("{}", ans);
}
