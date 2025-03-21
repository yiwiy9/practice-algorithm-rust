use itertools::Itertools;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        tws: [(usize,usize,usize); m],
    }

    let mut in_line = BinaryHeap::from((0..n).map(Reverse).collect::<Vec<_>>());
    let mut out_of_line = BinaryHeap::new();
    let mut ans = vec![0; n];
    for &(t_i, w_i, s_i) in &tws {
        while let Some(Reverse((t, i))) = out_of_line.pop() {
            if t <= t_i {
                in_line.push(Reverse(i));
            } else {
                out_of_line.push(Reverse((t, i)));
                break;
            }
        }

        if let Some(Reverse(i)) = in_line.pop() {
            ans[i] += w_i;
            out_of_line.push(Reverse((t_i + s_i, i)));
        }
    }

    println!("{}", ans.iter().join("\n"));
}
