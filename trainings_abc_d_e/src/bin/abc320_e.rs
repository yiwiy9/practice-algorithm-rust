use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        tws: [(usize, usize, usize); m],
    }

    let mut ans = vec![0; n];

    let mut in_line = BinaryHeap::from_iter((0..n).map(Reverse));
    let mut out_of_line = BinaryHeap::new();

    for &(t, w, s) in &tws {
        while let Some(Reverse((time, i))) = out_of_line.pop() {
            if time <= t {
                in_line.push(Reverse(i));
            } else {
                out_of_line.push(Reverse((time, i)));
                break;
            }
        }

        if let Some(Reverse(i)) = in_line.pop() {
            ans[i] += w;
            out_of_line.push(Reverse((t + s, i)));
        }
    }

    println!("{}", ans.iter().join("\n"));
}
