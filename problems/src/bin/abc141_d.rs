use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let mut heap = BinaryHeap::new();
    for a_i in a {
        heap.push(a_i);
    }

    for _ in 0..m {
        let mut x = heap.pop().unwrap();
        x /= 2;
        heap.push(x);
    }

    println!("{}", heap.iter().sum::<usize>());
}
