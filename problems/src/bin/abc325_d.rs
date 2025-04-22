use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize,
        td: [(usize, usize); n],
    }

    let mut products_heap = BinaryHeap::new();
    for &(t, d) in &td {
        products_heap.push(Reverse((t, d)));
    }

    let mut ans = 0;
    let mut time = 0;
    let mut machine_heap = BinaryHeap::new();
    loop {
        if machine_heap.is_empty() {
            if products_heap.is_empty() {
                break;
            }

            let Reverse((t, d)) = products_heap.pop().unwrap();
            time = t;
            machine_heap.push(Reverse(t + d));
        }

        while let Some(Reverse((t, d))) = products_heap.pop() {
            if t > time {
                products_heap.push(Reverse((t, d)));
                break;
            }
            machine_heap.push(Reverse(t + d));
        }

        while let Some(Reverse(t_d)) = machine_heap.pop() {
            if t_d >= time {
                ans += 1;
                time += 1;
                break;
            }
        }
    }

    println!("{}", ans);
}
