use proconio::input;
use std::cmp;
use superslice::*;

fn main() {
    input! {
        n: usize,
        mut a:[i32; n],
        q: usize,
        b:[i32; q],
    }

    a.sort();

    for b_j in &b {
        let lower_i = a.lower_bound(b_j);
        let ans = match lower_i {
            0 => (a[lower_i] - b_j).abs(),
            x if x == n => (a[lower_i - 1] - b_j).abs(),
            _ => cmp::min((a[lower_i] - b_j).abs(), (a[lower_i - 1] - b_j).abs()),
        };
        println!("{}", ans);
    }
}
