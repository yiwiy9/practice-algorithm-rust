use proconio::{input, marker::Usize1};
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [Usize1; m],
        a: [usize; m],
    }

    if a.iter().sum::<usize>() != n {
        println!("-1");
        return;
    }

    let mut heap = BinaryHeap::new();
    for i in 0..m {
        heap.push(Reverse((x[i], a[i])));
    }
    heap.push(Reverse((n, 0)));

    let mut ans = 0;
    let mut pos = 0;
    let mut left = 0;
    while let Some(Reverse((x_i, a_i))) = heap.pop() {
        if pos + left < x_i {
            println!("-1");
            return;
        }

        if pos + 1 < x_i {
            ans += (x_i - pos) * (x_i - pos - 1) / 2;
        }
        left -= x_i - pos;
        ans += left * (x_i - pos);
        pos = x_i;
        left += a_i;
    }

    println!("{}", ans);
}
