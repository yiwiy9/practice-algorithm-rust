use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    }

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        if i < k {
            heap.push(Reverse(p[i]));
            if i == k - 1 {
                println!("{}", heap.peek().unwrap().0);
            }
            continue;
        }

        let kth_val = heap.pop().unwrap().0;
        if p[i] > kth_val {
            heap.push(Reverse(p[i]));
        } else {
            heap.push(Reverse(kth_val));
        }
        println!("{}", heap.peek().unwrap().0);
    }
}
