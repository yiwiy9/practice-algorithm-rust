use proconio::{input, marker::Usize1};
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [Usize1; m],
        a: [usize; m],
    }

    let mut min_heap = BinaryHeap::new();
    for i in 0..m {
        min_heap.push(Reverse((x[i], a[i])));
    }
    min_heap.push(Reverse((n, 0)));

    let (mut cur_pos, mut cur_cnt) = min_heap.pop().unwrap().0;
    if cur_pos != 0 {
        println!("-1");
        return;
    }

    let mut ans = 0;
    while let Some(Reverse((next_pos, next_cnt))) = min_heap.pop() {
        if next_pos > cur_pos + cur_cnt {
            println!("-1");
            return;
        }

        let dist = next_pos - cur_pos;
        ans += (dist - 1) * dist / 2;
        cur_cnt -= dist;
        ans += dist * cur_cnt;
        cur_pos = next_pos;
        cur_cnt += next_cnt;
    }

    if cur_cnt != 0 {
        println!("-1");
        return;
    }

    println!("{}", ans);
}
