use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        sc: [(usize,usize); n],
    }

    let mut min_heap = BinaryHeap::new();
    for (s_i, c_i) in sc {
        min_heap.push(Reverse((s_i, c_i)));
    }

    let mut cur_s = 0;
    let mut cur_c = 0;
    let mut ans = 0_usize;
    while let Some(Reverse((s, c))) = min_heap.pop() {
        if cur_s == s {
            cur_c += c;
        } else {
            cur_s = s;
            cur_c = c;
        }

        if !min_heap.is_empty() && min_heap.peek().unwrap().0 .0 == cur_s {
            continue;
        }

        if cur_c % 2 == 1 {
            cur_c -= 1;
            ans += 1;
        }

        if cur_c > 0 {
            cur_s *= 2;
            cur_c /= 2;
            min_heap.push(Reverse((cur_s, cur_c)));
        }

        cur_s = 0;
        cur_c = 0;
    }

    println!("{}", ans);
}
