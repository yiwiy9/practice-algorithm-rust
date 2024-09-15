use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        x: [i64; n],
        p: [i64; n],
        q: usize,
        lr: [(i64, i64); q],
    }

    let mut p_s = vec![0; n + 1];
    for i in 0..n {
        p_s[i + 1] = p_s[i] + p[i];
    }

    for (l, r) in lr {
        let left_index = x.lower_bound(&l);
        let right_index = x.upper_bound(&r);
        println!("{}", p_s[right_index] - p_s[left_index]);
    }
}
