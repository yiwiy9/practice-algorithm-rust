use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n-1],
    }

    let mut a_heap = BinaryHeap::new();
    for &a_i in a.iter() {
        a_heap.push(a_i);
    }

    let mut b_heap = BinaryHeap::new();
    for &b_i in b.iter() {
        b_heap.push(b_i);
    }

    let mut ans = 0;
    let mut ok = true;
    while let Some(a_i) = a_heap.pop() {
        if let Some(b_i) = b_heap.pop() {
            if a_i <= b_i {
                continue;
            } else if ans == 0 {
                ans = a_i;
                b_heap.push(b_i);
            } else {
                ok = false;
                break;
            }
        } else {
            ans = a_i;
        }
    }

    println!("{}", if ok { ans as i32 } else { -1 });
}
