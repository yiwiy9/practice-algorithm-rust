use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        q: usize,
        queries: [(usize,usize); q],
    }

    let mut min_heap = BinaryHeap::new();
    for &(op, h) in &queries {
        if op == 1 {
            min_heap.push(Reverse(h));
        } else {
            while let Some(Reverse(cur)) = min_heap.pop() {
                if cur > h {
                    min_heap.push(Reverse(cur));
                    break;
                }
            }
        }

        println!("{}", min_heap.len());
    }
}
