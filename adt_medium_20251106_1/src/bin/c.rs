use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut heap = BinaryHeap::from_iter(a);

    let mut ans = 0_usize;
    while let Some(num1) = heap.pop() {
        if heap.is_empty() {
            break;
        }

        let num2 = heap.pop().unwrap();
        ans += 1;

        if num1 > 1 {
            heap.push(num1 - 1);
        }
        if num2 > 1 {
            heap.push(num2 - 1);
        }
    }

    println!("{}", ans);
}
