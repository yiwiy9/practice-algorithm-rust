use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize,usize); n],
    }

    let mut cur = 0;
    let mut min_heap = BinaryHeap::new();
    for (a, b) in ab {
        cur += b;
        min_heap.push(Reverse((a, b)));
    }

    if cur <= k {
        println!("{}", 1);
        return;
    }

    while let Some(Reverse((a, b))) = min_heap.pop() {
        cur -= b;
        if cur <= k {
            println!("{}", a + 1);
            return;
        }
    }
}
