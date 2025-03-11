use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        b: [i64; n],
        w: [i64; m],
    }

    let mut b_heap = BinaryHeap::new();
    for &b_i in &b {
        b_heap.push(b_i);
    }
    let mut w_heap = BinaryHeap::new();
    for &w_i in &w {
        w_heap.push(w_i);
    }

    let mut ans = 0;
    while let Some(b_i) = b_heap.pop() {
        if let Some(&w_i) = w_heap.peek() {
            if w_i > 0 && w_i + b_i >= 0 {
                let w_i = w_heap.pop().unwrap();
                ans += b_i + w_i;
                continue;
            }
        }

        if b_i > 0 {
            ans += b_i;
        } else {
            break;
        }
    }

    println!("{}", ans);
}
